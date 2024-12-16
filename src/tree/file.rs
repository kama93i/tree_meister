use std::path::PathBuf;

pub struct File {
    pub path: PathBuf
}

impl File {
   fn new(path: PathBuf) -> Self {
        Self { path }
   } 
}

