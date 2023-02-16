use static_site_generator::ThreadPool;
use std::{
    ffi::OsStr,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread::{self},
    time::Duration,
};

extern crate markdown;

fn main() {
    // loop over markdown files in docs directory
    for path in list_of_md_files("./docs").unwrap() {
        let path_string = path.into_os_string().into_string().unwrap();
        convert_to_html(path_string);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); //create a new thread pool with a configurable number of threads instead `thread::spawn`

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn convert_to_html(file_path: String) {
    // Read content of the markdown file
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    // convert md to html with crate 'markdown'
    let binding = markdown::to_html(contents.as_str());
    // println!("{binding}"); // test debug

    // write html in new file
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

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        //  We need to explicitly match on a slice of request_line to pattern match against the string literal values; match doesn’t do automatic referencing and dereferencing like the equality method does.
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /blog/myfile HTTP/1.1" => ("HTTP/1.1 200 OK", "docs/myfile.md.html"), // go to http://localhost:7878/blog/myfile
        "GET /sleep HTTP/1.1" => {
            // simulate a long request
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // fix this when filename is not afile...
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
