use fta::Converter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: fta <name of inputfile>");
        return;
    }

    let maybecnv = Converter::new(args[1].clone());
    let cnv = match maybecnv {
        Ok(t) => t,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
    let result = cnv.make_header();
    match result {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
            ()
        }
    }
}
