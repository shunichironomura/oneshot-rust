use crate::judge::{case, check_eq};
use crate::user::{Command, parse_command};

fn say(s: &str) -> Option<Command> {
    Some(Command::Say(s.to_string()))
}

pub fn run() {
    case("example-1", || {
        check_eq(
            [parse_command("move 3 -2"), parse_command("go west"), parse_command("say  hello   world ")],
            [Some(Command::Move { x: 3, y: -2 }), Some(Command::Move { x: -1, y: 0 }), say("hello world")],
        )
    });
    case("example-2", || {
        check_eq(
            [parse_command("wait"), parse_command("wait 61"), parse_command("exit"), parse_command("dance")],
            [Some(Command::Wait(1)), None, Some(Command::Quit), None],
        )
    });
    case("case-1", || {
        check_eq(
            [parse_command(""), parse_command("   "), parse_command("say"), parse_command("quit now")],
            [None, None, None, None],
        )
    });
    case("case-2", || {
        check_eq(
            [parse_command("move 1"), parse_command("move 1 2 3"), parse_command("move a 2"), parse_command("move 1 b")],
            [None, None, None, None],
        )
    });
    case("case-3", || {
        check_eq(
            [parse_command("wait 1"), parse_command("wait 60"), parse_command("wait 0"), parse_command("wait -5")],
            [Some(Command::Wait(1)), Some(Command::Wait(60)), None, None],
        )
    });
    case("case-4", || {
        check_eq(
            [parse_command("go north"), parse_command("go south"), parse_command("go east"), parse_command("go up"), parse_command("go")],
            [
                Some(Command::Move { x: 0, y: 1 }),
                Some(Command::Move { x: 0, y: -1 }),
                Some(Command::Move { x: 1, y: 0 }),
                None,
                None,
            ],
        )
    });
    case("case-5", || {
        // Commands are case-sensitive, and words after "say" are kept as they are.
        check_eq(
            [parse_command("Quit"), parse_command("say quit move 1 2"), parse_command("\tquit\n")],
            [None, say("quit move 1 2"), Some(Command::Quit)],
        )
    });
}
