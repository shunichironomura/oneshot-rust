use std::any::Any;

pub trait Component: Any {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self {
        World { items: Vec::new() }
    }

    pub fn add<C: Component>(&mut self, c: C) {
        self.items.push(Box::new(c));
    }

    /// The stored components, upcast to `dyn Any`, in order.
    /// `&**item` is the component itself: `item` alone would be the `Box`.
    fn anys(&self) -> impl Iterator<Item = &dyn Any> {
        self.items.iter().map(|item| &**item as &dyn Any)
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.anys().find_map(|any| any.downcast_ref::<C>())
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.items
            .iter_mut()
            .find_map(|item| (&mut **item as &mut dyn Any).downcast_mut::<C>())
    }

    pub fn count<C: Component>(&self) -> usize {
        self.anys().filter(|any| any.is::<C>()).count()
    }

    pub fn take_all<C: Component>(&mut self) -> Vec<C> {
        let mut taken = Vec::new();
        let mut kept = Vec::new();
        for item in std::mem::take(&mut self.items) {
            if (&*item as &dyn Any).is::<C>() {
                // Upcast the box itself. The type was checked, so the downcast succeeds.
                let any: Box<dyn Any> = item;
                taken.push(*any.downcast::<C>().unwrap());
            } else {
                kept.push(item);
            }
        }
        self.items = kept;
        taken
    }

    pub fn labels(&self) -> Vec<String> {
        self.items.iter().map(|item| item.label()).collect()
    }
}
