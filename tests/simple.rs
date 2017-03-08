//! Basic test to see what runs and what doesn't.

#![feature(plugin)]
#![plugin(stainless)]

#[macro_use]
extern crate second_law;

describe! spheres {

    before_each {
        let mut ucmd = new_scene!().ucmd();
    }

    it "should run successfully" {
        ucmd.succeeds();
    }

}
