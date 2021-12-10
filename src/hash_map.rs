use std::collections::HashMap;

pub fn hashMap(){
	let mut map = HashMap::new();
	map.insert("aaa".to_string(), 10);
	println!("{:?}", map);
}