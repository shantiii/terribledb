use std::io;
use crate::config;

struct LoopState {
    cfg: config::TerribleConfig,
    socket: std::net::UdpSocket,
    counter: u64,
}

fn init_loop_state() -> io::Result<LoopState> {
    use std::fs::File;
    use std::net::UdpSocket;
    use std::time::Duration;
    let cfg = File::open("okay.cfg").and_then(|mut f| config::load(&mut f))?;
    let socket = UdpSocket::bind("0.0.0.0:1234")?;
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set_read_timeout failed");
    Ok(LoopState {
        cfg: cfg,
        socket: socket,
        counter: 0,
    })
}

pub fn main_loop(break_check: impl Fn(&str) -> bool) -> io::Result<()> {
    let mut loop_state = init_loop_state()?;
    let mut recv_buffer = [0u8; 4096];
    loop {
        match loop_state.socket.recv_from(&mut recv_buffer) {
            Ok((bytes_read, src_addr)) => {
                let read_data = &mut recv_buffer[..bytes_read];
                let input = String::from_utf8(read_data.to_vec())
                    .expect("received data is not valid utf-8");
                eprintln!("recv {:?} from {:?}", &read_data, &src_addr);
                if break_check(&input) {
                    eprintln!("break recv'd");
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                eprint!(".");
            }
            Err(ref e) => {
                eprintln!("fucked up now: {:?}", e);
                break;
            }
        }
        loop_state.counter += 1;
    }
    Ok(())
}
