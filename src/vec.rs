pub fn vec(){
    let mut v = vec![10, 40, 10];
    v.push(30);
    println!("Vec [");

    for value in v{
        println!("  {:?},", value);
    }
    println!("]");
}