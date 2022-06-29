mod curseforge;
mod webfile;
mod filename;

pub trait ModFile {
    fn get_file(self) -> Self;
    fn get_name(&self) -> &str;
}
