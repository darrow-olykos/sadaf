use std::collections::HashMap;
use std::path::PathBuf;

type FileMap = HashMap<String, PathBuf>;

pub struct FileIndex {
    png: FileMap,
    jpg: FileMap,
    gif: FileMap,
    txt: FileMap,
    doc: FileMap,
    ppt: FileMap,
}

impl FileIndex {
    pub fn new() -> Self {
        Self {
            png: HashMap::new(),
            jpg: HashMap::new(),
            gif: HashMap::new(),
            txt: HashMap::new(),
            doc: HashMap::new(),
            ppt: HashMap::new()
        }
    }

    fn get_file_map(&mut self, file_ext: &str) -> &mut FileMap {
        match file_ext {
            "png" => &mut self.png,
            "jpg" | "jpeg" => &mut self.jpg,
            "gif" => &mut self.gif,
            "txt" => &mut self.txt,
            "doc" | "docx" => &mut self.doc,
            "ppt" | "pptx" =>&mut self.ppt,
            _ => panic!("ext not handled")
        }
    }

    pub fn set(&mut self, file_ext: &str, path_buf: PathBuf) {
        let ext = String::from(file_ext);
        self.get_file_map(file_ext).insert(ext, path_buf);
    }

    pub fn get(&mut self, file_ext: &str) -> Vec<PathBuf> {
        self.get_file_map(file_ext).into_iter().map(|(k, v)|{v.to_owned()}).collect()
    }
}