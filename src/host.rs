use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

const PORT: u16 = 7878;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Клиент {} отключился.", stream.peer_addr().unwrap());
                break;
            }
            Ok(_) => {
                print!("{}", buffer[0] as char);
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
    let address = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(&address)?;
    println!("Сервер запущен на {}", address);
    println!("Ожидание подключений...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Новое подключение от {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Ошибка при подключении: {}", e),
        }
    }

    Ok(())
}