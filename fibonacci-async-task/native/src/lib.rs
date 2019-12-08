use neon::prelude::*;
use neon::register_module;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

fn compute(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn fibonacci_sync(mut cx: FunctionContext) -> JsResult<JsString> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;
    let big = compute(n);
    Ok(cx.string(big.to_str_radix(10)))
}

struct FibonacciTask {
    argument: usize,
}

impl Task for FibonacciTask {
    type Output = BigUint;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<BigUint, ()> {
        Ok(compute(self.argument))
    }

    fn complete(self, mut cx: TaskContext, result: Result<BigUint, ()>) -> JsResult<JsString> {
        Ok(cx.string(result.unwrap().to_str_radix(10)))
    }
}

fn fibonacci_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;
    let cb = cx.argument::<JsFunction>(1)?;

    let task = FibonacciTask { argument: n };
    task.schedule(cb);

    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("fibonacciSync", fibonacci_sync)?;
    m.export_function("fibonacci", fibonacci_async)?;
    Ok(())
});
