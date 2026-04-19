// This file contains example methods that you can use for reference

use serde_json::{Value, json};


// named arguments examples (eg. { "a": 16, "b": 2 }):

pub fn eg_divide(args: &Value) -> Result<Value, String> {

    let a = args.get("a").and_then(|x| x.as_f64()).unwrap_or(0.0);
    let b = args.get("b").and_then(|x| x.as_f64()).unwrap_or(0.0);
    
    if b == 0.0 {
        return Err("division by zero".to_string());
    }

    Ok(json!(a / b))
}

pub fn eg_greet(args: &Value) -> Result<Value, String> {

    let name = args.get("name").and_then(|x| x.as_str()).unwrap_or("world");

    Ok(json!(format!("Hello {}", name)))
}



// positional arguments example (eg. [16,2]):

pub fn eg_divide_positional(args: &Value) -> Result<Value, String> {

    let a = args.get(0).and_then(|x| x.as_f64()).unwrap_or(0.0);
    let b = args.get(1).and_then(|x| x.as_f64()).unwrap_or(0.0);

    if b == 0.0 {
        return Err("division by zero".to_string());
    }

    Ok(json!(a / b))
}
