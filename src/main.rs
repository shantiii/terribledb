
use std::env;
use std::io;
use std::io::ErrorKind;

mod config;

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
                Err(io::Error::new(ErrorKind::InvalidInput, "incorrect number of arguments"))
            }
        },
        3 => {
            if args[1] == "gen_config" {
                Ok(RunMode::GenConfig(args[2].clone()))
            } else {
                eprintln!("error: must be a config and file");
                print_usage();
                Err(io::Error::new(ErrorKind::InvalidInput, "must be a config and file"))
            }
        },
        _ => {
            eprintln!("error: incorrect number of arguments");
            print_usage();
            Err(io::Error::new(ErrorKind::InvalidInput, "incorrect number of arguments"))
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
        RunMode::Loop => main_loop(|input: &str| -> bool {input.trim() == "stahp"}),
        RunMode::NoOp => Ok(())
    }
}

struct LoopState {
    cfg: config::TerribleConfig,
    socket: std::net::UdpSocket,
}

fn init_loop_state() -> io::Result<LoopState> {
    use std::fs::File;
    use std::net::UdpSocket;
    use std::time::Duration;
    let mut file = File::open("okay.cfg")?;
    let mut cfg = config::load(&mut file)?;
    let mut socket = UdpSocket::bind("0.0.0.0:1234")?;
    socket.set_read_timeout(Some(Duration::from_secs(5))).expect("set_read_timeout failed");
    Ok(LoopState {
        cfg: cfg,
        socket: socket,
    })
}

fn main_loop (mut break_check: impl Fn(&str) -> bool) -> io::Result<()> {
    use std::io::{Error, ErrorKind};
    let mut loop_state = init_loop_state()?;
    let mut recv_buffer = [0u8; 4096];
    loop {
        match loop_state.socket.recv_from(&mut recv_buffer) {
            Ok((bytes_read, src_addr)) => {
                let read_data = &mut recv_buffer[..bytes_read];
                let mut input = String::from_utf8(read_data.to_vec()).expect("owned");
                //io::stdin().read_line(&mut input)?;
                eprintln!("recv {:?} from {:?}", &read_data, &src_addr);
                if break_check(&input) {
                    break;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                eprint!(".");
                continue;
            }
            Err(ref e) => {
                eprintln!("fucked up now: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!('o' as u8, 'o' as u8);
    }
}
