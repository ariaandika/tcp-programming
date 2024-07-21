use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::Duration;
use std::{sync, thread};

fn get_addr() -> io::Result<String> {
    let ss = Command::new("ss")
        .arg("-tln")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut buf = String::new();
    let _ = ss.stdout.expect("piped").read_to_string(&mut buf)?;

    let bound_ports = buf
        .lines()
        .filter(|e| e.starts_with("LISTEN"))
        .filter_map(|e| {
            e.split_whitespace()
                .nth(3)?
                .get("127.0.0.1:".len()..)?
                .parse::<u64>()
                .ok()
        }).collect::<HashSet<_>>();

    let port = (4000u64..4020)
        .find(|e|!bound_ports.contains(e))
        .ok_or(io::Error::new(io::ErrorKind::AddrInUse, "No Port Avilable"))?;

    Ok(format!("127.0.0.1:{port}"))
}

fn main() -> io::Result<()> {
    let tcp = TcpListener::bind(get_addr()?)?;
    let (tx,_) = sync::mpsc::sync_channel::<String>(0);
    let pool = 0u8;

    println!("Listening {}",tcp.local_addr()?);

    for stream in tcp.incoming() {
        let stream = match stream {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("TcpError: {err}");
                continue;
            },
        };

        if pool < 16 {
            let tx = tx.clone();
            thread::spawn(||handle(stream, tx));
        } else {
            println!("Busy");
        }
    }

    Ok(())
}

fn handle(mut stream: TcpStream, _: SyncSender<String>) {
    let mut reader = BufReader::new(&mut stream);
    let mut buf = Vec::new();

    if let Err(err) = reader.read_until(b';', &mut buf) {
        eprintln!("{err}");
    };

    println!("{}", String::from_utf8_lossy(&buf));

    sleep(Duration::from_secs(1));

    if let Err(err) = stream.write(b"Ok") {
        eprintln!("{err}");
    }

    // stream disconnected on `drop`
}

