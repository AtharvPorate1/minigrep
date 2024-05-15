use std::env::args;


fn main() {

    let filename = args().nth(1).expect("Please provide a filename");
    let path = args().nth(2).expect("Please provide a path");
    
    let args = Cli{
        filename,
        path: std::path::PathBuf::from(path),
    };
    
    println!("filename: {:?}, path: {:?}", args.filename, args.path);
}

struct Cli{
    filename: String,
    path: std::path::PathBuf,
}
