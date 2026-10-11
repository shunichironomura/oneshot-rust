use std::any::Any;

pub trait Component: Any {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add<C: Component>(&mut self, c: C) {
        todo!()
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        todo!()
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        todo!()
    }

    pub fn count<C: Component>(&self) -> usize {
        todo!()
    }

    pub fn take_all<C: Component>(&mut self) -> Vec<C> {
        todo!()
    }

    pub fn labels(&self) -> Vec<String> {
        todo!()
    }
}
