#![allow(non_snake_case)]
use std::io::{self, Write};

pub struct Canvas {
    pub size: [i32; 2],
    pub buffer: Vec<Vec<i32>>
}

pub struct Shape {
    pub size: [u32; 2],
    pub position: [u32; 2],
}

impl Canvas {
    pub fn new(x_size: i32, y_size: i32) -> Self {
        Canvas {size: [x_size, y_size], buffer: vec![vec![]]}
    }
    pub fn init(&mut self) {
        for x in 0..self.size[0] as usize{
            self.buffer.push(Vec::new());
            for _y in 0..self.size[1] as usize {
                self.buffer[x].push(0);
            }
        }
    }
    pub fn draw(&mut self, shape: &Shape) {
        for y in 1..self.size[1] as usize - 1 {
            for x in 1..self.size[0] as usize - 1 {
                if x as u32 == shape.position[0] && y as u32 == shape.position[1] {
                    self.buffer[x][y] = 1;
                }
            }
        }
    }
    pub fn draw_border(&mut self) {
        for y in 0..self.size[1] as usize {
            for x in 0..self.size[0] as usize {
                if x as i32 == 0 || x as i32 == self.size[0] - 1 {
                    self.buffer[x][y] = 2;
                }
                if y as i32 == 0 || y as i32 == self.size[1] - 1 {
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
                    _ => print!("{}", "?")
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
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl Shape {
    pub fn new(x_pos: u32, y_pos: u32, x_size: u32, y_size: u32) -> Self {
        Shape {position: [x_pos, y_pos], size: [x_size, y_size]}
    }
    pub fn Move(&mut self, x_direction:  u32, y_direction: u32) {
        self.position[0] += x_direction;
        self.position[1] += y_direction;
    }
}