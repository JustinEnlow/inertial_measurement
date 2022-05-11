//! # Inertial Measurement Unit(IMU)

use game_utils::dimension3::Dimension3;
use game_utils::control_axis::ControlAxis;



#[derive(Clone, Copy)]
pub struct IMU<T>{
    acceleration: ControlAxis<Dimension3<T>>,
    velocity: ControlAxis<Dimension3<T>>
}

impl<T> IMU<T>
    where T: Copy
{
    pub fn new(zero: T) -> Self{
        Self{
            acceleration: ControlAxis::new(
                Dimension3::new(zero, zero, zero),
                Dimension3::new(zero, zero, zero)
            ),
            velocity: ControlAxis::new(
                Dimension3::new(zero, zero, zero),
                Dimension3::new(zero, zero, zero)
            )
        }
    }

    pub fn acceleration(self: &Self) -> &ControlAxis<Dimension3<T>>{
        &self.acceleration
    }
    pub fn acceleration_mut(self: &mut Self) -> &mut ControlAxis<Dimension3<T>>{
        &mut self.acceleration
    }

    pub fn velocity(self: &Self) -> &ControlAxis<Dimension3<T>>{
        &self.velocity
    }
    pub fn velocity_mut(self: &mut Self) -> &ControlAxis<Dimension3<T>>{
        &mut self.velocity
    }
}