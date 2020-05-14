use super::Command;

pub struct List<'a> {
    month: u32,
    year: i32,
    category: Option<&'a str>,
}

impl List<'_> {
    pub fn new(month: u32, year: i32, category: Option<&str>) -> List {
        List {
            month,
            year,
            category,
        }
    }
}

impl Command for List<'_> {
    fn run(&self) {
        println!(
            "List operations for year {:?} and month {:?} and category {:?}",
            self.year, self.month, self.category
        );
    }
}
