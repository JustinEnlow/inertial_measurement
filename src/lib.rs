//! # Inertial Measurement Unit(IMU)



#[derive(Clone, Copy)]
pub struct IMU<T>{
    acceleration: T,
    velocity: T
}

impl<T> IMU<T>
    where T: Copy
{
    pub fn new(acceleration: T, velocity: T) -> Self{
        Self{
            acceleration: acceleration,
            velocity: velocity
        }
    }

    pub fn acceleration(self: &Self) -> &T{
        &self.acceleration
    }
    pub fn acceleration_mut(self: &mut Self) -> &mut T{
        &mut self.acceleration
    }

    pub fn velocity(self: &Self) -> &T{
        &self.velocity
    }
    pub fn velocity_mut(self: &mut Self) -> &mut T{
        &mut self.velocity
    }
}