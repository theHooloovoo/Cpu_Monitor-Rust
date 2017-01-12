// Parse /proc/stat for cpu usage info

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("/proc/stat");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {},
    }

    let mut word = s.split_whitespace();
    // let data = word.collect::<Vec<&str>>();

    word.next();    // Skip 'cpu'
    let n1 = word.next().unwrap().parse::<f64>().unwrap();
    word.next();    // Skip this value, it's not needed
    let n2 = word.next().unwrap().parse::<f64>().unwrap();
    let n3 = word.next().unwrap().parse::<f64>().unwrap();

    println!("{}", {(n1+n2)/(n1+n2+n3)*100.0}.trunc());
}
