#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileSystemObject {
    Directory {
        name: String,
        contents: Vec<FileSystemObject>,
    },

    File {
        name: String,
        size: u64,
    },
}

impl FileSystemObject {
    pub fn calculate_size(&self) -> u64 {
        match self {
            FileSystemObject::Directory { contents, .. } => {
                contents.iter().map(|obj| obj.calculate_size()).sum()
            }
            FileSystemObject::File { size, .. } => *size,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            FileSystemObject::Directory { name, .. } => name,
            FileSystemObject::File { name, .. } => name,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileSystemObject::File { .. })
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, FileSystemObject::Directory { .. })
    }

    pub fn is_file_with_name(&self, file_name: &str) -> bool {
        match self {
            FileSystemObject::Directory { .. } => false,
            FileSystemObject::File { name, .. } => name == file_name,
        }
    }

    pub fn is_dir_with_name(&self, dir_name: &str) -> bool {
        match self {
            FileSystemObject::Directory { name, .. } => dir_name == name,
            FileSystemObject::File { .. } => false,
        }
    }

    pub fn add_dir(&mut self, dir_name: String) -> &mut FileSystemObject {
        let new_dir = FileSystemObject::Directory {
            name: dir_name,
            contents: vec![],
        };

        match self {
            FileSystemObject::Directory { contents, .. } => {
                contents.push(new_dir);
                // Always succeeds since we just pushed a value into it
                contents.last_mut().unwrap()
            }
            FileSystemObject::File { .. } => panic!("Attempted to add dir to file"),
        }
    }

    pub fn add_file(&mut self, file_name: String, size: u64) -> &mut FileSystemObject {
        let new_dir = FileSystemObject::File {
            name: file_name,
            size,
        };

        match self {
            FileSystemObject::Directory { contents, .. } => {
                contents.push(new_dir);
                // Always succeeds since we just pushed a value into it
                contents.last_mut().unwrap()
            }
            FileSystemObject::File { .. } => panic!("Attempted to add file to file"),
        }
    }

    pub fn find_dir(&self, name: &str) -> &FileSystemObject {
        match self {
            FileSystemObject::Directory { contents, .. } => {
                contents.iter().find(|n| n.is_dir_with_name(name)).unwrap()
            }

            FileSystemObject::File { .. } => panic!("Attempting to search a file"),
        }
    }

    pub fn find_dir_mut(&mut self, name: &str) -> &mut FileSystemObject {
        match self {
            FileSystemObject::Directory { contents, .. } => contents
                .iter_mut()
                .find(|n| n.is_dir_with_name(name))
                .unwrap(),

            FileSystemObject::File { .. } => panic!("Attempting to search a file"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileSystem {
    pub root: FileSystemObject,
}

impl FileSystem {
    pub fn get_size(&self) -> u64 {
        self.root.calculate_size()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            root: FileSystemObject::Directory {
                name: "/".to_owned(),
                contents: vec![],
            },
        }
    }
}
