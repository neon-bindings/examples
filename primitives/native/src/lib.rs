use neon::prelude::*;
use neon::register_module;

fn primitives(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Either use function context to create number (preferred)
    let _number = cx.number(23 as f64);
    // or use JsNumber struct
    let _number = JsNumber::new(&mut cx, 23);

    // Other primitives follow a similar pattern:

    // Strings
    let _string = cx.string("foobar"); // preferred
    let _string = JsString::new(&mut cx, "foobar");

    // Booleans
    let _boolean = cx.boolean(true); // preferred
    let _boolean = JsBoolean::new(&mut cx, true);

    // Undefined
    let _undefined = cx.undefined(); // preferred
    let _undefined = JsUndefined::new();

    // Null
    let _null = cx.null(); // preferred
    let _null = JsNull::new();

    Ok(cx.undefined())
}

register_module!(mut m, { m.export_function("primitives", primitives) });
