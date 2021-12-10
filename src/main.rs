#![allow(non_snake_case)]
mod cast;
mod array;
mod hash_map;
mod vec;
mod closure;
mod tup;

fn main() {
    cast::cast();
    array::array();
    hash_map::hashMap();
    vec::vec();
    closure::closure();
    tup::tup();
}