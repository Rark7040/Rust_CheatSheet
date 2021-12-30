extern crate rand;
use rand::Rng;

pub fn result(){
    let i: i32 = rand::thread_rng().gen_range(1, 3);
    let value: Result<i32, &'static str> = check(i == 1, i);
    let result: Result<i32, &'static str> = handle(value);
    match result{
        Ok(x) => println!("[SUCCESS] {}", x),
        Err(err) => println!("[ERROR] {}", err)
    }
}

fn check(boolean: bool, v: i32)->Result<i32, &'static str>{
    if boolean {Ok(v)} else {Err("Invalid value")}
}

fn handle(r: Result<i32, &'static str>)->Result<i32, &'static str>{
    let ok = r?; //errはここで弾かれる
    //some
    Ok(ok)
}