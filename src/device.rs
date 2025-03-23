use std::net::TcpStream;
use std::io::Write;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use crossterm::terminal;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Подключен к серверу. Нажимайте клавиши... (ESC для выхода)");

    // Включаем raw mode (небуферизованный ввод)
    terminal::enable_raw_mode().expect("Не удалось включить raw mode");

    loop {
        if let Event::Key(key_event) = event::read()? {
            // Отфильтровываем только нажатия (KeyDown)
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

                stream.write_all(key_str.as_bytes())?;
                stream.flush()?;
            }
        }
    }

    // Выключаем raw mode перед выходом
    terminal::disable_raw_mode().expect("Не удалось отключить raw mode");

    Ok(())
}