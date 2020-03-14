use super::colour::Colour;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum RenderCommand {
    Clear(Colour),
    End,
    SetColour(Colour),
    SetBackground(Colour),
    DrawChar(usize, usize, char),

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
                RenderCommand::Clear(c)          => self.clear_bg(),
                RenderCommand::End               => Self::clear(),
                RenderCommand::DrawChar(x, y, c) => self.draw_char(*x, *y, *c, false),
                RenderCommand::SetColour(c)      => Self::set_colour(false, *c),
                RenderCommand::SetBackground(c)  => Self::set_colour(true, *c),

                RenderCommand::DrawBorder(c)     => self.draw_border(*c),
            }
        }

        self.commands.clear();
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
            y2 = (y % (self.height + 1)) + 2;
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
