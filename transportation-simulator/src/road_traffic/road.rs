use tagged_vec::TaggedVec;
use tagged_vec_deque::TaggedVecDeque;
use uom::si::f32::Length;

use super::indices::{LaneIndex, LaneQueueIndex, RoadVehicleIndex};

#[derive(Default)]
pub struct DynamicRoad {
    lanes: TaggedVec<LaneIndex, DynamicLane>,
}

pub struct StaticRoad {
    lanes: TaggedVec<LaneIndex, StaticLane>,
}

#[derive(Default)]
pub struct DynamicLane {
    queue: TaggedVecDeque<LaneQueueIndex, LaneQueueEntry>,
}

pub struct StaticLane {
    length: Length,
}

pub struct LaneQueueEntry {
    vehicle: RoadVehicleIndex,
    offset: Length,
}

impl StaticRoad {
    pub fn translate_lane_offset(
        &self,
        from_lane: LaneIndex,
        to_lane: LaneIndex,
        offset: Length,
    ) -> Length {
        let ratio = offset / self.lanes[from_lane].length;
        ratio * self.lanes[to_lane].length
    }
}

impl DynamicRoad {
    pub fn new(static_road: &StaticRoad) -> Self {
        Self {
            lanes: TaggedVec::from_iter(
                static_road
                    .lanes()
                    .iter_indices(..)
                    .map(|_| DynamicLane::default()),
            ),
        }
    }

    pub fn lane(&self, index: LaneIndex) -> &DynamicLane {
        &self.lanes[index]
    }
}

impl StaticRoad {
    pub fn lanes(&self) -> &TaggedVec<LaneIndex, StaticLane> {
        &self.lanes
    }
}

impl DynamicLane {
    pub fn queue(&self) -> &TaggedVecDeque<LaneQueueIndex, LaneQueueEntry> {
        &self.queue
    }

    pub fn vehicle(&self, queue_index: LaneQueueIndex) -> RoadVehicleIndex {
        self.queue[queue_index].vehicle
    }
}

impl LaneQueueEntry {
    pub fn vehicle(&self) -> RoadVehicleIndex {
        self.vehicle
    }

    pub fn offset(&self) -> Length {
        self.offset
    }
}
