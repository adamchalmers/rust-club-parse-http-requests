use std::collections::HashMap;
use std::io::BufRead;
use std::io::{BufReader, Read};

fn main() {
    let file = match std::fs::File::open("request.txt") {
        Ok(s) => s,
        Err(e) => terminate(&e.to_string()),
    };
    let mut file = BufReader::new(file);
    let mut buffer = String::new();
    match file.read_line(&mut buffer) {
        Ok(_num_bytes) => {}
        Err(e) => terminate(&e.to_string()),
    };
    println!("{buffer}");

    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        buffer.clear();
        match file.read_line(&mut buffer) {
            Ok(_num_bytes) => {}
            Err(e) => terminate(&e.to_string()),
        };
        let buffer = buffer.trim();
        if buffer.is_empty() {
            break;
        }
        let (k, v) = buffer.split_once(": ").unwrap();
        headers.insert(k.to_owned(), v.to_owned());
    }
    println!("{headers:#?}");
    let num_bytes = headers.get("Content-Length").unwrap();
    let num_bytes: usize = num_bytes.parse().unwrap();
    let mut body_buffer = vec![0; num_bytes];
    file.read_exact(&mut body_buffer).unwrap();
    println!("Body:");
    let body_string = String::from_utf8(body_buffer);
    // let body_string = body_string.unwrap();
    let body_string = match body_string {
        Ok(x) => x,
        Err(e) => {
            panic!("{e}");
        }
    };
    println!("{body_string}");
}

fn terminate(s: &str) -> ! {
    eprintln!("Error: {s}");
    std::process::exit(1);
}
