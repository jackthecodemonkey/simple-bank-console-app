use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct FileContext<'a> {
    pub dbContext: File,
    pub openOptions: File,
    pub path: &'a str,
}

impl FileContext<'_> {
    pub fn new(path: &str) -> FileContext {
        let file_path = Path::new(path);
        let display = file_path.display();
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => panic!("couldn't open {}", display),
        };
        let openOptions = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&file_path)
            .unwrap();
        FileContext {
            dbContext: file,
            openOptions: openOptions,
            path,
        }
    }
}