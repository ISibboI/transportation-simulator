use uom::si::f32::{Acceleration, Length, Velocity};

use super::road_location::RoadLocation;

pub mod engine;

pub struct RoadVehicle<Engine> {
    length: Length,
    location: RoadLocation,
    velocity: Velocity,
    target_acceleration: Acceleration,
    actual_acceleration: Acceleration,
    engine: Engine,
}

impl<Engine> RoadVehicle<Engine> {
    pub fn length(&self) -> Length {
        self.length
    }

    pub fn location(&self) -> RoadLocation {
        self.location
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn target_acceleration(&self) -> Acceleration {
        self.target_acceleration
    }

    pub fn actual_acceleration(&self) -> Acceleration {
        self.actual_acceleration
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
