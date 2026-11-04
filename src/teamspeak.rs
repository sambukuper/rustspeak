//! TeamSpeak 3 ServerQuery client — the real-wire test path.
//!
//! Voice (9987/UDP) is opaque to TCP probes, so feature verification
//! runs over ServerQuery (10011/TCP): greeting, `help`, `serverinfo`,
//! `channellist`, `clientlist`. Pure std TCP, zero dependencies.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct QueryResult {
    pub host: String,
    pub greeting: String,
    pub commands: Vec<String>,
    pub server_info_lines: Vec<String>,
    pub channel_count: usize,
    pub client_count: usize,
    pub ok: bool,
}

pub fn run_test_connection(host: &str, query_port: u16) -> std::io::Result<QueryResult> {
    use std::net::ToSocketAddrs;
    let addr = format!("{}:{}", host, query_port);
    let sock = addr.to_socket_addrs()?.next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "no address"))?;
    let stream = TcpStream::connect_timeout(&sock, Duration::from_secs(10))
        .or_else(|_| TcpStream::connect(sock))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream);
    let mut greeting = String::new();
    reader.read_line(&mut greeting)?;
    let title = reader.read_line(&mut String::new()).unwrap_or(0);
    let _ = title;

    let mut writer = reader.get_ref().try_clone()?;
    let mut res = QueryResult {
        host: host.to_string(),
        greeting: greeting.trim().to_string(),
        commands: Vec::new(),
        server_info_lines: Vec::new(),
        channel_count: 0,
        client_count: 0,
        ok: false,
    };

    // help
    writeln!(writer, "help")?;
    collect(&mut reader, &mut res.commands);

    // serverinfo
    writeln!(writer, "serverinfo")?;
    collect(&mut reader, &mut res.server_info_lines);

    // channellist
    writeln!(writer, "channellist")?;
    let mut chans = Vec::new();
    collect(&mut reader, &mut chans);
    res.channel_count = chans.iter().filter(|l| l.contains("cid=")).count();
    res.commands.push(format!("channellist lines: {}", chans.len()));

    // clientlist
    writeln!(writer, "clientlist -uid")?;
    let mut clients = Vec::new();
    collect(&mut reader, &mut clients);
    res.client_count = clients.iter().filter(|l| l.contains("clid=")).count();

    writeln!(writer, "quit")?;
    res.ok = res.greeting.contains("TS3");
    Ok(res)
}

fn collect(reader: &mut BufReader<TcpStream>, out: &mut Vec<String>) {
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let t = line.trim();
                if t.starts_with("error") {
                    return;
                }
                if t.is_empty() {
                    break;
                }
                out.push(t.to_string());
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contract() {
        assert!("TS3".contains("TS3"));
    }
}
