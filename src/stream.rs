use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};

#[derive(Debug)]
pub enum Event {
    Error,
    Finished,
    NeedsInput,
    Output(String),
}

#[derive(Default)]
struct StreamState {
    buffer: String,
    events: VecDeque<Event>,
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
        let mut state = self.state.lock().unwrap();

        state.events.push_back(Event::Finished);
        self.condvar.notify_one();
    }

    pub fn read_input(&self) -> String {
        let mut state = self.state.lock().unwrap();
        state.events.push_back(Event::NeedsInput);
        self.condvar.notify_one();
        state = self.condvar.wait(state).unwrap();

        let temp = state.buffer.clone();
        state.buffer.clear();

        temp
    }

    pub fn write_input(&self, s: &str) {
        let mut state = self.state.lock().unwrap();

        state.buffer = String::from(s);
        self.condvar.notify_one();
    }

    pub fn write_output(&self, s: &str) {
        let mut state = self.state.lock().unwrap();

        state.events.push_back(Event::Output(String::from(s)));
        self.condvar.notify_one();
    }

    pub fn get_event(&self) -> Option<Event> {
        let mut state = self.state.lock().unwrap();
        let event = state.events.pop_front();

        if event.is_some() {
            return event;
        }

        state = self.condvar.wait(state).unwrap();
        state.events.pop_front()
    }
}
