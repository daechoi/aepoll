use std::{
    io::{ErrorKind, Read, Result, Write},
    net::TcpStream,
};

use ffi::Event;
use poll::Poll;

mod ffi;
mod poll;

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events: usize = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 2000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(&request)?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    println!("Sent all events!");
    let mut handled_events: usize = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        // wake up when the event has occured.
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        handled_events += handle_events(&events, &mut streams)?;
    }
    println!("Finished");
    Ok(())
}

fn get_req(path: &str) -> Vec<u8> {
    format!(
        "GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\r\n"
    )
    .into_bytes()
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(0) => {
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n---------\n");
                }
                // Not ready to send in a non-blocking manner. This could
                // happen even if the event was reported as ready
                Err(e)
                    if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::Interrupted =>
                {
                    break
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}
