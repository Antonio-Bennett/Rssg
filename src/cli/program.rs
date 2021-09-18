use crate::cli::arguments::read_arguments;

pub fn run() {
    //Takes appropriate action based on arguments passed ex. rssg --input
    read_arguments().unwrap();
}
