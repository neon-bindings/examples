use num_cpus;
use neon::prelude::*;
use neon::register_module;

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

register_module!(mut m, { m.export_function("threadCount", thread_count) });
