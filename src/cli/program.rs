use std::process;

use crate::cli::arguments::read_arguments;

pub fn run() {
    //Takes appropriate action based on arguments passed ex. rssg --input
    if read_arguments().is_err() {
        process::exit(0);
    }
}
