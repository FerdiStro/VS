use clap::{ArgAction, Parser};
use crate::structures::cv::CV;

#[derive(Parser, Debug)]
#[command(
    name = "CV-generator",
    version = "1.0",
    about = "This CLI allows you to create a professional CV tailored to your programming career. To get started, you need to install Rust and Docker Compose.",
    long_about = None
)]
pub struct Cli {
    /// When set to true flags CLI flags mode active and more flags are needed
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub cli: bool,

    /// Your first name     [Required when cli]
    #[arg(long)]
    pub first_name: Option<String>,

    /// Your last name      [Required when cli]
    #[arg(long)]
    pub last_name: Option<String>,

    /// Your phone Number   [Required when cli]
    #[arg(long)]
    pub phone_number: Option<String>,

    /// Your E-Mail         [Required when cli]
    #[arg(long)]
    pub email_address: Option<String>,

    /// Color from you CV. Default #007bff (blue)
    #[arg(long)]
    pub color: Option<String>,

    /// activate Debug
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub debug: bool,
}
impl Cli {
    pub fn validate(&self) -> bool {
        if self.cli {
            let mut validate: bool = true;

            if self.first_name.is_none() {
                validate = false;
                eprintln!("Error: --first-name nee to be set when used cli only mode");
            }
            if self.last_name.is_none() {
                validate = false;

                eprintln!("Error: --last-name nee to be set when used cli only mode");
            }
            if self.phone_number.is_none() {
                validate = false;

                eprintln!("Error: --phone-number nee to be set when used cli only mode");
            }
            if self.email_address.is_none() {
                validate = false;

                eprintln!("Error: --email-address nee to be set when used cli only mode");
            }

            if !validate{
                eprintln!("Run--help for help ");
                std::process::exit(1);
            }
        return true;
        }
        false
    }

    pub  fn get_cv_out_of_args(&self) -> CV{

    }
}
