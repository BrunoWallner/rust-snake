mod engine;
use engine::Canvas;
use engine::Shape;

mod functions;
use functions::sleep;

fn main() {
    let mut canvas = Canvas::new(32, 32);

    let mut shape = Shape::new(1, 1, 1, 1);

    canvas.Init();

    loop {
        canvas.Clear();

        canvas.Draw(&shape);
        shape.Move([1, 1]);

        canvas.Update();

        sleep(1000/1);
    }
}