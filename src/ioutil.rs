use std::fs;
use std::io;
use std::path;
use std::time;

/// Get the modified time for a given Path
fn modified_time(path: &path::Path) -> io::Result<time::SystemTime> {
    fs::metadata(path).and_then(|metadata| metadata.modified())
}

/// A wrapper around fs::copy which will skip the copy if the destination
/// file is newer than the source file.
///
/// Note that this keeps the normal return value from `fs::copy`, which
/// returns the number of bytes copied. Here if we skip the copy we return
/// `Ok(0)`, so you can tell that the copy was skipped.
pub fn copy_if_src_newer(from: &path::Path, to: &path::Path) -> io::Result<u64> {
    let from_modified = modified_time(from)?;
    let to_modified = modified_time(to).unwrap_or(time::SystemTime::UNIX_EPOCH);

    if from_modified > to_modified {
        fs::copy(from, to)
    } else {
        Ok(0)
    }
}

/// Creates a directory if it does not exist, otherwise, it doesn't!
pub fn mkdir(path: &path::Path) {
    match fs::create_dir(path) {
        Ok(_) => {
            println!("created directory {}", path.display());
        }
        Err(e) => {
            println!("{} happend when creating directory", e);
        }
    }
}
