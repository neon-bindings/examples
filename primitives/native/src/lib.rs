use neon::prelude::*;
use neon::register_module;

/// Generate a value of the `Boolean`(JS) type
fn generate_boolean(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // Either use function context to create a value (preferred)
    let boolean = cx.boolean(true); // preferred
    let _boolean = JsBoolean::new(&mut cx, true); // or
    Ok(boolean)
}

/// Generate a value of the `Null`(JS) type
fn generate_null(mut cx: FunctionContext) -> JsResult<JsNull> {
    let null = cx.null(); // preferred
    let _null = JsNull::new(); // or
    Ok(null)
}

/// Generate a value of the `Undefined`(JS) type
fn generate_undefined(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let undefined = cx.undefined(); // preferred
    let _undefined = JsUndefined::new();
    Ok(undefined)
}

/// Generate a value of the `Number`(JS) type
fn generate_number(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let number = cx.number(23 as f64);
    // or use JsNumber struct
    let _number = JsNumber::new(&mut cx, 23);
    Ok(number)
}

// TODO: https://github.com/neon-bindings/neon/issues/376
// /// Generate a value of the `BigInt`(JS) type
// fn generate_bigint(mut cx: FunctionContext) -> JsResult<JsBigInt> {
//     unimplemented!()
// }

/// Generate a value of the `String`(JS) type
fn generate_string(mut cx: FunctionContext) -> JsResult<JsString> {
    // Strings
    let string = cx.string("foobar"); // preferred
    let _string = JsString::new(&mut cx, "foobar");
    Ok(string)
}

// TODO: https://github.com/neon-bindings/neon/issues/502
// /// Generate a value of the `Symbol`(JS) type
// fn generate_symbol(mut cx: FunctionContext) -> JsResult<JsSymbol> {
//     unimplemented!()
// }

register_module!(mut m, {
    m.export_function("generateBoolean", generate_boolean)?;
    m.export_function("generateNull", generate_null)?;
    m.export_function("generateUndefined", generate_undefined)?;
    m.export_function("generateNumber", generate_number)?;
    // m.export_function("generateBigInt", generate_bigint)?;
    m.export_function("generateString", generate_string)?;
    // m.export_function("generateSymbol", generate_symbol)?;
    Ok(())
});
