pub fn closure(){
    let fnc = |i: u32| -> u32 {
        i*10
    };
    println!("{:?}", fnc(19));
}