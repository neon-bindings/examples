use std::{ops::ControlFlow, sync::mpsc, thread};

use neon::{
    prelude::*,
    types::Deferred,
    types::extract::{Error, TryIntoJs},
};

use rusqlite::Connection;

// `DbTask` is a low level task that is executed on the database event loop thread. It
// is responsible for executing user-provided functions as well as terminating the event loop.
type DbTask = Box<dyn FnOnce(&mut Connection, &Channel, Deferred) -> ControlFlow<(), ()> + Send>;

// Wraps a channel to a SQLite connection, allowing concurrent access
// Note: The connection will be dropped when all senders have been dropped.
struct Database {
    tx: mpsc::Sender<(Deferred, DbTask)>,
}

// Internal implementation
impl Database {
    fn exec<'cx, O, V, F>(&self, cx: &mut Cx<'cx>, f: F) -> JsResult<'cx, JsPromise>
    where
        F: FnOnce(&mut Connection) -> O + Send + 'static,
        for<'a> O: TryIntoJs<'a, Value = V> + Send + 'static,
        V: Value,
    {
        self.send(cx, |conn| ControlFlow::Continue(f(conn)))
    }

    fn send<'cx, O, V, F>(&self, cx: &mut Cx<'cx>, f: F) -> JsResult<'cx, JsPromise>
    where
        F: FnOnce(&mut Connection) -> ControlFlow<(), O> + Send + 'static,
        for<'a> O: TryIntoJs<'a, Value = V> + Send + 'static,
        V: Value,
    {
        let (deferred, promise) = cx.promise();

        // Create a callback that will execute on the database worker thread
        let callback: DbTask = Box::new(move |conn, channel, deferred| {
            // Execute the user's function
            let control_flow_result = f(conn);

            // Check if the function requested to break the loop before extracting the value
            let should_stop_worker = control_flow_result.is_break();

            // Extract the output value
            let output = control_flow_result.continue_value();

            // Settle the promise with the output value
            let settle_result =
                deferred.try_settle_with(channel, move |mut cx| output.try_into_js(&mut cx));

            // Stop the worker thread if:
            // 1. We can no longer settle promises (channel closed), or
            // 2. The function explicitly requested to break
            if settle_result.is_err() || should_stop_worker {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });

        // Send the callback to the worker thread
        match self.tx.send((deferred, callback)) {
            Ok(_) => Ok(promise),
            Err(mpsc::SendError((deferred, _))) => {
                // Database is closed, reject the promise
                let err = cx.error("Database is closed")?;
                deferred.reject(cx, err);
                Ok(promise)
            }
        }
    }
}

#[neon::export(class)]
// JavaScript class
impl Database {
    fn new(cx: &mut Cx) -> Result<Self, Error> {
        // Channel for sending callbacks to execute on the sqlite connection thread
        let (tx, rx) = mpsc::channel();

        // Open a connection sqlite, this will be moved to the thread
        let mut conn = Connection::open_in_memory()?;

        // Create a `Channel` for calling back to JavaScript. It is more efficient
        // to create a single channel and re-use it for all database callbacks.
        // The JavaScript process will not exit as long as this channel has not been
        // dropped.
        let channel = cx.channel();
        let db = Self { tx };

        // Create a table in the in-memory database
        // In production code, this would likely be handled somewhere else
        conn.execute(
            r#"
                CREATE TABLE person (
                    id   INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL
                )
            "#,
            [],
        )?;

        // Spawn a thread to act as the database event loop.
        // This will not block the JavaScript main thread and will continue executing
        // concurrently.
        thread::spawn(move || {
            // Blocks until a callback is available.
            // When the instance of the `Database` class is dropped, the channel will be
            // closed and `rx.recv()` will return an `Err`, ending the loop and terminating
            // the thread.
            while let Ok((d, callback)) = rx.recv() {
                // Returning `ControlFlow::Break` means to immediately _stop_ the loop and close the database connection
                if callback(&mut conn, &channel, d).is_break() {
                    break;
                }
            }
        });

        Ok(db)
    }

    // Inserts a `name` into the database
    // Accepts a `name` and returns a `Promise<number>`
    fn insert<'cx>(&self, cx: &mut Cx<'cx>, name: String) -> JsResult<'cx, JsPromise> {
        self.exec(cx, move |conn| -> Result<f64, Error> {
            conn.execute(
                "INSERT INTO person (name) VALUES (?)",
                rusqlite::params![name],
            )?;

            Ok(conn.last_insert_rowid() as f64)
        })
    }

    // Get a `name` by `id` value
    // Accepts an `id` and callback as parameters
    fn by_id<'cx>(&self, cx: &mut Cx<'cx>, id: f64) -> JsResult<'cx, JsPromise> {
        self.exec(cx, move |conn| -> Result<Option<String>, Error> {
            match conn
                .prepare("SELECT name FROM person WHERE id = ?")?
                .query_row(rusqlite::params![id], |row| row.get(0))
            {
                Ok(name) => Ok(Some(name)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(err) => Err(Error::from(err)),
            }
        })
    }

    // Many more methods can be added here by using the simple `self.send(..)` pattern.

    // Idiomatic rust would take an owned `self` to prevent use after close
    // However, it's not possible to prevent JavaScript from continuing to hold a closed database.
    // Instead, we send a `ControlFlow::Break` to stop the loop and drop the `mpsc` channel. Once
    // dropped, the channel will be closed and sending messages will result in an error. These
    // errors will be converted into a `Promise` rejection.
    fn close<'cx>(&self, cx: &mut Cx<'cx>) -> JsResult<'cx, JsPromise> {
        self.send(cx, |_| ControlFlow::<(), ()>::Break(()))
    }
}
