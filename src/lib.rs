use dimension3::Dimension3;



#[derive(Clone, Copy)]
pub struct IMU<T>{
    acceleration: Acceleration<T>,
    velocity: Velocity<T>
}

impl<T> IMU<T>
    where T: Copy
{
    pub fn new(zero: T) -> Self{
        Self{
            acceleration: Acceleration::new(zero),
            velocity: Velocity::new(zero)
        }
    }

    pub fn acceleration(self: &Self) -> &Acceleration<T>{
        &self.acceleration
    }
    pub fn acceleration_mut(self: &mut Self) -> &mut Acceleration<T>{
        &mut self.acceleration
    }

    pub fn velocity(self: &Self) -> &Velocity<T>{
        &self.velocity
    }
    pub fn velocity_mut(self: &mut Self) -> &mut Velocity<T>{
        &mut self.velocity
    }
}

#[derive(Clone, Copy)]
pub struct Acceleration<T>{
    linear: Dimension3<T>,
    rotational: Dimension3<T>,
}

impl<T> Acceleration<T>
    where T: Copy
{
    pub fn new(zero: T) -> Self{
        Self{
            linear: Dimension3::new(zero, zero, zero),
            rotational: Dimension3::new(zero, zero, zero),
        }
    }

    pub fn linear(self: &Self) -> &Dimension3<T>{
        &self.linear
    }
    pub fn linear_mut(self: &mut Self) -> &mut Dimension3<T>{
        &mut self.linear
    }

    pub fn rotational(self: &Self) -> &Dimension3<T>{
        &self.rotational
    }
    pub fn rotational_mut(self: &mut Self) -> &mut Dimension3<T>{
        &mut self.rotational
    }
}

#[derive(Clone, Copy)]
pub struct Velocity<T>{
    linear: Dimension3<T>,
    rotational: Dimension3<T>
}

impl<T> Velocity<T>
    where T: Copy
{
    pub fn new(zero: T) -> Self{
        Self{
            linear: Dimension3::new(zero, zero, zero),
            rotational: Dimension3::new(zero, zero, zero)
        }
    }

    pub fn linear(self: &Self) -> &Dimension3<T>{
        &self.linear
    }
    pub fn linear_mut(self: &mut Self) -> &mut Dimension3<T>{
        &mut self.linear
    }

    pub fn rotational(self: &Self) -> &Dimension3<T>{
        &self.rotational
    }
    pub fn rotational_mut(self: &mut Self) -> &mut Dimension3<T>{
        &mut self.rotational
    }
}
