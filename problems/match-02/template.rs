#[derive(Debug, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Wait(u32),
    Quit,
}

pub fn parse_command(line: &str) -> Option<Command> {
    todo!()
}
