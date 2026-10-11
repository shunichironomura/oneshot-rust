#[derive(Debug, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Wait(u32),
    Quit,
}

pub fn parse_command(line: &str) -> Option<Command> {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        ["move", x, y] => Some(Command::Move {
            x: x.parse().ok()?,
            y: y.parse().ok()?,
        }),
        ["go", dir] => {
            let (x, y) = match *dir {
                "north" => (0, 1),
                "south" => (0, -1),
                "east" => (1, 0),
                "west" => (-1, 0),
                _ => return None,
            };
            Some(Command::Move { x, y })
        }
        ["say", rest @ ..] if !rest.is_empty() => Some(Command::Say(rest.join(" "))),
        ["wait"] => Some(Command::Wait(1)),
        ["wait", n] => match n.parse() {
            Ok(n @ 1..=60) => Some(Command::Wait(n)),
            _ => None,
        },
        ["quit" | "exit"] => Some(Command::Quit),
        _ => None,
    }
}
