use std::env;
use std::io;
use std::io::ErrorKind;
use std::net::{SocketAddrV4,Ipv4Addr};
use std::thread;

mod server;
mod config;
mod journal;

mod foo {
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Instant,Duration};

    pub struct AsyncTimer<T: 'static + Ord + Copy + Send> {
        join_handle: Option<thread::JoinHandle<()>>,
        tx: mpsc::Sender<TimerSignal<T>>,
    }

    enum TimerSignal<T> {
        Stop,
        Clear,
        Recurring(Duration, T),
        Once(Instant, T),
    }

    impl<T: 'static + Ord + Copy + Send> AsyncTimer<T> {
        pub fn new(emit: mpsc::Sender<T>) -> Self {
            /* To give to the thread */
            let emit = emit.clone();
            let (tx, rx) = mpsc::channel();
            let jh = thread::spawn(move || -> () {
                timer_loop(emit, rx);
            });
            AsyncTimer {
                join_handle: Some(jh),
                tx: tx,
            }
        }

        pub fn at(&mut self, data: T, at: Instant) {
            self.tx.send(TimerSignal::Once(at, data));
        }

        pub fn for_duration(&mut self, data: T, in_dur: Duration) {
            let at = Instant::now() + in_dur;
            self.tx.send(TimerSignal::Once(at, data));
        }

        pub fn recurring(&mut self, data: T, period: Duration) {
            self.tx.send(TimerSignal::Recurring(period, data));
        }

        pub fn clear(&mut self) {
            self.tx.send(TimerSignal::Clear);
        }

        pub fn stop(&mut self) {
            self.tx.send(TimerSignal::Stop);
        }
    }

    impl<T: Ord + Copy + Send> Drop for AsyncTimer<T> {
        fn drop(&mut self) {
            self.stop();
            if let Some(join_handle) = self.join_handle.take() {
                join_handle.join();
            }
        }
    }


    fn timer_loop<T: Ord + Copy + Send>(emit: mpsc::Sender<T>, signal: mpsc::Receiver<TimerSignal<T>>) {
        use std::collections::binary_heap::{BinaryHeap};
        let mut event_queue: BinaryHeap<(Instant, Duration, T)> = BinaryHeap::new();
        const IMMEDIATE: Duration = Duration::from_secs(0);
        loop {
            let now = Instant::now();
            match event_queue.peek() {
                None => {
                    match signal.recv() {
                        Ok(TimerSignal::Stop) => break,
                        Ok(TimerSignal::Clear) => { event_queue.clear(); },
                        Ok(TimerSignal::Recurring(duration, data)) => { event_queue.push((now + duration, duration, data)); },
                        Ok(TimerSignal::Once(instant, data)) => { event_queue.push((instant, IMMEDIATE, data)); },
                        Err(e) => { panic!("Timer signal channel closed! {:#?}", e); },
                    }
                },
                Some(&(instant, recurrence, data)) => {
                    eprintln!("instants: {:#?} vs {:#?}", now, instant);
                    if now > instant {
                        emit.send(data);
                        event_queue.pop();
                        continue;
                    }
                    match signal.recv_timeout(instant - now) {
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            emit.send(data);
                            event_queue.pop();
                            if recurrence != IMMEDIATE {
                                event_queue.push((instant + recurrence, recurrence, data));
                            }
                        }
                        Ok(TimerSignal::Stop) => break,
                        Ok(TimerSignal::Clear) => { event_queue.clear(); },
                        Ok(TimerSignal::Recurring(duration, data)) => { event_queue.push((now + duration, duration, data)); },
                        Ok(TimerSignal::Once(instant, data)) => { event_queue.push((instant, IMMEDIATE, data)); },
                        Err(e) => { panic!("some badness happened! {:#?}", e); },
                    }
                }
            }
        }
    }
}

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
    if args.len() == 1 {
        Ok(RunMode::NoOp)
    } else {
        match args[1].as_ref() {
            "loop" => Ok(RunMode::Loop),
            "local" => Ok(RunMode::LocalCluster),
            "gen_config" => {
                if args[1] == "gen_config" {
                    Ok(RunMode::GenConfig(args[2].clone()))
                } else {
                    print_usage();
                    Err(io::Error::new(ErrorKind::InvalidInput, "must be a config and file",))
                }
            }
            cmd => Err(io::Error::new(ErrorKind::InvalidInput, format!("command `{}` not understood", cmd))),
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let run_mode = parse_args(args)?;

    eprintln!("run_mode {:?}", run_mode);
    let x = 
    match run_mode {
        RunMode::GenConfig(filename) => {
            println!("lol: {}", filename);
            let cfg = config::with_name(&filename);
            let mut file = std::fs::File::create(filename)?;
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
                        eprintln!("<{:?}> Hello, listening on {}", thread::current().id(), saddr);
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
    };
    /* Timer testing */
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut timer = foo::AsyncTimer::new(sender.clone());
    timer.for_duration(3, std::time::Duration::from_secs(3));
    let pre = std::time::Instant::now();
    receiver.recv();
    let post = std::time::Instant::now();
    eprintln!("timer waited {:#?}", post - pre);
    x
}
