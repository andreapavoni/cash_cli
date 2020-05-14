use super::Command;

pub struct Report<'a> {
    month: u32,
    year: i32,
    category: Option<&'a str>,
}

impl Report<'_> {
    pub fn new(month: u32, year: i32, category: Option<&str>) -> Report {
        Report {
            month,
            year,
            category,
        }
    }
}

impl Command for Report<'_> {
    fn run(&self) {
        println!(
            "Report operations for year {:?} and month {:?} and category {:?}",
            self.year, self.month, self.category
        );
    }
}
