use optional_numeric_index::implement_fixed_index;

implement_fixed_index!(pub RoadIndex, pub OptionalRoadIndex, usize);

implement_fixed_index!(pub LaneIndex, pub OptionalLaneIndex, usize);

implement_fixed_index!(pub RoadVehicleIndex, pub OptionalRoadVehicleIndex, u32);

implement_fixed_index!(pub LaneQueueIndex, pub OptionalLaneQueueIndex, usize);
