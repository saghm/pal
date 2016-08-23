use std::sync::{Mutex, Condvar};

#[derive(Default)]
struct StreamState {
    buffer: String,
    finished: bool,
}

pub struct Stream {
    state: Mutex<StreamState>,
    condvar: Condvar,
}

impl Stream {
    pub fn new() -> Self {
        Stream { state: Mutex::new(StreamState::default()), condvar: Condvar::new() }
    }

    pub fn finished(&self) {
        self.state.lock().unwrap().finished = true;
        self.condvar.notify_one();
    }

    pub fn is_finished(&self) -> bool {
        self.state.lock().unwrap().finished
    }

    pub fn write(&self, s: &str) {
        let mut state = self.state.lock().unwrap();

        state.buffer.push_str(s);
        self.condvar.notify_one();
    }

    pub fn read(&self) -> String {
        let mut state = self.state.lock().unwrap();
        state = self.condvar.wait(state).unwrap();

        let temp = state.buffer.clone();
        state.buffer.clear();

        temp
    }
}
