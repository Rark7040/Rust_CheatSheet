extern crate rand;
use rand::Rng;

pub fn option(){
    let i: i32 = rand::thread_rng().gen_range(1, 10);
    let value: Option<i32> = get_value(i < 5, 100);

    match value {
        Some(x) => println!("{:?}", x),
        None => println!("none")
    }
}


fn get_value(boolean: bool, v: i32)->Option<i32>{
    if boolean {
        Some(v)

    } else {
        None
    }
}