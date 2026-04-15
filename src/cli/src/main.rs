mod generator;
mod merger;
mod structures;

use generator::generator as gen;

use crate::generator::cv_manual::get_cv_manual;
use crate::structures::cli::Cli;
use crate::structures::cv::CV;
use clap::Parser;

fn main() {
    
    
    let mut cli_args = Cli::parse();
    let use_cli = cli_args.validate();

    let cv_data: CV = if !use_cli {
        get_cv_manual()
    } else {
        cli_args.get_cv_out_of_args()
    };

    gen::generate(cv_data);
}
