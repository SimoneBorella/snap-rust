use global_hotkey::GlobalHotKeyManager;
use global_hotkey::hotkey::{HotKey, Modifiers, Code};


// static hotkey_manager: GlobalHotKeyManager = GlobalHotKeyManager::new().expect("Failed to initialize GlobalHotKeyManager");


pub struct Hotkey {
    pub label: String,
    pub modifier: String,
    pub tmp_modifier: String,
    pub code: String,
    pub tmp_code: String,
    pub registered_hotkey: HotKey
}

impl Hotkey {
    pub fn new(label: String, modifier: String, code: String) -> Self {
        let registered_hotkey = HotKey::new(string_to_modifiers(&modifier), string_to_key(&code).unwrap());
        Hotkey{
            label: label,
            modifier: modifier.clone(),
            tmp_modifier: modifier,
            code: code.clone(),
            tmp_code: code,
            registered_hotkey: registered_hotkey
        }
    }

    pub fn register(&mut self, manager: &GlobalHotKeyManager) {
        let registered_hotkey = HotKey::new(string_to_modifiers(&self.modifier), string_to_key(&self.code).unwrap());
        self.registered_hotkey = registered_hotkey.clone();
        if let Err(err) = manager.register(self.registered_hotkey) {
            eprintln!("Failed to register hotkey: {}", err);
        }
    }

    pub fn unregister(&mut self, manager: &GlobalHotKeyManager) {
        if let Err(err) = manager.unregister(self.registered_hotkey) {
            eprintln!("Failed to unregister hotkey: {}", err);
        }
    }
}


pub fn string_to_key(s: &str) -> Option<Code> {
    match s.to_uppercase().as_str() {
        "A" => Some(Code::KeyA),
        "B" => Some(Code::KeyB),
        "C" => Some(Code::KeyC),
        "D" => Some(Code::KeyD),
        "E" => Some(Code::KeyE),
        "F" => Some(Code::KeyF),
        "G" => Some(Code::KeyG),
        "H" => Some(Code::KeyH),
        "I" => Some(Code::KeyI),
        "J" => Some(Code::KeyJ),
        "K" => Some(Code::KeyK),
        "L" => Some(Code::KeyL),
        "M" => Some(Code::KeyM),
        "N" => Some(Code::KeyN),
        "O" => Some(Code::KeyO),
        "P" => Some(Code::KeyP),
        "Q" => Some(Code::KeyQ),
        "R" => Some(Code::KeyR),
        "S" => Some(Code::KeyS),
        "T" => Some(Code::KeyT),
        "U" => Some(Code::KeyU),
        "V" => Some(Code::KeyV),
        "W" => Some(Code::KeyW),
        "X" => Some(Code::KeyX),
        "Y" => Some(Code::KeyY),
        "Z" => Some(Code::KeyZ),
        _ => None,
    }
}

pub fn string_to_modifiers(s: &String) -> Option<Modifiers> {
    match s.as_str() {
        "ALT" => Some(Modifiers::ALT),
        "CTRL" => Some(Modifiers::CONTROL),
        "SHIFT" => Some(Modifiers::SHIFT),
        _ => {
            todo!("Handle unknown modifier: {}", s);
        }
    }
}