use std::sync::{Mutex, Condvar};

pub struct Stream {
    buffer: Mutex<String>,
    condvar: Condvar,
}

impl Stream {
    pub fn new() -> Self {
        Stream { buffer: Mutex::new(String::new()), condvar: Condvar::new() }
    }

    pub fn write(&self, s: &str) {
        let mut buf = self.buffer.lock().unwrap();

        buf.push_str(s);
        self.condvar.notify_one();
    }

    pub fn read(&self) -> String {
        let mut buf = self.buffer.lock().unwrap();
        let temp = buf.clone();

        buf.clear();

        temp
    }
}
