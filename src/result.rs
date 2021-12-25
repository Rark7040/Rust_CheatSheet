extern crate rand;
use rand::Rng;

pub fn result(){
    let i: i32 = rand::thread_rng().gen_range(1, 3);
    let value: Result<i32, &'static str> = check(i == 2, i);

    match value {
        Ok(x) => println!("{:?}", x),
        Err(err) => println!("[Error] {:?}", err)
    }
}

fn check(boolean: bool, v: i32)->Result<i32, &'static str>{
    if boolean {Ok(v)} else {Err("Invalid value")}
}