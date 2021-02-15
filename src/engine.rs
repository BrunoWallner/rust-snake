use std::io::{self, Write};

pub struct Canvas {
    pub size: [i32; 2],
    pub buffer: Vec<i32>,
}

pub struct Shape {
    pub size: [u32; 2],
    pub position: [u32; 2]
}

impl Canvas {
    pub fn Init(&mut self) {
        for _x in 0..self.size[0] as usize * self.size[1] as usize {
            self.buffer.push(0);
        }
    }

    pub fn Draw(&mut self, shape: &Shape) {
        for y in 0..self.size[0] as usize {
            for x in 0..self.size[1] as usize {
                if shape.position[0] == x as u32 && shape.position[1] == y as u32 {
                    self.buffer[x + y] = 1;
                }
            }
        }
    }

    pub fn Update(&self) {
        for y in 0..self.size[0] as usize {
            for x in 0..self.size[1] as usize {
                print!("{}", self.buffer[x + y]);
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }
    }

    pub fn Clear(&mut self) {
        for x in 0..self.size[0] as usize * self.size[1] as usize {
            self.buffer[x] = 0;
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

impl Shape {
    pub fn Move(&mut self, direction: [u32; 2]) {
        self.position[0] += direction[0];
        self.position[1] += direction[1];
    }
}