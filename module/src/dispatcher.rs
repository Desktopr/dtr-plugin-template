
use serde_json::{Value, json};
use crate::function_examples::*; // just for reference/examples
use crate::functions::*;

/// Dispatcher: map "fn" field to functions name.
/// Add your custom functions in this match.
pub fn dispatch(op: &str, args: &Value) -> Result<Value, String> {
    match op {
        "ping" => ping(args),

        "dividePos" => eg_divide_positional(args),       // this is from function_examples.rs, you can remove this
        "divide" => eg_divide(args),       // this is from function_examples.rs, you can remove this
        "greet" => eg_greet(args),   // this is from function_examples.rs, you can remove this
        "write" => eg_storage_write(args),   // this is from function_examples.rs, you can remove this
        "read" => eg_storage_read(args),   // this is from function_examples.rs, you can remove this

        /* ============================

            ADD HERE YOUR FUNCTIONS

            e.g. "yourFunctionName" => your_function(args),

        ============================ */

        _ => Err(format!("unknown function: {}", op)),
    }
}