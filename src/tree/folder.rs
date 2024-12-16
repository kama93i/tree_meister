use std::path::PathBuf;

enum Content {
    Folder,
    File,
}       

pub struct Folder {
    path: PathBuf,
    content: Vec<Content>,
}

impl Folder {
    fn new(path: PathBuf, content: Vec<Content>) -> Self {
        Self { path, content }
    }

    pub fn get_content(&self) -> Vec<Content> {
        &self.content
    }
}
