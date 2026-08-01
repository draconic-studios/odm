use odm_core::{exit_code, OdmError};
use serde::Serialize;

pub struct GlobalOut {
    pub json: bool,
}

pub fn print_json<T: Serialize>(value: &T) -> Result<(), OdmError> {
    let s = serde_json::to_string_pretty(value)
        .map_err(|e| OdmError::operation(format!("json encode failed: {e}")))?;
    println!("{s}");
    Ok(())
}

/// Print error (human stderr or JSON stdout) and return exit code.
pub fn print_error(out: &GlobalOut, err: &OdmError) -> i32 {
    let code = exit_code(err);
    if out.json {
        let detail = err.detail();
        let body = serde_json::json!({
            "ok": false,
            "error": {
                "code": err.code(),
                "message": err.message(),
                "detail": detail,
            }
        });
        match serde_json::to_string_pretty(&body) {
            Ok(s) => println!("{s}"),
            Err(_) => eprintln!("error: {}", err.message()),
        }
    } else {
        let msg = err.message();
        if let Some((first, rest)) = msg.split_once('\n') {
            eprintln!("error: {first}");
            if !rest.is_empty() {
                eprintln!("{rest}");
            }
        } else {
            eprintln!("error: {msg}");
        }
    }
    code
}
