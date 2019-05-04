fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: fta <name of inputfile>");
        return;
    }

    for s in args {
        println!("{}", s);
    }
}
