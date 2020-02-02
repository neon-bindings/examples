use neon::prelude::*;
use neon::{declare_types, register_module};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[allow(dead_code)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

declare_types! {
  pub class JsUser for User {
    init(mut cx) {
      let id = cx.argument::<JsNumber>(0)?;
      let first_name: Handle<JsString> = cx.argument::<JsString>(1)?;
      let last_name: Handle<JsString> = cx.argument::<JsString>(2)?;
      let email: Handle<JsString> = cx.argument::<JsString>(3)?;

      Ok(User {
        id: id.value() as i32,
        first_name: first_name.value(),
        last_name: last_name.value(),
        email: email.value(),
      })
    }
  }
}

register_module!(mut m, {
    // Export a function
    m.export_function("hello", hello)?;
    // Export a class
    m.export_class::<JsUser>("User")?;
    // Export strings, numbers, booleans, etc
    let baz = m.string("baz");
    m.export_value("baz", baz)?;
    Ok(())
});
