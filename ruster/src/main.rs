use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::Duration;
use std::{sync, thread};

const PORTS: std::ops::Range<u16> = 4000..4020;
const HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

fn get_addr() -> io::Result<SocketAddr> {
    let ss = Command::new("ss")
        .arg("-tln")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut buf = String::new();
    let _ = ss.stdout.expect("piped").read_to_string(&mut buf)?;

    let mut addrs = buf.lines()
        .filter_map(|e| {
            e.starts_with("LISTEN").then_some(())?;

            let addr = e.split_whitespace().nth(3)
                .map(str::parse::<SocketAddr>)?.ok()?;

            (addr.ip() == HOST && PORTS.contains(&addr.port())).then_some(addr)
        })
        .collect::<Vec<_>>();

    if addrs.is_empty() {
        return Ok(SocketAddr::new(HOST, 4000));
    }

    addrs.sort_by(|a,b|a.port().partial_cmp(&b.port()).expect("port cannot be NaN"));

    let mut ports = PORTS.clone();

    for (addr, port) in addrs.into_iter().zip(&mut ports) {
        if port != addr.port() {
            return Ok(SocketAddr::new(HOST, port));
        }
    }

    match ports.next() {
        Some(port) => Ok(SocketAddr::new(HOST, port)),
        None => Err(io::Error::new(io::ErrorKind::AddrInUse, "No Port Available")),
    }
}

fn main() -> io::Result<()> {
    let tcp = TcpListener::bind(dbg!(get_addr()?))?;
    let (tx,_) = sync::mpsc::sync_channel::<String>(0);
    let pool = 0u8;

    println!("[Ruster] Listening {}",tcp.local_addr()?);

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
            println!("[TODO] Busy");
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

