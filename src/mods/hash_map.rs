use std::collections::HashMap;

pub fn hashMap(){
	let mut map = HashMap::new();
	map.insert("aaa", 10);
	map.insert("bbb", 20);
	map.insert("ccc", 30);
	map.remove("bbb");
	println!("{:?}", map);
}