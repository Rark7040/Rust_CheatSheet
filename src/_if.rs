pub fn _if(arg: &str){
  if arg == "Hello"{
      let mut msg = arg.to_string();
      msg.push_str(", World!");
      println!("{}", msg);

  }else{
      println!("lol");
  }
}