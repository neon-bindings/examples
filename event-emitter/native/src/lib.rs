#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use neon::context::{Context, FunctionContext, TaskContext};
use neon::object::Object;
use neon::result::{JsResult, JsResultExt};
use neon::task::Task;
use neon::types::{
	JsArray,
	JsFunction,
	JsNumber,
	JsObject,
	JsString,
	JsUndefined,
	JsValue,
};

use serde_bytes::ByteBuf;

#[derive(Deserialize)]
struct Request {
	body: ByteBuf,
}

#[derive(Deserialize)]
struct HelloRequest {
	name: String,
}

#[derive(Serialize)]
struct HelloResponse {
	greeting: String,
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
	let arg0 = cx.argument(0)?;

	let req: Request = neon_serde::from_value(&mut cx, arg0)?;
	let req_body: HelloRequest = serde_json::from_slice(&req.body)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let res = HelloResponse {
		greeting: format!("Hello, {}!", req_body.name),
	};

	let res_body = serde_json::to_vec(&res)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let ret = neon_serde::to_value(&mut cx, &ByteBuf::from(res_body))?;

	Ok(ret)
}

fn parse(mut cx: FunctionContext) -> JsResult<JsValue> {
	let s = cx.argument::<JsString>(0)?;

	let o: serde_json::Value = serde_json::from_str(&s.value())
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let o = neon_serde::to_value(&mut cx, &o)?;

	Ok(o)
}

fn stringify(mut cx: FunctionContext) -> JsResult<JsString> {
	let o = cx.argument(0)?;

	let o: serde_json::Value = neon_serde::from_value(&mut cx, o)?;
	let s = serde_json::to_string(&o)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	Ok(JsString::new(&mut cx, &s))
}

struct SuccessTask;

impl Task for SuccessTask {
	type Output = i32;
	type Error = String;
	type JsEvent = JsNumber;

	fn perform(&self) -> Result<Self::Output, Self::Error> {
		Ok(17)
	}

	fn complete(
		self,
		mut cx: TaskContext,
		result: Result<Self::Output, Self::Error>,
	) -> JsResult<Self::JsEvent> {
		Ok(JsNumber::new(&mut cx, result.unwrap() as f64))
	}
}

fn perform_async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let f = cx.argument::<JsFunction>(0)?;
	SuccessTask.schedule(f);
	Ok(JsUndefined::new())
}

fn array_process(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let arr = cx.argument::<JsArray>(0)?;

	for i in 0..arr.len() {
		let item = arr
			.get(&mut cx, i)?
			.downcast::<JsObject>()
			.or_throw(&mut cx)?;
		let operator = item
			.get(&mut cx, "operator")?
			.downcast::<JsString>()
			.or_throw(&mut cx)?
			.value();
		let value = item
			.get(&mut cx, "value")?
			.downcast::<JsString>()
			.or_throw(&mut cx)?
			.value();

		match operator.as_str() {
			"print" => {
				println!("{}", value);
			}
			_ => {
				let msg = format!("Unsupported operator: {}", operator);

				return cx.throw_error(&msg);
			}
		}
	}

	Ok(JsUndefined::new())
}

#[derive(Deserialize)]
#[serde(tag = "operator")]
enum Operation {
	#[serde(rename = "print")]
	Print { value: String },
}

fn array_process_serde(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let arg0 = cx.argument(0)?;
	let ops: Vec<Operation> = neon_serde::from_value(&mut cx, arg0)?;

	for op in ops {
		match op {
			Operation::Print { value } => {
				println!("{}", value);
			}
		}
	}

	Ok(JsUndefined::new())
}

pub enum Event {
	Tick { count: f64 }
}

fn event_thread(rx: mpsc::Receiver<()>) -> mpsc::Receiver<Event> {
	let (tx, events) = mpsc::channel();

	thread::spawn(move || {
		let mut count = 0.0;

		loop {
			thread::sleep(Duration::from_millis(500));

			match rx.try_recv() {
				Ok(_) | Err(TryRecvError::Disconnected) => {
					break;
				}
				Err(TryRecvError::Empty) => {}
			}

			tx.send(Event::Tick { count }).expect("Send failed");
			count += 1.0;
		}
	});

	events
}

pub struct EventEmitter {
	events: Arc<Mutex<mpsc::Receiver<Event>>>,
	shutdown: mpsc::Sender<()>,
}

pub struct EventEmitterTask(Arc<Mutex<mpsc::Receiver<Event>>>);

impl Task for EventEmitterTask {
	type Output = Event;
	type Error = String;
	type JsEvent = JsObject;

	fn perform(&self) -> Result<Self::Output, Self::Error> {
		let rx = self.0.lock()
			.map_err(|_| "Could not obtain lock on receiver".to_string())?;

		rx.recv()
			.map_err(|_| "Failed to receive event".to_string())
	}

	fn complete(
		self,
		mut cx: TaskContext,
		event: Result<Self::Output, Self::Error>,
	) -> JsResult<Self::JsEvent> {
		let event = event
			.or_else(|err| cx.throw_error(&err.to_string()))?;

		let o = cx.empty_object();

		match event {
			Event::Tick { count } => {
				let event_name = cx.string("tick");
				let event_count = cx.number(count);

				o.set(&mut cx, "event", event_name)?;
				o.set(&mut cx, "count", event_count)?;
			},
		}

		Ok(o)
	}
}

declare_types! {
	pub class JsEventEmitter for EventEmitter {
		init(_) {
			let (shutdown, shutdown_rx) = mpsc::channel();
			let rx = event_thread(shutdown_rx);

			Ok(EventEmitter {
				events: Arc::new(Mutex::new(rx)),
				shutdown,
			})
		}

		method poll(mut cx) {
			let cb = cx.argument::<JsFunction>(0)?;
			let this = cx.this();
			let events = cx.borrow(&this, |emitter| Arc::clone(&emitter.events));
			let emitter = EventEmitterTask(events);

			emitter.schedule(cb);

			Ok(JsUndefined::new().upcast())
		}

		method shutdown(mut cx) {
			let this = cx.this();

			cx.borrow(&this, |emitter| emitter.shutdown.send(()))
				.or_else(|err| cx.throw_error(&err.to_string()))?;

			Ok(JsUndefined::new().upcast())
		}
	}
}

register_module!(mut cx, {
	cx.export_function("parse", parse)?;
	cx.export_function("stringify", stringify)?;
	cx.export_function("hello", hello)?;
	cx.export_function("performAsyncTask", perform_async_task)?;
	cx.export_function("arrayProcess", array_process)?;
	cx.export_function("arrayProcessSerde", array_process_serde)?;
	cx.export_class::<JsEventEmitter>("EventEmitter")?;

	Ok(())
});
