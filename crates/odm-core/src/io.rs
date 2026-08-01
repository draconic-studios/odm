use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::OdmError;

/// Atomic write: temp sibling then rename over target.
pub fn atomic_write(path: &Path, contents: &str) -> Result<(), OdmError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|e| {
        OdmError::operation(format!("failed to create {}: {e}", parent.display()))
    })?;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("odm.tmp");
    let tmp = parent.join(format!(".{file_name}.{nanos}.tmp"));

    {
        let mut f = fs::File::create(&tmp).map_err(|e| {
            OdmError::operation(format!("failed to create temp {}: {e}", tmp.display()))
        })?;
        f.write_all(contents.as_bytes()).map_err(|e| {
            OdmError::operation(format!("failed to write temp {}: {e}", tmp.display()))
        })?;
        f.sync_all().ok();
    }

    fs::rename(&tmp, path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        OdmError::operation(format!(
            "failed to rename {} -> {}: {e}",
            tmp.display(),
            path.display()
        ))
    })?;
    Ok(())
}
