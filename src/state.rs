use std::sync::Mutex;

pub static STATE: Mutex<State> = Mutex::new(State{
    DEBUG: false
});

pub struct State {
    pub DEBUG: bool
}
