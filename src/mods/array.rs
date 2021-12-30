pub fn array(){
	let ar:[u64; 5] = [10, 20, 30, 40, 50];
	let ar2:[&str; 5] = ["test1", "test2", "test3", "test4", "test5"];

	println!("{:?}", ar);
	println!("{:?}", ar2);
}