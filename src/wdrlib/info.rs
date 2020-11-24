pub enum State {
    Init,
    Downloading,
    Ready,
    Running,
    Stopped,
}

pub struct ProcessInfo {
    pub name: String,
    pub version: String,
    pub state: State,
}
