pub fn cast(){
	let x: i32 = 10;
	let y: i64 = x as i64;

	let a = 100;
	let b = a.to_string();

	println!("cast {:?} {:?}", y, b);
}