#![allow(non_snake_case)]
mod cast;
mod array;
mod hash_map;
mod vec;
mod closure;
mod tup;
mod _macro;

fn main() {
    cast::cast();
    array::array();
    hash_map::hashMap();
    vec::vec();
    closure::closure();
    tup::tup();
    _macro::_macro();
}