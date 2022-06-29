use std::error::Error;
use std::path::Path;

pub fn get_filename_from_url(url: &str) -> Result<&str, Box<dyn Error>> {
    let name = Path::new(url)
        .file_name()
        .ok_or(format!("Can't make path from url {}", url))?
        .to_str()
        .ok_or(format!("Can't convert &OsStr to &str for path from url {}", url))?;
    Ok(name)
}
