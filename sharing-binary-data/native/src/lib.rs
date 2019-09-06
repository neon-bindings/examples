use neon::prelude::*;
use neon::register_module;
// use cslice::CMutSlice;
// use std::mem;

fn hello(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let buffer = JsArrayBuffer::new(&mut cx, 8)?;
    // cx.borrow_mut(&mut buffer, |mut slice| {
    //     let len = slice.len();
    //     let raw = slice.as_mut_ptr();
    //     let mut slice: CMutSlice<f32> = unsafe {
    //         CMutSlice::new(mem::transmute(raw), len / 4)
    //     };
    //     slice[0] = 1.8;
    //     slice[1] = 13.4;
    // });
    Ok(buffer)
}

register_module!(mut m, { m.export_function("hello", hello) });
