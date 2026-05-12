
use serde_json::{Value, json};

/* ============================
    FUNCTIONS
    - Define your functions here as `fn function_name(args: &Value) -> Result<Value, String>`.
    - Example:
        pub fn ping(_args: &Value) -> Result<Value, String> {
            Ok(json!({"pong": true}))
        }
   - You must add your functions to dispatcher.rs to expose them.
    - Functions should:
        * Read inputs from `args` (array or object as you prefer).
        * Return `Ok(Value)` on success, or `Err(String)` on error.
    - If you need filesystem access, use relative paths such as "data/file.txt".
        Relative paths are resolved inside the plugin's persistent storage directory.
    - Anything written to stderr is captured by Desktopr and returned in the plugin call
        response under the `stderr` field.
        Example output: { "stderr": "entered _start\nreading input\ndispatching function\n" }
    - Do not print to stdout/stderr from your functions unless you are intentionally logging
        to stderr. The final JSON response is printed to stdout by the template runtime.
============================ */


// WRITE YOUR FUNCTIONS HERE

// IMPORTANT: You must add your functions to dispatcher.rs to expose them.




/* ============================
    LOGGING WITH `stderr`
    - `stderr` is reserved for debug logs and diagnostic messages.
    - Anything written to `stderr` is captured by Desktopr and returned
        in the plugin call response under the `stderr` field.
    - Do not use `stderr` for the final result; print the JSON response to `stdout`.

    EXAMPLE
    - In your plugin code:
        let _ = std::io::stderr().write_all(b"log message 1\n");
        let _ = std::io::stderr().write_all(b"log message 2\n");

    - Output:
        {
            ...,
            "stderr": "log message 1\nlog message 2\n"
        }
============================ */