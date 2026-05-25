use optional_numeric_index::implement_fixed_index;
use tagged_vec::TaggedVec;
use uom::{
    ConstZero,
    si::{
        f32::{Acceleration, Ratio, ReciprocalLength, Velocity},
        ratio::percent,
    },
};

pub trait DynamicRoadEngine {
    type StaticEngine;

    /// Current angular velocity as ratio of maximum.
    fn angular_velocity_ratio(
        &self,
        static_engine: &Self::StaticEngine,
        velocity: Velocity,
    ) -> Ratio;

    /// Currently selected gear (zero-based).
    fn gear(&self) -> Gear;

    /// Request a certain acceleration from the engine.
    ///
    /// Returns the actual acceleration provided by the engine.
    fn request_acceleration(
        &mut self,
        static_engine: &Self::StaticEngine,
        velocity: Velocity,
        requested_acceleration: Acceleration,
    ) -> Acceleration;
}

implement_fixed_index!(pub Gear, pub OptionalGear, u8);

pub struct DynamicStandardEngine {
    gear: Gear,
}

pub struct StaticStandardEngine {
    gear_max_velocities: TaggedVec<Gear, Velocity>,
    drag_coefficient: ReciprocalLength,
    gear_zero_power: Acceleration,
    model: StandardEngineModel,
}

pub struct StandardEngineModel {
    /// Power output ratio at idle speed.
    idle_power_ratio: Ratio,
    /// Rpm ratio at which power output is maximal.
    peak_power_rpm_ratio: Ratio,
    /// Power output ratio at max rpm.
    max_power_ratio: Ratio,
}

impl StaticStandardEngine {
    pub fn new(
        gear_max_velocities: TaggedVec<Gear, Velocity>,
        drag_coefficient: ReciprocalLength,
        gear_zero_power: Acceleration,
        model: StandardEngineModel,
    ) -> Self {
        assert!(!gear_max_velocities.is_empty());
        assert!(gear_max_velocities[Gear::new(0)] > Velocity::ZERO);
        assert!(
            gear_max_velocities
                .iter_values()
                .zip(gear_max_velocities.iter_values().skip(1))
                .all(|(v1, v2)| v1 < v2)
        );
        assert!(drag_coefficient >= ReciprocalLength::ZERO);
        assert!(gear_zero_power > Acceleration::ZERO);

        Self {
            gear_max_velocities,
            drag_coefficient,
            gear_zero_power,
            model,
        }
    }

    fn acceleration_capability_by_gear(&self, velocity: Velocity, gear: Gear) -> Acceleration {
        assert!(velocity >= Velocity::ZERO);

        let max_velocity = self.gear_max_velocities[gear];
        let rpm_ratio = velocity / max_velocity;
        let power_ratio = self.model.power_ratio(rpm_ratio);
        let gear_max_power = self.gear_max_power(gear);
        let raw_acceleration = power_ratio * gear_max_power;
        let drag_acceleration = -self.drag_coefficient * velocity * velocity;

        raw_acceleration + drag_acceleration
    }

    fn gear_max_power(&self, gear: Gear) -> Acceleration {
        self.gear_zero_power * self.gear_max_velocities[Gear::new(0)]
            / self.gear_max_velocities[gear]
    }
}

impl DynamicRoadEngine for DynamicStandardEngine {
    type StaticEngine = StaticStandardEngine;

    fn angular_velocity_ratio(
        &self,
        static_engine: &Self::StaticEngine,
        velocity: Velocity,
    ) -> Ratio {
        velocity / static_engine.gear_max_velocities[self.gear]
    }

    fn gear(&self) -> Gear {
        self.gear
    }

    fn request_acceleration(
        &mut self,
        static_engine: &Self::StaticEngine,
        velocity: Velocity,
        requested_acceleration: Acceleration,
    ) -> Acceleration {
        // Check if gear should be shifted up.
        if self.gear < Gear::from_usize(static_engine.gear_max_velocities.len() - 1) {
            let higher_gear_acceleration =
                static_engine.acceleration_capability_by_gear(velocity, self.gear.gear_up());
            if higher_gear_acceleration > requested_acceleration {
                self.gear = self.gear.gear_up();
                return requested_acceleration;
            }
        }

        let current_gear_acceleration =
            static_engine.acceleration_capability_by_gear(velocity, self.gear);

        // Check if gear should be shifted down.
        if current_gear_acceleration < requested_acceleration && self.gear > Gear::new(0) {
            self.gear = self.gear.gear_down();
            return requested_acceleration
                .min(static_engine.acceleration_capability_by_gear(velocity, self.gear));
        }

        // If no shifting happens, apply acceleration according to current gear.
        requested_acceleration.min(current_gear_acceleration)
    }
}

impl StandardEngineModel {
    pub fn new(
        idle_power_ratio: Ratio,
        peak_power_rpm_ratio: Ratio,
        max_power_ratio: Ratio,
    ) -> Self {
        assert!(idle_power_ratio <= Ratio::new::<percent>(100.0));
        assert!(idle_power_ratio >= Ratio::new::<percent>(0.0));
        assert!(peak_power_rpm_ratio <= Ratio::new::<percent>(100.0));
        assert!(peak_power_rpm_ratio >= Ratio::new::<percent>(0.0));
        assert!(max_power_ratio <= Ratio::new::<percent>(100.0));
        assert!(max_power_ratio >= Ratio::new::<percent>(0.0));

        Self {
            idle_power_ratio,
            peak_power_rpm_ratio,
            max_power_ratio,
        }
    }

    /// Calculate the power output ratio of the engine at a given RPM ratio.
    ///
    /// **Panics** if `rpm_ratio` is negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use transportation_simulator::road_traffic::vehicle::engine::StandardEngineModel;
    /// use uom::si::f32::Ratio;
    /// use uom::si::ratio::percent;
    /// use approx::assert_relative_eq;
    ///
    /// let model = StandardEngineModel::new(Ratio::new::<percent>(20.0), Ratio::new::<percent>(80.0), Ratio::new::<percent>(70.0));
    /// assert_relative_eq!(model.power_ratio(Ratio::new::<percent>(0.0)).get::<percent>(), 20.0, epsilon = 1e-4);
    /// assert_relative_eq!(model.power_ratio(Ratio::new::<percent>(40.0)).get::<percent>(), 60.0, epsilon = 1e-4);
    /// assert_relative_eq!(model.power_ratio(Ratio::new::<percent>(80.0)).get::<percent>(), 100.0, epsilon = 1e-4);
    /// assert_relative_eq!(model.power_ratio(Ratio::new::<percent>(90.0)).get::<percent>(), 85.0, epsilon = 1e-4);
    /// assert_relative_eq!(model.power_ratio(Ratio::new::<percent>(100.0)).get::<percent>(), 70.0, epsilon = 1e-4);
    /// ```
    pub fn power_ratio(&self, rpm_ratio: Ratio) -> Ratio {
        assert!(rpm_ratio >= Ratio::new::<percent>(0.0));

        if rpm_ratio <= self.peak_power_rpm_ratio {
            // Linear interpolation between idle and peak power.
            let slope =
                (Ratio::new::<percent>(100.0) - self.idle_power_ratio) / self.peak_power_rpm_ratio;
            self.idle_power_ratio + slope * rpm_ratio
        } else {
            // Linear interpolation between peak power and power at max rpm.
            let slope = (self.max_power_ratio - Ratio::new::<percent>(100.0))
                / (Ratio::new::<percent>(100.0) - self.peak_power_rpm_ratio);
            (Ratio::new::<percent>(100.0) + slope * (rpm_ratio - self.peak_power_rpm_ratio))
                .max(Ratio::ZERO)
        }
    }
}

impl Gear {
    fn gear_down(self) -> Self {
        Self(self.0.checked_sub(1).unwrap())
    }

    fn gear_up(self) -> Self {
        Self(self.0.checked_add(1).unwrap())
    }
}
