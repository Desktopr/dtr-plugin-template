/* ============================
   DISPATCHER
   - Do not rename `pub fn dispatch` or change its signature.
   - Desktopr's plugin runtime uses this function to route calls from the `fn` input field.
   - Only edit the content of `match op` block to register your plugin functions.
   - Do not edit imports.
============================ */

use serde_json::{Value, json};
use crate::functions::*;

pub fn dispatch(op: &str, args: &Value) -> Result<Value, String> {
    match op {
        
        /*================== EXAMPLE FUNCTIONS ==================*/
        /*============== you can remove this block ==============*/
        /*=======================================================*/
        /*===*/                                             /*===*/
        /*===*/ "dividePos" => eg_divide_positional(args),  /*===*/
        /*===*/ "divide" => eg_divide(args),                /*===*/
        /*===*/ "greet" => eg_greet(args),                  /*===*/
        /*===*/ "write" => eg_storage_write(args),          /*===*/
        /*===*/ "read" => eg_storage_read(args),            /*===*/
        /*===*/                                             /*===*/
        /*=======================================================*/
        /*============== you can remove this block ==============*/
        /*=======================================================*/


        // ADD HERE YOUR ACTUAL FUNCTIONS

        // e.g. "yourFunctionName" => your_function(args),


        _ => Err(format!("unknown function: {}", op)),
    }
}