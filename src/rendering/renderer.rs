use super::colour::Colour;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum RenderCommand {
    Clear(Colour),
    End,
    SetColour(Colour),
    SetBackground(Colour),
    DrawChar(usize, usize, char),

    DrawLine(usize, usize, usize, usize, char),
    DrawBorder(char),
}

pub struct Renderer {
    width:    usize,
    height:   usize,
    commands: Vec<RenderCommand>,
}

impl Renderer {

    pub fn new(size: (usize, usize)) -> Renderer {
        Renderer { width: size.0, height: size.1, commands: Vec::new() }
    }

    pub fn push_cmd(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }

    pub fn update(&mut self) {
        for command in self.commands.iter() {
            match command {
                RenderCommand::Clear(c)                    => self.clear_bg(*c),
                RenderCommand::End                         => Self::clear(),
                RenderCommand::DrawChar(x, y, c)           => self.draw_char(*x, *y, *c, false),
                RenderCommand::SetColour(c)                => Self::set_colour(false, *c),
                RenderCommand::SetBackground(c)            => Self::set_colour(true, *c),

                RenderCommand::DrawBorder(c)               => self.draw_border(*c),
                RenderCommand::DrawLine(x1, y1, x2, y2, c) => self.draw_line(*x1, *y1, *x2, *y2, *c),
            }
        }

        self.commands.clear();
    }

    fn draw_line(&self, x1: usize, y1: usize, x2: usize, y2: usize, c: char) {
        let a =  (y1 + y2) as f64 / (x1 + x2) as f64;
        let a_pos = a >= 0.0;
        let mut counter = 0;
        let mut curr_x = match x1 { x if x > self.width  => self.width,  x => x };
        let mut curr_y = match y1 { y if y > self.height => self.height, y => y };

        let line_cond = |c_x, c_y| match x1 as isize - x2 as isize {
            x if x >= 0 => c_x <= x2,
            _           => c_x >= x2
        } && match y1 as isize - y2 as isize {
            y if y >= 0 => c_y >= y2,
            _           => c_y <= y2
        };

        while curr_x <= self.width && curr_y <= self.height && line_cond(curr_x, curr_y) {
                let cond;
                if a_pos {
                    cond = counter as f64 >= a;
                } else {
                    cond = counter as f64 <= a;
                }

                if cond {
                    curr_x += 1;
                    counter = 0;
                }

                self.draw_char(curr_x, curr_y, c, false);
                curr_y += 1;

                if a_pos {
                    counter += 1;
                } else {
                    counter -= 1;
                }
                
            }
    }

    fn draw_border(&self, c: char) {
        for x in 0..self.width + 2 {
            self.draw_char(x, 0, c, true);
            self.draw_char(x, self.height + 2, c, true);
        }

        for y in 0..self.height + 3 {
            self.draw_char(0, y, c, true);
            self.draw_char(self.width + 2, y, c, true);
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
            x2 = (x % (self.width  + 1)) + 2;
            y2 = (y % (self.height + 1)) + 1;
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

    fn clear() {
        print!("\x1b[0m");
        print!("\x1b[2J");
        print!("\x1b[{};{}H", 0, 0);
        print!(" ");
    }

}
