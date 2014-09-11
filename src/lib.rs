#![feature(phase)]

#[phase(link, plugin)]
extern crate printerr;

use std::from_str::FromStr;
use std::os;

pub fn os_args_to_numbers<T: Num + FromStr>() -> Vec<T> {
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