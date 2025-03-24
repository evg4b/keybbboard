mod protocol;

use std::net::TcpStream;
use std::io::{stdin, Read, Write};
use std::env;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use crossterm::terminal;
use crate::protocol::ServerResponse;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: client <IP-адрес сервера>");
        return Ok(());
    }

    let server_addr = format!("{}:7878", args[1]);
    let mut stream = TcpStream::connect(&server_addr)?;
    println!("Подключен к серверу {}. Нажимайте клавиши... (ESC для выхода)", server_addr);

    terminal::enable_raw_mode().expect("Не удалось включить raw mode");

    let mut buffer: Vec<u8> = vec![0u8; 64 * 1024]; // 64 KB buffer initialized with 0s

    let response = stream.read(&mut buffer);
    match response {
        Err(e) => {
            eprintln!("Failed to receive data: {}", e);
        }
        Ok(n) => {
            let item: ServerResponse = bitcode::decode(&mut buffer).unwrap();
            match item {
                ServerResponse::ConnectionError(msg) => {
                    panic!("Connection error: {}", msg);
                }
                _ => {}
            }
        },
    }

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                let key_str = match key_event.code {
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Enter => "\n".to_string(),
                    KeyCode::Tab => "[TAB]".to_string(),
                    KeyCode::Backspace => "[BACKSPACE]".to_string(),
                    KeyCode::Esc => {
                        println!("\nОтключение...");
                        break;
                    }
                    _ => continue,
                };
                let bytes = key_str.as_bytes();

                stream.write_all(bytes)?;
                stream.flush()?;
            }
        }
    }

    terminal::disable_raw_mode().expect("Не удалось отключить raw mode");

    Ok(())
}