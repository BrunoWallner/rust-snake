use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};

pub struct Canvas {
    pub size: [u32; 2],
    pub buffer: Vec<Vec<i32>>
}

pub struct Shape {
    pub size: [u32; 2],
    pub position: [f32; 2],
}

impl Canvas {
    pub fn new(x_size: u32, y_size: u32) -> Self {
        Canvas {size: [x_size, y_size], buffer: vec![vec![]]}
    }
    pub fn resize(&mut self) {
        if let Some((Width(w), Height(h))) = terminal_size() {
            if (self.size[0] != (w / 2) as u32) || (self.size[1] != (h -1) as u32) {
                //deletes framebuffer
                for _x in 0..self.size[0] as usize{
                    self.buffer.pop();
                }
                //updates canvas size
                self.size = [(w / 2) as u32, (h - 1) as u32];
                //rebuilds framebuffer
                for x in 0..self.size[0] as usize{
                    self.buffer.push(Vec::new());
                    for _y in 0..self.size[1] as usize {
                        self.buffer[x].push(0);
                    }
                }
            }
        }
    }
    pub fn draw(&mut self, shape: &Shape) {
        for y in 1..self.size[1] as usize - 1 {
            for x in 1..self.size[0] as usize - 1 {
                if x as u32 == shape.position[0] as u32 && y as u32 == shape.position[1] as u32 {
                    self.buffer[x][y] = 1;
                }
            }
        }
    }
    pub fn draw_border(&mut self) {
        for y in 0..self.size[1] as usize {
            for x in 0..self.size[0] as usize {
                if x as u32 == 0 || x as u32 == self.size[0] - 1 {
                    self.buffer[x][y] = 2;
                }
                if y as u32 == 0 || y as u32 == self.size[1] - 1 {
                    self.buffer[x][y] = 2;
                }
            }
        }
    }
    pub fn update(&self) {
        for y in 0..self.size[1] as usize {
            for x in 0..self.size[0] as usize {
                match self.buffer[x][y] {
                    0 => print!("  "),
                    1 => print!("{}", "██"),
                    2 => print!("{}", "▒▒"),
                    3 => print!("{}", "░░"),
                    _ => print!("{}", "??")
                }
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }
    }
    pub fn clear(&mut self) {
        for y in 0..self.size[1] as usize {
            for x in 0..self.size[0] as usize {
                self.buffer[x][y] = 0;
            }
        }
        print!("\x1B[2J\x1B[1;1H"); //not working on windows cmd because its trash and doesnt support ansi escape sequences
        io::stdout().flush().unwrap();
    }
}

impl Shape {
    pub fn new(x_pos: u32, y_pos: u32, x_size: u32, y_size: u32) -> Self {
        Shape {position: [x_pos as f32, y_pos as f32], size: [x_size, y_size]}
    }
    pub fn shift(&mut self, x_direction: f32, y_direction: f32) {
        self.position[0] += x_direction;
        self.position[1] += y_direction;
    }
}