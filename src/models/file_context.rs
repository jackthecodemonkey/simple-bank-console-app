use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct file_context<'a> {
    pub db_context: File,
    pub open_options: File,
    pub path: &'a str,
}

impl file_context<'_> {
    pub fn new(path: &str) -> file_context {
        let file_path = Path::new(path);
        let display = file_path.display();
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => panic!("couldn't open {}", display),
        };
        let open_options = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&file_path)
            .unwrap();
        file_context {
            db_context: file,
            open_options: open_options,
            path,
        }
    }
}