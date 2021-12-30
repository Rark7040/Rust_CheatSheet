use std::ops::RangeInclusive;

pub fn _range(){
    let range: RangeInclusive<i32> = 0..=10;

    if range.contains(&10){
        println!("inside");

    }else{
        println!("outside");
    }
}