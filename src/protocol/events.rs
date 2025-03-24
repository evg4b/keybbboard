use bitcode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ServerResponse {
    Ok { version: u32 },
    ConnectionError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = ServerResponse::ConnectionError {
            message: "Connection error".to_string(),
        };

        let encoded = bitcode::encode(&data);
        let decoded = bitcode::decode(&encoded).unwrap();

        assert_eq!(data, decoded);
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum KeyboardEvent {
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Space,
    Enter,
    Escape,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Backspace,
    Tab,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}
