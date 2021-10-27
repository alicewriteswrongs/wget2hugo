use std::fs;
use std::io;
use std::path;
use std::time::SystemTime;

/// Get the modified time for a given Path
fn modified_time(path: &path::Path) -> io::Result<SystemTime> {
    fs::metadata(path).and_then(|metadata| metadata.modified())
}

/// Check if the file at `to` is out of date w/r/t the file at `from`.
///
/// Although this function reads from the environment, it unwraps times
/// to safe values which will generally cause it to default to returning
/// `true`, so it should be robust to files which don't exist and whatnot.
pub fn out_of_date(from: &path::Path, to: &path::Path) -> bool {
    let from_modified = modified_time(from).unwrap_or(SystemTime::now());
    let to_modified = modified_time(to).unwrap_or(SystemTime::UNIX_EPOCH);
    return from_modified > to_modified;
}

/// A wrapper around fs::copy which will skip the copy if the destination
/// file is newer than the source file.
///
/// Note that this keeps the normal return value from `fs::copy`, which
/// returns the number of bytes copied. Here if we skip the copy we return
/// `Ok(0)`, so you can tell that the copy was skipped.
pub fn copy_if_src_newer(from: &path::Path, to: &path::Path) -> io::Result<u64> {
    if out_of_date(from, to) {
        info!("copying file {}", to.display());
        fs::copy(from, to)
    } else {
        info!("skipping copy for file {}", to.display());
        Ok(0)
    }
}

/// Creates a directory if it does not exist, otherwise, it doesn't!
pub fn mkdir(path: &path::Path) {
    match fs::create_dir(path) {
        Ok(_) => {
            info!("created directory {}", path.display());
        }
        Err(e) => {
            info!("skipping directory creation, {}", e);
        }
    }
}
