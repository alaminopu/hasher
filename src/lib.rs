use neon::prelude::*;
use crypto_hash::{Algorithm, hex_digest};


fn hash(mut cx: FunctionContext) -> JsResult<JsString> {
    let data = cx.argument::<JsString>(0)?;
    let result = hex_digest(Algorithm::MD5, data.value(&mut cx).as_bytes());
    Ok(cx.string(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hash", hash)?;
    Ok(())
}
