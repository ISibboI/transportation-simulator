use super::indices::{LaneIndex, LaneQueueIndex, RoadIndex};

#[derive(Clone, Copy)]
pub struct RoadLocation {
    road: RoadIndex,
    lane: LaneIndex,
    queue_index: LaneQueueIndex,
}

impl RoadLocation {
    pub fn road(&self) -> RoadIndex {
        self.road
    }

    pub fn lane(&self) -> LaneIndex {
        self.lane
    }

    pub fn queue_index(&self) -> LaneQueueIndex {
        self.queue_index
    }
}
