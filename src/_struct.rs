use std::fmt::{Debug, Formatter, Result};

pub struct Vector3{
    x: i32,
    y: i32,
    z: i32
}

pub struct Position{
    vec: Vector3,
    world: String
}

impl Vector3{
    pub fn new(x: i32, y: i32, z: i32)->Vector3{Vector3{x, y, z}}
    pub fn _get_x(&self)->&i32 {&self.x}
    pub fn _get_y(&self)->&i32 {&self.y}
    pub fn _get_z(&self)->&i32 {&self.z}
    pub fn _set_x(&mut self, new_x: i32){self.x = new_x;}
    pub fn _set_y(&mut self, new_y: i32){self.y = new_y;}
    pub fn _set_z(&mut self, new_z: i32){self.z = new_z;}
}

impl Position{
    pub fn new(x: i32, y: i32, z: i32, world: String)->Position{
        Position{vec: Vector3::new(x, y, z), world}
    }
    pub fn _get_from_vector(vec: Vector3, world: String)->Position{
        Position{vec, world}
    }
    pub fn _get_vec_mut(&mut self)->&mut Vector3{&mut self.vec}
    pub fn _get_world(&self)->&String{&self.world}
    pub fn _set_world(&mut self, new_world: String){self.world = new_world;}
}

impl Debug for Vector3{//マクロで実装できないかな
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Debug for Position{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Position")
            .field("x", &self.vec.x)
            .field("y", &self.vec.y)
            .field("z", &self.vec.z)
            .field("world", &self.world)
            .finish()
    }
}

impl Drop for Vector3{
    fn drop(&mut self){//destruct
        println!("on drop...");
    }
}

pub fn _struct(){
    let mut pos = Position::new(10, 30, 10, String::from("World"));
    pos._get_vec_mut()._set_x(50);
    pos._set_world(String::from("World_copied"));
    println!("{:?}", pos);
}