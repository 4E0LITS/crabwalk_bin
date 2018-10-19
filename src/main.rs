extern crate getopts;
use getopts::{
    Options,
    ParsingStyle,
};

use std::env::{args, Args};

fn main() {
    
}

fn params() {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::FloatingFrees)
        .optflag("h", "help", "this")
        .optopt("p", "port", "requested port, default 80", "PORT");

}