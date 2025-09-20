enum FileSystem {
  File(String, usize),
  Folder(String, Vec<FileSystem>),
}

fn total_size(fs: &FileSystem) -> usize {
  match fs {
      FileSystem::File(_f, size) => *size,
      FileSystem::Folder(_folder, files) => {
        let mut size = 0;
        for file in files {
          size += total_size(file)
        }
        size
      }
  }
}


fn list_files(fs: &FileSystem, indent: usize) {
    match fs {
        FileSystem::File(name, _) => {
            println!("{}- {}", " ".repeat(indent), name);
        }
        FileSystem::Folder(name, files) => {
            println!("{}+ {}", " ".repeat(indent), name);
            for file in files {
                list_files(file, indent + 2);
            }
        }
    }
}

fn main() {
    let fs = FileSystem::Folder(
        "root".to_string(),
        vec![
            FileSystem::File("a.txt".to_string(), 10),
            FileSystem::Folder(
                "docs".to_string(),
                vec![
                    FileSystem::File("b.txt".to_string(), 20),
                    FileSystem::File("c.txt".to_string(), 30),
                ],
            ),
        ],
    );

    println!("Total size: {}", total_size(&fs));
    println!("File tree:");
    list_files(&fs, 0);
}