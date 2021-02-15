mod engine;
use engine::Canvas;
use engine::Shape;

mod functions;
use functions::sleep;

fn main() {
    let mut canvas = Canvas {size: [16, 8], buffer: vec![vec![0]]};

    let mut shape = Shape {size: [1, 1], position: [3, 1]};

    canvas.Init();

    'running: loop {
        canvas.Clear();

        canvas.Draw(&shape);
        shape.Move([1, 0]);

        //debug
        println!("{} : {}", shape.position[0], shape.position[1]);

        canvas.Update();

        sleep(1000/1);
    }
}