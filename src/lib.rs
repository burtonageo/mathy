#![feature(phase)]

extern crate core;

#[phase(link, plugin)]
extern crate printerr;

use core::fmt::Show;

use std::from_str::FromStr;
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

pub fn display_average_from_os_args<T: FromStr + Num + Show>(avg: |Vec<T>| -> Option<T>) {
    match avg(os_args_to_numbers()) {
        Some(n) => println!("{}", n),
        None => {
                    err_println!("mean: could not calculate average from arguments");
                    os::set_exit_status(1);
                }
    };
}