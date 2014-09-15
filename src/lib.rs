#![feature(phase)]

extern crate core;

#[phase(link, plugin)]
extern crate printerr;

use core::fmt::Show;

use std::from_str::FromStr;
use std::num::{cast, zero, Zero};
use std::os;

pub fn os_args_to_numbers<T: FromStr + Num>() -> Vec<T> {
    let args = os::args();
    args.iter()
        .skip(1) // skip the program name
        .filter_map(|a| match from_str(a.as_slice()) {
                            None => {
                                let err_msg = format!("{}: bad args: {}", args[0], a);
                                err_println!("{}", err_msg);
                                None
                            },
                            Some(x) => Some(x)
                        })
        .collect()
}

pub fn display_numeric_fn_result_from_os_args<T: FromStr + Num + Show>(f: |Vec<T>| -> Option<T>) {
    match f(os_args_to_numbers()) {
        Some(n) => println!("{}", n),
        None => {
                    err_println!("{}: could not calculate average from arguments", os::args()[0]);
                    os::set_exit_status(1);
                }
    };
}

pub fn mean<T: Float>(nums: Vec<T>) -> Option<T> {
    let sum = nums.iter().fold(zero::<T>(), |a, b| a.add(b));
    let len = match cast::<uint, T>(nums.len()) {
                  Some(l) if !l.is_zero() => l,
                  _ => return None
              };

    Some(sum / len)
}

pub fn median<T: Float>(nums: Vec<T>) -> Option<T> {
    let med_idx = match nums.len() {
                      l if !l.is_zero() => if l % 2 == 0 {
                                               l / 2 
                                           } else {
                                               (l / 2) + 1
                                           },
                      _ => return None
                  };
    Some(nums[med_idx])
}

pub fn mode<T: Float>(_nums: Vec<T>) -> Option<T> {
    unimplemented!();
}

fn reduce<T: Float>(nums: Vec<T>, f: |T, &T| -> T) -> Option<T> {
    match nums.len() {
        0 => None,
        _ => Some(nums.iter().fold(zero::<T>(), f))
    }
}

pub fn sum<T: Float>(nums: Vec<T>) -> Option<T> {
    reduce(nums, |a, b| a.add(b))
}

pub fn sub<T: Float>(nums: Vec<T>) -> Option<T> {
    reduce(nums, |a, b| a.sub(b))
}

pub fn div<T: Float>(nums: Vec<T>) -> Option<T> {
    reduce(nums, |a, b| a.div(b))
}

pub fn mul<T: Float>(nums: Vec<T>) -> Option<T> {
    reduce(nums, |a, b| a.mul(b))
}