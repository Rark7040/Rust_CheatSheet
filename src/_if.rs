pub fn _if(arg: &str){
    let msg: String  = if arg == "Hello"{
        arg.to_string()

    }else{
        String::from("lol")
    };
    println!("{:?}", msg);
}