pub fn iterator(){
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2 = v.iter().map(|x| x*2).collect::<Vec<i32>>();
    println!("{:?}", v2);
}