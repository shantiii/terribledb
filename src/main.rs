use std::env;
use std::io;
use std::io::ErrorKind;

mod server;
mod config;
mod journal;

fn print_usage() {
    eprintln!("usage: terribledb <node_name>");
}

#[derive(Debug)]
enum RunMode {
    GenConfig(String),
    Loop,
    NoOp,
}

fn parse_args(args: Vec<String>) -> io::Result<RunMode> {
    match args.len() {
        1 => Ok(RunMode::NoOp),
        2 => {
            if args[1] == "loop" {
                Ok(RunMode::Loop)
            } else {
                eprintln!("error: incorrect number of arguments");
                print_usage();
                Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "incorrect number of arguments",
                ))
            }
        }
        3 => {
            if args[1] == "gen_config" {
                Ok(RunMode::GenConfig(args[2].clone()))
            } else {
                eprintln!("error: must be a config and file");
                print_usage();
                Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "must be a config and file",
                ))
            }
        }
        _ => {
            eprintln!("error: incorrect number of arguments");
            print_usage();
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "incorrect number of arguments",
            ))
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let run_mode = parse_args(args)?;

    eprintln!("run_mode {:?}", run_mode);
    match run_mode {
        RunMode::GenConfig(filename) => {
            println!("lol: {}", filename);
            let cfg = config::with_name(&filename);
            let mut file = std::fs::File::create("okay.cfg")?;
            config::save(&cfg, &mut file)?;
            Ok(())
        }
        RunMode::Loop => server::main_loop(|input: &str| -> bool { input.trim() == "stahp" }),
        RunMode::NoOp => Ok(()),
    }
}
