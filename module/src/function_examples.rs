// This file contains example methods that you can use for reference
use serde_json::{Value, json};
use std::fs;


// olugin fs examples

pub fn eg_storage_write(args: &Value) -> Result<Value, String> {
    let path = args
        .get("path")
        .and_then(|x| x.as_str())
        .unwrap_or("test-storage.txt");

    let contents = args
        .get("contents")
        .and_then(|x| x.as_str())
        .unwrap_or("Hello from Desktopr plugin storage");

    fs::write(path, contents).map_err(|e| e.to_string())?;

    Ok(json!({
        "written": true,
        "path": path,
        "bytes": contents.as_bytes().len()
    }))
}

pub fn eg_storage_read(args: &Value) -> Result<Value, String> {
    let path = args
        .get("path")
        .and_then(|x| x.as_str())
        .unwrap_or("test-storage.txt");

    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

    Ok(json!({
        "read": true,
        "path": path,
        "contents": contents
    }))
}


// math examples

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