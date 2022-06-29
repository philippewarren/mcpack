use std::error::Error;
use std::fs::File;
use std::io;

use reqwest::blocking as breqwest;

use super::filename;

pub fn get_file(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = breqwest::get(url)?;
    let file_name: &str = filename::get_filename_from_url(url)?;
    let mut out = File::create(file_name)?;
    io::copy(&mut resp.bytes()?.as_ref(), &mut out)?;
    Ok(String::from(file_name))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    #[test]
    fn get_file_test() {
        env::set_current_dir(env::temp_dir()).unwrap();

        let name =
            super::get_file("https://www.rust-lang.org/logos/rust-logo-128x128-blk.png").unwrap();
        assert_eq!(name, "rust-logo-128x128-blk.png");
        assert!(Path::new(&name).exists());
        assert!(Path::new(&name).is_file());
        fs::remove_file(&name).unwrap();
    }
}
