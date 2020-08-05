//! Benchmarks :^)
#![allow(unused_imports)]

use super::*;

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn mask(b: &mut Bencher)
{
    b.iter(|| {
	for i in 0..0o777u32 {
	    let bits = Permissions::from(i);
	    black_box(bits);
	}
    });
}


#[bench]
fn from_mask(b: &mut Bencher)
{
    b.iter(|| {
	for i in 0..0o777u32 {
	    let bits = Permissions::from(i);
	    black_box(u32::from(bits));
	}
    });
}

