mod engine;
use engine::Canvas;
use engine::Shape;

mod functions;
use functions::sleep;

fn main() {
    let mut canvas = Canvas::new(64, 48);

    let mut shape = Shape::new(1, 1, 1, 1);

    canvas.resize();

    let fps: u32 = 15;
    loop {
        canvas.resize();

        canvas.clear();
        canvas.draw_border();

        shape.shift(0.1, 0.1);
        canvas.draw(&shape);

        canvas.update();

        sleep(1000/fps as u64);
    }
}