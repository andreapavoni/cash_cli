use super::Command;

pub struct Import {
    src: String,
}

impl Import {
    pub fn new(src: String) -> Import {
        Import { src }
    }
}

impl Command for Import {
    fn run(&self) {
        println!("Import from file {:?}", self.src);
    }
}
