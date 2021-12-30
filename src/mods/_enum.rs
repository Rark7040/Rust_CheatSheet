pub fn _enum(){
    let day: Days<&str> = Days::_Sunday("holiday");
    let result = match day{
        Days::_Monday |
        Days::_Tuesday |
        Days::_Wednesday |
        Days::_Thursday |
        Days::_Friday  => "WeekDay",
        Days::_Sunday(x) | Days::_Saturday(x) => x
    };
    println!("it's {:?}!", result);
}

enum Days<T>{
    _Sunday(T),
    _Monday,
    _Tuesday,
    _Wednesday,
    _Thursday,
    _Friday,
    _Saturday(T)
}