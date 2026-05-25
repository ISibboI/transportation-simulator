use uom::si::f32::Time;

use crate::road_traffic::road_network::{DynamicRoadNetwork, StaticRoadNetwork};

pub struct StaticWorld {
    road_network: StaticRoadNetwork,
}

#[derive(Default)]
pub struct DynamicWorld {
    road_network: DynamicRoadNetwork,
}

impl DynamicWorld {
    pub fn update(&mut self, old_world: &Self, static_world: &StaticWorld, delta: Time) {
        self.road_network
            .update(&old_world.road_network, &static_world.road_network, delta);
    }
}
