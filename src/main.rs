#![recursion_limit = "1024"]

extern crate rand;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate downcast_rs;
extern crate unicode_normalization;
extern crate libc;
extern crate regex;
extern crate chrono;
extern crate glob;
extern crate fnv;
extern crate png;
extern crate isbn;
extern crate titlecase;
extern crate clap;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            ParseInt(::std::num::ParseIntError);
        }
        links {
            Font(::font::Error, ::font::ErrorKind);
        }
    }
}

#[macro_use]
mod geom;
mod unit;
mod color;
mod device;
mod framebuffer;
mod frontlight;
mod lightsensor;
mod battery;
mod input;
mod gesture;
mod helpers;
mod document;
mod metadata;
mod symbolic_path;
mod settings;
mod trash;
mod view;
mod font;
mod app;
use clap::{Arg, App};
use std::str::FromStr;
use app::run;

fn main() {
    let matches = App::new("Plato")
                    .version("0.4.0")
                    .about("A document reader originally for Kobo eReaders, and now for reMarkable")
                    .arg(Arg::with_name("width").short("w").long("width").takes_value(true))
                    .arg(Arg::with_name("height").short("h").long("height").takes_value(true))
                    .arg(Arg::with_name("h_offset").short("ho").long("h_offset").takes_value(true))
                    .arg(Arg::with_name("v_offset").short("vo").long("v_offset").takes_value(true))
                    .get_matches();
    let width = f32::from_str(matches.value_of("width").unwrap_or("1")).unwrap();
    let height = f32::from_str(matches.value_of("height").unwrap_or("1")).unwrap();
    let h_offset = f32::from_str(matches.value_of("h_offset").unwrap_or("0")).unwrap();
    let v_offset = f32::from_str(matches.value_of("v_offset").unwrap_or("0")).unwrap();
    run(width, height, h_offset, v_offset);
}