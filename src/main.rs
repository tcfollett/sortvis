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

        self.min_index = self.right;
        for i in self.right..self.arr.len() {
            if self.arr[i] < self.arr[self.min_index] {
                self.min_index = i;
            }
        }
        self.compared = (self.min_index, self.right);
        self.arr.swap(self.right, self.min_index);
        self.right += 1;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let n = self.arr.len();
        let x_step = WIDTH as f32 / n as f32;

        for (i, &v) in self.arr.iter().enumerate() {
            let height_rect = v * HEIGHT as f32 * 0.9;
            let x = i as f32 * x_step;
            let y = HEIGHT as f32 - height_rect;

            let color = if self.done() {
                Color::GREEN
            } else if i == self.compared.0 || i == self.compared.1 {
                Color::RED
            } else {
                Color::WHITE
            };

            d.draw_rectangle(
                x as i32,
                y as i32,
                x_step.ceil() as i32,
                height_rect as i32,
                color,
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Window").build();

    rl.set_target_fps(60);

    let mut viz = SortingVisualizer::new(MAX_AMOUNT);

    while !rl.window_should_close() {
        viz.step();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        viz.draw(&mut d);
    }
}
