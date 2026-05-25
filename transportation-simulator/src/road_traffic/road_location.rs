use super::{road_index::RoadIndex, road_queue_index::RoadQueueIndex};

#[derive(Clone, Copy)]
pub struct RoadLocation {
    road: RoadIndex,
    queue_index: RoadQueueIndex,
}

impl RoadLocation {
    pub fn road(&self) -> RoadIndex {
        self.road
    }

    pub fn queue_index(&self) -> RoadQueueIndex {
        self.queue_index
    }
}
