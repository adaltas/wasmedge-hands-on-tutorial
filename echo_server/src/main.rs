use std::{
    env,
    io::{Read, Result, Write},
};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let host_address = format!("0.0.0.0:{}", &port);
    let listener = TcpListener::bind(&host_address, false).expect("Failed to bind on address");
    println!("Listening on {}", host_address);
    loop {
        let stream = listener.accept(0).unwrap().0;
        if let Err(error) = echo(stream) {
            println!("Error: {:#?}", error);
        };
    }
}

fn echo(mut stream: TcpStream) -> Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }
    println!("Received {} bytes", data.len());
    stream.write_all(&data)?;
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}
