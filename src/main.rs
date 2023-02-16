use std::{ffi::OsStr, fs, path::PathBuf};

extern crate markdown;

fn main() {
    for path in list_of_md_files("./docs").unwrap() {
        let path_string = path.into_os_string().into_string().unwrap();
        convert_to_html(path_string);
    }
}

// Read content of the markdown file in /docs
// convert md to html using crate 'markdown'
// write html in new file in /docs
fn convert_to_html(file_path: String) {
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let binding = markdown::to_html(contents.as_str());
    fs::write(format!("{}.html", &file_path), binding).expect("Unable to write file");
}

fn list_of_md_files(root: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("md") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}
