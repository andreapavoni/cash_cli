use super::Command;

pub struct Export {
    src: String,
}

impl Export {
    pub fn new(src: String) -> Export {
        Export { src }
    }
}

impl Command for Export {
    fn run(&self) {
        println!("Export to file {:?}", self.src);
    }
}
