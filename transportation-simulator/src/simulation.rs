use std::{cell::RefCell, rc::Rc};

use uom::si::f32::Time;

use crate::world::{DynamicWorld, StaticWorld};

pub struct Simulation {
    dynamic_worlds: [Rc<RefCell<DynamicWorld>>; 2],
    static_world: StaticWorld,
}

impl Simulation {
    pub fn new(initial_dynamic_world: DynamicWorld, static_world: StaticWorld) -> Self {
        Self {
            dynamic_worlds: [
                Rc::new(RefCell::new(initial_dynamic_world)),
                Rc::new(RefCell::new(DynamicWorld::default())),
            ],
            static_world,
        }
    }

    pub fn update(&mut self, delta: Time) {
        self.dynamic_worlds.swap(0, 1);
        let new_world = self.dynamic_worlds[0].clone();
        let mut new_world = new_world.borrow_mut();
        let old_world = self.dynamic_worlds[1].clone();
        let old_world = old_world.borrow();

        new_world.update(&old_world, &self.static_world, delta);
    }
}
