use std::env::args;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


fn main() {

    let search = args().nth(1).expect("Please provide a filename");
    let path = args().nth(2).expect("Please provide a path");
    
    let args = Cli{
        search,
        path: std::path::PathBuf::from(path),
    };
    
    println!("filename: {:?}, path: {:?}", args.search, args.path);

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);
    

    for line in reader.lines() {
        // Check if the line contains the search term
        let line = line.expect("could not read line");
        if line.contains(&args.search) {
            println!("{}", line);
        }
    }



}

struct Cli{
    search: String,
    path: std::path::PathBuf,
}
