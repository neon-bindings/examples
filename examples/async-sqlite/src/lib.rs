use std::{ops::ControlFlow, sync::mpsc, thread};

use neon::{
    prelude::*,
    types::Deferred,
    types::extract::{Error, TryIntoJs},
};

use rusqlite::Connection;

type DbCallback =
    Box<dyn FnOnce(&mut Connection, &Channel, Deferred) -> ControlFlow<(), ()> + Send>;

// Wraps a SQLite connection a channel, allowing concurrent access
struct Database {
    tx: mpsc::Sender<(Deferred, DbCallback)>,
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
        let (d, promise) = cx.promise();
        let Err(mpsc::SendError((d, _))) = self.tx.send((
            d,
            Box::new(move |conn, ch, d| {
                let output = f(conn).continue_value();
                let should_break = output.is_none();
                let res = d.try_settle_with(ch, move |mut cx| output.try_into_js(&mut cx));

                // If we can no longer settle promises, we can stop the worker thread
                if res.is_err() || should_break {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            }),
        )) else {
            return Ok(promise);
        };

        let err = cx.error("Database is closed")?;
        d.reject(cx, err);

        Ok(promise)
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

        // Create an `Channel` for calling back to JavaScript. It is more efficient
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

        // Spawn a thread for processing database queries
        // This will not block the JavaScript main thread and will continue executing
        // concurrently.
        thread::spawn(move || {
            // Blocks until a callback is available
            // When the instance of `Database` is dropped, the channel will be closed
            // and `rx.recv()` will return an `Err`, ending the loop and terminating
            // the thread.
            while let Ok((d, callback)) = rx.recv() {
                // Returning `true` means to _stop_
                if callback(&mut conn, &channel, d).is_break() {
                    break;
                }
            }
        });

        Ok(db)
    }

    // Inserts a `name` into the database
    // Accepts a `name` and returns a `Promise`
    fn insert<'cx>(&self, cx: &mut Cx<'cx>, name: String) -> JsResult<'cx, JsPromise> {
        self.exec(cx, move |conn| -> Result<_, Error> {
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

    // Idiomatic rust would take an owned `self` to prevent use after close
    // However, it's not possible to prevent JavaScript from continuing to hold a closed database
    fn close<'cx>(&self, cx: &mut Cx<'cx>) -> JsResult<'cx, JsPromise> {
        self.send(cx, |_| ControlFlow::<(), ()>::Break(()))
    }
}
