use std::fs::File;
use std::io::Write;
use std::{ffi::OsStr, fs, path::PathBuf};

extern crate markdown;

fn main() {
    for file_path in list_of_md_files("../").unwrap() {
        // println!("file dsada : {:?}", file_path); // debug
        // get file name
        let filename = file_path.file_stem().unwrap().to_str().unwrap();
        if filename != "README" {
            // read
            let contents =
                fs::read_to_string(&file_path).expect("Should have been able to read the file");
            // convert
            let binding = markdown::to_html(contents.as_str()); // Convert .md to .html using crate 'markdown'
                                                                // println!("{}", binding); // debug
                                                                // 4. create new path (in directory  "/_site")
            let mut new_entry = File::create(format!("../_site/{}.html", filename)).unwrap();
            // write html in new path :
            new_entry
                .write_all(binding.as_bytes())
                .expect("Unable to write file");
        }
    }
}

// Read content of the markdown file in /docs
fn list_of_md_files(root: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if let Some("md") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}
