use super::colour::Colour;

use std::io::prelude::*;                                                           
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum RenderCommand {
    Clear(Colour),
    Reset,
    SetColour(Colour),
    SetBackground(Colour),
    DrawChar(usize, usize, char),

    DrawLine(usize, usize, usize, usize, char),
    DrawString(usize, usize, String),
    DrawBorder(char),
}

pub struct Renderer {
    pub width:    usize,
    pub height:   usize,
    commands: Vec<RenderCommand>,
}

impl Renderer {

    pub fn new(size: (usize, usize)) -> Renderer {
        let mut r = Renderer { width: size.0, height: size.1, commands: Vec::new() };
        r.push_cmd(RenderCommand::Reset);
        r.update();
        r
    }

    pub fn push_cmds(&mut self, mut commands: Vec<RenderCommand>) {
        self.commands.append(&mut commands);
    }

    pub fn push_cmd(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }

    pub fn update(&mut self) {
        for command in self.commands.iter() {
            match command {
                RenderCommand::Clear(c)                    => self.clear_bg(*c),
                RenderCommand::Reset                       => Self::clear(),
                RenderCommand::DrawChar(x, y, c)           => { self.draw_char(*x, *y, *c, false); Self::flush() },
                RenderCommand::SetColour(c)                => Self::set_colour(false, *c),
                RenderCommand::SetBackground(c)            => Self::set_colour(true, *c),

                RenderCommand::DrawBorder(c)               => { self.draw_border(*c); Self::flush() },
                RenderCommand::DrawLine(x1, y1, x2, y2, c) => { self.draw_line(*x1, *y1, *x2, *y2, *c); Self::flush() },
                RenderCommand::DrawString(x, y, s)         => { self.draw_string(*x, *y, s); Self::flush() }, 
            }
        }

        self.commands.clear();
    }

    fn draw_string(&self, x_: usize, y_: usize, s: &String) {
        let first_x = x_;
        let mut x = x_;
        let mut y = y_;
        for c in s.chars() {
            match c {
                '\n' => { y += 1; x = first_x; },
                _    => self.draw_char(x, y, c, false),
            }
            x += 1;
        }
    }

    fn draw_line(&self, x1_: usize, y1_: usize, x2_: usize, y2_: usize, c: char) {
        let x1 = x1_ as f64;
        let x2 = x2_ as f64;
        let y1 = y1_ as f64;
        let y2 = y2_ as f64;
        let mut delta_x = x2 - x1;
        let mut delta_y = y2 - y1;
        let mut x: f64; let mut y: f64;
        let step: f64;
        let mut i = 1;
        
        if delta_x.abs() >= delta_y.abs() {
            step = delta_x.abs();
        } else {
            step = delta_y.abs();
        }

        delta_x /= step;
        delta_y /= step;
        x = x1;
        y = y1;

        while i as f64 <= step {
            self.draw_char(x as usize, y as usize, c, false);
            x += delta_x;
            y += delta_y;
            i += 1;
        }
    }

    fn draw_border(&self, c: char) {
        for x in 0..self.width + 1 {
            self.draw_char(x, 0, c, true);
            self.draw_char(x, self.height + 1, c, true);
        }

        for y in 0..self.height + 2 {
            self.draw_char(0, y, c, true);
            self.draw_char(self.width + 1, y, c, true);
        }
    }

    fn draw_char(&self, x: usize, y: usize, c: char, real: bool) {
        self.set_position(x, y, real);
        print!("{}", c);
    }

    fn set_position(&self, x: usize, y: usize, real: bool) {
        let x2; let y2;
        if real {
            x2 = x + 1;
            y2 = y + 1;
        } else {
            x2 = (x % self.width)  + 2;
            y2 = (y % self.height) + 2;
        }
        print!("\x1b[{};{}H", y2, x2);
    }

    fn set_colour(back: bool, c: Colour) {
        let ansi = match back {
            true  => 48,
            false => 38,
        };

        print!("\x1b[{};2;{};{};{}m", ansi, c.r, c.g, c.b);
    }

    fn clear_bg(&self, c: Colour) {
        Self::clear();
        Self::set_colour(true, c);
        for x in 0..self.width + 2 {
            for y in 0..self.height + 2 {
                self.draw_char(x, y, ' ', true);
            }
        }
    }

    fn flush() {
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    fn clear() {
        print!("\x1b[0m");
        print!("\x1b[2J");
        print!("\x1b[{};{}H", 0, 0);
        print!(" ");
    }

}
