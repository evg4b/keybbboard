mod protocol;

use crate::protocol::{KeyboardEvent, ServerResponse};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use uinput::event::keyboard;

const PORT: u16 = 7878;

fn handle_client(mut stream: TcpStream) {
    let mut device = uinput::default()
        .unwrap()
        .name("test")
        .unwrap()
        .event(uinput::event::Keyboard::All)
        .unwrap()
        .create()
        .unwrap();

    let mut buffer: Vec<u8> = vec![0u8; 64 * 1024]; // 64 KB buffer initialized with 0s
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Клиент {} отключился.", stream.peer_addr().unwrap());
                break;
            }
            Ok(size) => {
                let event = bitcode::decode(&mut buffer[..size]);
                match event {
                    Ok(event) => {
                        device.click(&map_key(event)).unwrap();
                        std::io::stdout().flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Ошибка декодирования: {}", e);
                    }
                }
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

fn map_key(event: KeyboardEvent) -> keyboard::Key {
    match event {
        KeyboardEvent::KeyQ => keyboard::Key::Q,
        KeyboardEvent::KeyW => keyboard::Key::W,
        KeyboardEvent::KeyE => keyboard::Key::E,
        KeyboardEvent::KeyR => keyboard::Key::R,
        KeyboardEvent::KeyT => keyboard::Key::T,
        KeyboardEvent::KeyY => keyboard::Key::Y,
        KeyboardEvent::KeyU => keyboard::Key::U,
        KeyboardEvent::KeyI => keyboard::Key::I,
        KeyboardEvent::KeyO => keyboard::Key::O,
        KeyboardEvent::KeyP => keyboard::Key::P,
        KeyboardEvent::KeyA => keyboard::Key::A,
        KeyboardEvent::KeyS => keyboard::Key::S,
        KeyboardEvent::KeyD => keyboard::Key::D,
        KeyboardEvent::KeyF => keyboard::Key::F,
        KeyboardEvent::KeyG => keyboard::Key::G,
        KeyboardEvent::KeyH => keyboard::Key::H,
        KeyboardEvent::KeyJ => keyboard::Key::J,
        KeyboardEvent::KeyK => keyboard::Key::K,
        KeyboardEvent::KeyL => keyboard::Key::L,
        KeyboardEvent::KeyZ => keyboard::Key::Z,
        KeyboardEvent::KeyX => keyboard::Key::X,
        KeyboardEvent::KeyC => keyboard::Key::C,
        KeyboardEvent::KeyV => keyboard::Key::V,
        KeyboardEvent::KeyB => keyboard::Key::B,
        KeyboardEvent::KeyN => keyboard::Key::N,
        KeyboardEvent::KeyM => keyboard::Key::M,
        KeyboardEvent::Space => keyboard::Key::Space,
        KeyboardEvent::Enter => keyboard::Key::Enter,
        KeyboardEvent::Escape => keyboard::Key::Esc,
        KeyboardEvent::Key1 => keyboard::Key::_1,
        KeyboardEvent::Key2 => keyboard::Key::_2,
        KeyboardEvent::Key3 => keyboard::Key::_3,
        KeyboardEvent::Key4 => keyboard::Key::_4,
        KeyboardEvent::Key5 => keyboard::Key::_5,
        KeyboardEvent::Key6 => keyboard::Key::_6,
        KeyboardEvent::Key7 => keyboard::Key::_7,
        KeyboardEvent::Key8 => keyboard::Key::_8,
        KeyboardEvent::Key9 => keyboard::Key::_9,
        KeyboardEvent::Key0 => keyboard::Key::_0,
        KeyboardEvent::Backspace => keyboard::Key::BackSpace,
        KeyboardEvent::Tab => keyboard::Key::Tab,
        KeyboardEvent::CapsLock => keyboard::Key::CapsLock,
        KeyboardEvent::F1 => keyboard::Key::F1,
        KeyboardEvent::F2 => keyboard::Key::F2,
        KeyboardEvent::F3 => keyboard::Key::F3,
        KeyboardEvent::F4 => keyboard::Key::F4,
        KeyboardEvent::F5 => keyboard::Key::F5,
        KeyboardEvent::F6 => keyboard::Key::F6,
        KeyboardEvent::F7 => keyboard::Key::F7,
        KeyboardEvent::F8 => keyboard::Key::F8,
        KeyboardEvent::F9 => keyboard::Key::F9,
        KeyboardEvent::F10 => keyboard::Key::F10,
        KeyboardEvent::F11 => keyboard::Key::F11,
        KeyboardEvent::F12 => keyboard::Key::F12,
    }
}
