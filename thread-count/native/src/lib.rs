#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::prelude::*;

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

register_module!(mut m, {
    m.export_function("threadCount", thread_count)
});
