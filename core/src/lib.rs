// Buffalo Core Vietnamese Typing Engine Library

pub enum Mode {
    Vietnamese,
    English,
    Punctuation,
}

pub enum InputMethod {
    Telex,
    Vni,
    Viqr,
}

pub struct Config {
    pub mode: Mode,
    pub method: InputMethod,
    pub std_tone: bool,
    pub free_marking: bool,
}

pub struct Engine {
    config: Config,
    buffer: String,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            buffer: String::new(),
        }
    }

    pub fn process_key(&mut self, key: char) -> String {
        // Placeholder for processing logic
        self.buffer.push(key);
        self.buffer.clone()
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
    }
}
