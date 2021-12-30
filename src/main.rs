#![allow(non_snake_case)]
mod mods;
use mods::{*};

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
    result::result();
    _box::_box();
    _loop::_loop();
    _for::_for();
    _enum::_enum();
    _range::_range();
    _thread::_thread();
}