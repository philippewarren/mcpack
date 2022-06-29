use super::ModFile;
use super::webfile;

pub struct CurseforgeModFile {
    url: String,
    name: String,
}

impl ModFile for CurseforgeModFile {
    fn get_file(mut self) -> Self {
        self.name = webfile::get_file(&self.url).unwrap();
        self
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
