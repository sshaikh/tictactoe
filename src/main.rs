fn main() {
    println!("Hello, world!");
}


#[derive(Clone, Copy)]
enum Counter {
    X,
    O,
}

struct Board {
    width: usize,
    height: usize,
    state: [[Counter; 3]; 3]
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> Option<Counter> {
        Some(*self.state.get(row)?.get(col)?)
    }
}

