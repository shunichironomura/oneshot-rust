use crate::judge::{case, check, check_eq};
use crate::user::{Component, World};

#[derive(Debug, PartialEq)]
struct Pos(i32);

#[derive(Debug, PartialEq)]
struct Vel(i32);

#[derive(Debug, PartialEq)]
struct Tag;

#[derive(Debug, PartialEq)]
struct Name(String);

impl Component for Pos {
    fn label(&self) -> String {
        "pos".to_string()
    }
}

impl Component for Vel {
    fn label(&self) -> String {
        "vel".to_string()
    }
}

impl Component for Tag {
    fn label(&self) -> String {
        "tag".to_string()
    }
}

impl Component for Name {
    fn label(&self) -> String {
        self.0.clone()
    }
}

fn name(s: &str) -> Name {
    Name(s.to_string())
}

pub fn run() {
    case("sample-1", || {
        let mut w = World::new();
        w.add(Pos(1));
        w.add(Vel(2));
        w.add(Pos(3));
        check_eq(w.count::<Pos>(), 2)?;
        check_eq(w.get::<Vel>(), Some(&Vel(2)))?;
        check_eq(w.take_all::<Pos>(), vec![Pos(1), Pos(3)])?;
        check_eq(w.labels(), vec!["vel"])
    });
    case("sample-2", || {
        let mut w = World::new();
        w.add(Vel(5));
        w.add(Tag);
        w.add(Vel(7));
        let Some(v) = w.get_mut::<Vel>() else {
            return Err("get_mut::<Vel>() returned None".to_string());
        };
        v.0 += 1;
        check_eq(w.get::<Vel>(), Some(&Vel(6)))?;
        check_eq(w.get::<Pos>(), None)?;
        check_eq(w.labels(), vec!["vel", "tag", "vel"])
    });
    case("hidden-1", || {
        // An empty world.
        let mut w = World::new();
        check_eq(w.labels(), Vec::<String>::new())?;
        check_eq(w.get::<Pos>(), None)?;
        check(w.get_mut::<Tag>().is_none(), "get_mut on an empty world")?;
        check_eq(w.count::<Tag>(), 0)?;
        check_eq(w.take_all::<Vel>(), Vec::new())?;
        check_eq(w.labels(), Vec::<String>::new())
    });
    case("hidden-2", || {
        // get and get_mut return the first match, and get_mut changes the stored item.
        let mut w = World::new();
        w.add(Vel(9));
        w.add(Pos(1));
        w.add(Pos(2));
        check_eq(w.get::<Pos>(), Some(&Pos(1)))?;
        let Some(p) = w.get_mut::<Pos>() else {
            return Err("get_mut::<Pos>() returned None".to_string());
        };
        p.0 = 10;
        check_eq(w.get::<Pos>(), Some(&Pos(10)))?;
        check_eq(w.count::<Pos>(), 2)?;
        check_eq(w.take_all::<Pos>(), vec![Pos(10), Pos(2)])?;
        check_eq(w.get::<Vel>(), Some(&Vel(9)))
    });
    case("hidden-3", || {
        // A zero-sized component.
        let mut w = World::new();
        w.add(Tag);
        w.add(Pos(1));
        w.add(Tag);
        w.add(Tag);
        check_eq(w.count::<Tag>(), 3)?;
        check_eq(w.get::<Tag>(), Some(&Tag))?;
        check_eq(w.count::<Vel>(), 0)?;
        check_eq(w.take_all::<Tag>(), vec![Tag, Tag, Tag])?;
        check_eq(w.count::<Tag>(), 0)?;
        check_eq(w.labels(), vec!["pos"])
    });
    case("hidden-4", || {
        // take_all keeps the order of both the taken and the remaining items.
        let mut w = World::new();
        w.add(Pos(1));
        w.add(Pos(2));
        w.add(Vel(3));
        w.add(Pos(4));
        w.add(Tag);
        w.add(Vel(5));
        w.add(Pos(6));
        check_eq(w.take_all::<Name>(), Vec::new())?;
        check_eq(w.labels(), vec!["pos", "pos", "vel", "pos", "tag", "vel", "pos"])?;
        check_eq(w.take_all::<Pos>(), vec![Pos(1), Pos(2), Pos(4), Pos(6)])?;
        check_eq(w.labels(), vec!["vel", "tag", "vel"])?;
        check_eq(w.get::<Vel>(), Some(&Vel(3)))?;
        check_eq(w.take_all::<Pos>(), Vec::new())?;
        check_eq(w.take_all::<Vel>(), vec![Vel(3), Vel(5)])?;
        check_eq(w.labels(), vec!["tag"])
    });
    case("hidden-5", || {
        // Types are told apart by type, not by label.
        let mut w = World::new();
        w.add(name("pos"));
        w.add(Pos(7));
        w.add(name("vel"));
        check_eq(w.get::<Pos>(), Some(&Pos(7)))?;
        check_eq(w.count::<Name>(), 2)?;
        check_eq(w.count::<Pos>(), 1)?;
        check_eq(w.count::<Vel>(), 0)?;
        check_eq(w.labels(), vec!["pos", "pos", "vel"])?;
        check_eq(w.take_all::<Name>(), vec![name("pos"), name("vel")])?;
        check_eq(w.labels(), vec!["pos"])?;
        check_eq(w.get::<Name>(), None)
    });
    case("hidden-6", || {
        // Items added after take_all go to the end; owned data moves out intact.
        let mut w = World::new();
        w.add(name("a"));
        w.add(Vel(1));
        w.add(name("b"));
        check_eq(w.take_all::<Vel>(), vec![Vel(1)])?;
        w.add(Vel(2));
        w.add(name("c"));
        check_eq(w.labels(), vec!["a", "b", "vel", "c"])?;
        let Some(n) = w.get_mut::<Name>() else {
            return Err("get_mut::<Name>() returned None".to_string());
        };
        n.0.push('!');
        check_eq(w.labels(), vec!["a!", "b", "vel", "c"])?;
        check_eq(w.take_all::<Name>(), vec![name("a!"), name("b"), name("c")])?;
        check_eq(w.labels(), vec!["vel"])
    });
}
