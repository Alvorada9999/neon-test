use neon::prelude::*;

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let mut x: f64 = 1.0;
    let mut y: f64 = 1.0;
    let mut result: f64 = 1.0;
    let mut k: f64 = 1.0;
    let number: f64 = cx.argument::<JsNumber>(0)?.value();
    while k < number {
        result = x + y;
        y = x;
        x = result;
        k += 1.0;
    }
    Ok(cx.number(result))
}

register_module!(mut second_module, {
    second_module.export_function("fibonacci", fibonacci)
});