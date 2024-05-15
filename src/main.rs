use std::env::args;
use std::fs;


fn main() {

    let search = args().nth(1).expect("Please provide a filename");
    let path = args().nth(2).expect("Please provide a path");
    
    let args = Cli{
        search,
        path: std::path::PathBuf::from(path),
    };
    
    println!("filename: {:?}, path: {:?}", args.search, args.path);

    let content = fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines(){
        if line.contains(&args.search){
            println!("{}", line);
        }
    
    }



}

struct Cli{
    search: String,
    path: std::path::PathBuf,
}
