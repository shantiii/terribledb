use std::env;
use std::io;
use std::io::ErrorKind;
use std::net::{SocketAddrV4,Ipv4Addr};
use std::thread;

mod server;
mod config;
mod journal;
mod mem_journal;
mod kv_state;

fn print_usage() {
    eprintln!("usage: terribledb <node_name>");
}

#[derive(Debug)]
enum RunMode {
    GenConfig(String),
    Loop,
    NoOp,
    LocalCluster,
}

fn parse_args(args: Vec<String>) -> io::Result<RunMode> {
    match args.len() {
        1 => Ok(RunMode::NoOp),
        2 => {
            match args[1].as_ref() {
                "loop" => Ok(RunMode::Loop),
                "local" => Ok(RunMode::LocalCluster),
                _ => Err(io::Error::new(ErrorKind::InvalidInput, "incorrect number of arguments"))
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
        RunMode::Loop => server::main_loop(None, None, |input: &str| -> bool { input.trim() == "stahp" }),
        RunMode::LocalCluster => {
            let ports: Vec<u16> = vec![55550, 55551, 55552];
            let cluster: Vec<SocketAddrV4> = ports.into_iter().map(|port| { SocketAddrV4::new(Ipv4Addr::LOCALHOST, port) }).collect();
            let mut join_handles = vec![];
            for saddr in cluster.clone() {
                let name = format!("server [{}]", saddr);
                let cluster = cluster.clone();
                join_handles.push(
                thread::Builder::new()
                    .name(name)
                    .spawn(move || {
                        let bluster = cluster.clone();
                        println!("<{:?}> Hello, listening on {}", thread::current().id(), saddr);
                        server::main_loop(Some(saddr), Some(bluster), |input: &str| -> bool { input.trim() == "stahp" });
                    }).unwrap()
                    );
            };
            for join_handle in join_handles {
                join_handle.join();
            }
            Ok(())
        }
        RunMode::NoOp => Ok(()),
    }
}
