use rand::Rng;
use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const MAX_AMOUNT: usize = 100;

struct SortingVisualizer {
    arr: Vec<f32>,
    right: usize,
    min_index: usize,
    compared: (usize, usize),
}

impl SortingVisualizer {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let arr = (0..n).map(|_| rng.gen_range(0.0..1.0f32)).collect();

        Self {
            arr,
            right: 0,
            min_index: 0,
            compared: (0, 0),
        }
    }

    fn done(&self) -> bool {
        self.right >= self.arr.len()
    }

    fn step(&mut self) {
        if self.done() {
            return;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Window").build();

    rl.set_target_fps(60);
}
