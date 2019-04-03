
use std::env;
use std::io;
use std::io::ErrorKind;

fn lol() -> u8 {
    'o' as u8
}

fn print_usage() {
    eprintln!("usage: terribledb <node_name>");
}

fn parse_args(args: Vec<String>) -> io::Result<String> {
    match args.len() {
        2 => {
            Ok(args[1].clone())
        },
        _ => {
            eprintln!("error: incorrect number of arguments");
            print_usage();
            Err(io::Error::new(ErrorKind::InvalidInput, "incorrect number of arguments"))
        }
    }
}

struct TerribleConfig {
    name: String
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let result = parse_args(args)?;

    main_loop(|input: &str| -> bool {input.trim() == "stahp"})
}

fn main_loop<F> (mut break_check: F) -> io::Result<()>
where F: FnMut(&str) -> bool {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        println!("You typed: {}", input.trim());
        if break_check(&input) {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(lol(), 'o' as u8);
    }
}
