#![allow(non_snake_case)]
mod cast;
mod array;
mod hash_map;
mod vec;
mod closure;
mod tup;
mod _macro;
mod _if;
mod _struct;
mod option;
mod iterator;

fn main() {
    cast::cast();
    array::array();
    hash_map::hashMap();
    vec::vec();
    closure::closure();
    tup::tup();
    _macro::_macro();
    _if::_if("Hello");
    _struct::_struct();
    option::option();
    iterator::iterator();
}