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
        Err(e) => panic!("couldn't open {}: {}", display,
                                                   e.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("couldn't read {}: {}", display,
                                                   e.description()),
        Ok(_) => {},
    }

    let lines: Vec<&str> = s.lines().collect::<Vec<&str>>();
    let mut cpu_data: Vec<f64> = Vec::with_capacity(5);

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        // I could use a regular expression on each line to at least ensure that all of the unwrap
        // calls are at least warrented, but I think I'll be fine
        if words.len() >= 8 && words[0].starts_with("cpu") {
            let n1 = words[1].parse::<f64>().unwrap();  // user:    normal processes in user mode
         // let n2 = words[2].parse::<f64>().unwrap();  // nice:    niced processes executing in user mode
            let n3 = words[3].parse::<f64>().unwrap();  // system:  processes executing in user mode
            let n4 = words[4].parse::<f64>().unwrap();  // idle:    twiddling thumbs
            let n5 = words[5].parse::<f64>().unwrap();  // iowait:  waiting for I/O to complete
            let n6 = words[6].parse::<f64>().unwrap();  // irq:     servicing interrupts
            let n7 = words[7].parse::<f64>().unwrap();  // softirq: servicing softirqs

            cpu_data.push( ( (n1+n3+n5+n6+n7) / (n1+n3+n4) ) * 100.0 );
        } else {
            break   // Once that if is false, we don't need to go any further
        }
    }

    for n in cpu_data {
        println!("{}", (((n * 100.0) as i64) / 100) );
    }

    /*
    let mut word = s.split_whitespace();
    // let data = word.collect::<Vec<&str>>();

    word.next();    // Skip 'cpu'
    let n1 = word.next().unwrap().parse::<f64>().unwrap();
    word.next();    // Skip this value, it's not needed
    let n2 = word.next().unwrap().parse::<f64>().unwrap();
    let n3 = word.next().unwrap().parse::<f64>().unwrap();

    println!("{}", {(n1+n2)/(n1+n2+n3)*100.0}.trunc());
    */
}
