mod engine;
use engine::Canvas;
use engine::Shape;

mod functions;
use functions::sleep;

fn main() {
    let mut canvas = Canvas {
        size: [32, 32],
        buffer: vec![0]
    };

    let mut shape = Shape {
        size: [2, 2],
        position: [2, 2]
    };

    canvas.Init();

    'running: loop {
        canvas.Clear();

        canvas.Draw(&shape);
        shape.Move([0, 1]);

        canvas.Update();

        sleep(1000/5);
    }
}