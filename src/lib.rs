use std::{thread, time};

struct Board {
    data: String,
    next: Vec<u8>,
}

impl Board {
    fn build(size: usize) -> Board {
        Board {
            //data = a repeated call to "generate random bool", size times, then collect the result
            data: std::iter::repeat_with(rand::random::<bool>)
                .take(size)
                .map(|x| match x {
                    true => 'x',
                    false => ' ',
                })
                .collect(),
            next: Vec::with_capacity(size),
        }
    }

    fn show(&mut self) -> bool {
        println!("{}", self.data);
        self.data.contains('x')
    }

    fn update(&mut self) {
        for i in 0..self.data.len() {
            self.next.push(
                self.data
                    .chars()
                    .cycle()
                    .skip(i + self.data.len() - 2)
                    .take(5)
                    .filter(|&x| x == 'x')
                    .count() as u8, // ok, is in [0, 5]
            );
        }

        self.data = self
            .next
            .iter()
            .map(|x| match *x {
                2 | 4 => 'x',
                _ => ' ',
            })
            .collect();
        self.next.clear();
    }
}

pub fn run() {
    let mut board = Board::build(50);
    const DELAY: std::time::Duration = time::Duration::from_millis(50);

    while board.show() {
        thread::sleep(DELAY);
        board.update();
    }
}
