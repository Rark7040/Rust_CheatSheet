use std::fmt::{Debug, Formatter, Result};

pub struct Vector3{
    x: i32,
    y: i32,
    z: i32
}
impl Vector3{
    pub fn new(x: i32, y:i32, z:i32)->Vector3 {Vector3{x, y, z}}
    pub fn _get_x(&self)->&i32 {&self.x}
    pub fn _get_y(&self)->&i32 {&self.y}
    pub fn _get_z(&self)->&i32 {&self.z}
}

impl Debug for Vector3{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Drop for Vector3{
    fn drop(&mut self){//destruct
        println!("on drop...");
    }
}

pub fn _struct(){
    let v = Vector3::new(10, 30, 10);
    println!("{:?}", v);
}