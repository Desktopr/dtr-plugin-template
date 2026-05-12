/* ============================
   DISPATCHER
   - Do not rename `pub fn dispatch` or change its signature.
   - Desktopr's plugin runtime uses this function to route calls from the `fn` input field.
   - Only edit the `match op` block to register your plugin functions.
============================ */

use serde_json::{Value, json}; // required
use crate::functions::*;

use crate::function_examples::*; // just for reference/examples (can be removed)


pub fn dispatch(op: &str, args: &Value) -> Result<Value, String> {
    match op {
        
        /*================== EXAMPLE FUNCTIONS ==================*/
        /*============== you can remove this block ==============*/
        /*=======================================================*/
        /*===*/                                             /*===*/
        /*===*/ "ping" => ping(args),                       /*===*/
        /*===*/ "dividePos" => eg_divide_positional(args),  /*===*/
        /*===*/ "divide" => eg_divide(args),                /*===*/
        /*===*/ "greet" => eg_greet(args),                  /*===*/
        /*===*/ "write" => eg_storage_write(args),          /*===*/
        /*===*/ "read" => eg_storage_read(args)             /*===*/
        /*===*/                                             /*===*/
        /*=======================================================*/
        /*============== you can remove this block ==============*/
        /*=======================================================*/


        // ADD HERE YOUR ACTUAL FUNCTIONS

        // e.g. "yourFunctionName" => your_function(args),


        _ => Err(format!("unknown function: {}", op)),
    }
}