pub fn _loop(){
    let mut i: i32 = 0;
    let result = loop{
        i+=1;

        if i > 29{
            break i;
        }
    };
    println!("{:?}", result);
}
