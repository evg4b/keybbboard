use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Клиент отключился.");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                print!("{}", received);
                std::io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("Ошибка чтения: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Сервер слушает на 127.0.0.1:7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Новое подключение: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Ошибка при подключении: {}", e),
        }
    }

    Ok(())
}