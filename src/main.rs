mod engine;
use engine::Canvas;
use engine::Shape;

mod functions;
use functions::sleep;

fn main() {
    let mut canvas = Canvas::new(64, 48);

    let mut shape = Shape::new(0, 0, 1, 1);

    canvas.init();

    loop {
        canvas.clear();
        canvas.draw_border();

        shape.Move(1, 1);
        canvas.draw(&shape);

        canvas.update();

        sleep(1000/15);
    }
}