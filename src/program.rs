use crate::ast::{Line, Statement};

pub const MAX_LINES: usize = 8 * 1024;

pub struct Program {
    pub lines: [Option<Line>; MAX_LINES],
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: [const { None }; MAX_LINES],
        }
    }

    pub fn set(&mut self, line: Line) {
        let line_number = line.number.unwrap();
        self.lines[line_number] = Some(line);
    }

    pub fn get(&self, number: usize) -> Option<Line> {
        self.lines.get(number).and_then(|line| line.clone())
    }

    pub fn clear(&mut self) {
        self.lines = [const { None }; MAX_LINES];
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Line> + 'a {
        self.lines
            .iter()
            .filter_map(|line| line.as_ref())
            .map(|line| line.clone())
    }

    pub fn print(&self) -> String {
        let mut output: Vec<String> = vec![];
        for line in self.iter() {
            output.push(format!("{}", line.source.trim()));
        }

        output.join("\n")
    }
}
