use tagged_vec::TaggedVec;
use uom::si::f32::Time;

use super::{
    indices::{RoadIndex, RoadVehicleIndex},
    road::{DynamicRoad, LaneQueueEntry, StaticRoad},
    vehicle::{DynamicRoadVehicle, StaticRoadVehicle, engine::DynamicStandardEngine},
};

#[derive(Default)]
pub struct DynamicRoadNetwork {
    roads: TaggedVec<RoadIndex, DynamicRoad>,
    vehicles: TaggedVec<RoadVehicleIndex, DynamicRoadVehicle<DynamicStandardEngine>>,
}

pub struct StaticRoadNetwork {
    roads: TaggedVec<RoadIndex, StaticRoad>,
    vehicles: TaggedVec<RoadVehicleIndex, StaticRoadVehicle<DynamicStandardEngine>>,
}

impl DynamicRoadNetwork {
    pub fn road(&self, index: RoadIndex) -> &DynamicRoad {
        &self.roads[index]
    }

    pub fn update(
        &mut self,
        old_road_network: &Self,
        static_road_network: &StaticRoadNetwork,
        delta: Time,
    ) {
        self.roads.clear();
        self.vehicles.clear();

        for (road_index, static_road) in static_road_network.roads.iter(..) {
            self.roads.push(DynamicRoad::new(static_road));
            let road = &mut self.roads[road_index];
            for (lane_index, lane) in static_road_network.roads[road_index].lanes().iter(..) {
                for (queue_index, vehicle) in old_road_network
                    .road(road_index)
                    .lane(lane_index)
                    .queue()
                    .iter(..)
                {
                    let vehicle_index = vehicle.vehicle();
                    let offset = vehicle.offset();
                }
            }
        }
    }
}
