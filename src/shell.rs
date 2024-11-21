#[derive(Debug, Clone, Copy)]
pub enum ShellType {
    Live,
    Blank,
}

#[derive(Debug, Clone)]
pub struct Shell(ShellType);

impl Shell {
    pub fn new(shell_type: ShellType) -> Self {
        Shell(shell_type)
    }

    pub fn invert(&mut self) {
        match self.0 {
            ShellType::Live => self.0 = ShellType::Blank,
            ShellType::Blank => self.0 = ShellType::Live,
        }
    }
}
