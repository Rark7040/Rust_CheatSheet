macro_rules! test {
    ($x: expr) => {
        $x = 20;
    }
}

pub fn _macro(){
    let x: i32;
    test!(x);
    println!("{:?}", x);
}