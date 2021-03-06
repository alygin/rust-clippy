#![feature(plugin)]
#![plugin(clippy)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![deny(unsafe_removed_from_name)]

use std::cell::{UnsafeCell as TotallySafeCell};
//~^ ERROR removed "unsafe" from the name of `UnsafeCell` in use as `TotallySafeCell`

use std::cell::UnsafeCell as TotallySafeCellAgain;
//~^ ERROR removed "unsafe" from the name of `UnsafeCell` in use as `TotallySafeCellAgain`

// Shouldn't error
use std::cell::{UnsafeCell as SuperDangerousUnsafeCell};
use std::cell::{UnsafeCell as Dangerunsafe};
use std::cell::UnsafeCell as Bombsawayunsafe;
use std::cell::{RefCell as ProbablyNotUnsafe};
use std::cell::RefCell as RefCellThatCantBeUnsafe;

mod mod_with_some_unsafe_things {
    pub struct Safe {}
    pub struct Unsafe {}
}

use mod_with_some_unsafe_things::Unsafe as LieAboutModSafety;
//~^ ERROR removed "unsafe" from the name of `Unsafe` in use as `LieAboutModSafety`

// Shouldn't error
use mod_with_some_unsafe_things::Safe as IPromiseItsSafeThisTime;
use mod_with_some_unsafe_things::Unsafe as SuperUnsafeModThing;

fn main() {}
