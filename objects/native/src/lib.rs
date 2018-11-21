#[macro_use]
extern crate neon;

use neon::prelude::*;

// Mapping a struct to a JsObject
// Here is a simple example of converting a rust Struct to a JS Object using JsObject:
struct Foo {
    pub bar: u64,
    pub baz: String
}

// Convert a Rust struct to a JsObject
fn convert_struct_to_js_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let foo = Foo {
        bar: 1234,
        baz: "baz".to_string()
    };
    let object = JsObject::new(&mut cx);
    let js_string = cx.string(&foo.baz);
    let js_number = cx.number(foo.bar as f64);
    object.set(&mut cx, "myStringProperty", js_string).unwrap();
    object.set(&mut cx, "myNumberProperty", js_number).unwrap();
    Ok(object)
}

register_module!(mut m, {
    m.export_function("convertStructToJsObject", convert_struct_to_js_object)
});
