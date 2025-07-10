use crate::structures::cv::CV;
use crate::structures::skill::Skill;
use crate::structures::time_point::TimePoint;
use clap::{ArgAction, Parser};
use std::string::String;

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

    /// Programming Skills as a list. Add ass many skills as you like via a flag: --skill "skill_name=Java,rating=5"
    #[arg(long)]
    pub skill: Vec<String>,

    /// Speaking languages as a list. Add ass many skills as you like via a flag: --language "skill_name=English,rating=5"
    #[arg(long)]
    pub language: Vec<String>,

    /// Color from you CV. Default #007bff (blue)
    #[arg(long)]
    pub color: Option<String>,

    /// Timeline only set when you want to generate a second page with timeline. Add ass many timepoints as you like via flag: --time_point "type=1,title=Uni,description=Studium,date=2015-2020,location=München,space=false"
    #[arg(long)]
    pub time_point: Vec<String>,

    /// activate Debug
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub debug: bool,
}
impl Cli {
    pub fn validate(&mut self) -> bool {
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

            if self.color.is_none() {
                self.color = Some("#007bff".to_string());
            }

            if !validate {
                eprintln!("Run--help for help ");
                std::process::exit(1);
            }
            return true;
        }
        false
    }

    pub fn get_cv_out_of_args(&self) -> CV {
        let skills: Vec<Skill> = Self::get_skill_list(self.skill.clone());
        let languages: Vec<Skill> = Self::get_skill_list(self.language.clone());
        let time_stamps: Vec<TimePoint> = Self::get_time_line_list(self.time_point.clone());

        CV {
            first_name: self.first_name.clone().unwrap(),
            last_name: self.last_name.clone().unwrap(),
            phone_number: self.phone_number.clone().unwrap(),
            email_address: self.email_address.clone().unwrap(),
            skills,
            languages,
            time_stamps,
            color: self.color.clone().unwrap(),
        }
    }

    /*
       TimePoint List parsing from CLI input to Valid TimePoint Vec
    */
    fn get_time_line_list(string_list: Vec<String>) -> Vec<TimePoint> {
        string_list
            .iter()
            .map(|i| {
                let timepoint: TimePoint = Self::parse_time_point(i).unwrap_or_else(|err| {
                    panic!("ERROR while parsing your time_point flag:  {}", err)
                });
                timepoint
            })
            .collect()
    }
    fn parse_time_point(s: &str) -> Result<TimePoint, String> {
        let mut type_: i8 = 0;
        let mut titel = String::new();
        let mut description = String::new();
        let mut date = String::new();
        let mut location = String::new();
        let mut space: bool = false;
 

        for kv in s.split(',') {
            let mut parts = kv.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();
            match key {
                "type" => {
                    type_ = Some(
                        value
                            .parse::<i8>()
                            .expect("Can't convert type in timepoint flag"),
                    )
                    .unwrap();
                }
                "title" => titel = value.to_string(),
                "description" => description = value.to_string(),
                "date" => date = value.to_string(),
                "location" => location = value.to_string(),
                "space" => {
                    space = Some(
                        value
                            .parse::<bool>()
                            .expect("Can't convert type in timepoint flag"),
                    )
                        .unwrap();
                }
                _ => {}
            }
        }

        Ok(TimePoint::new(
            type_,
            titel,
            description,
            date,
            location,
            space,
        ).convert_type_to_svg())
    }

    /*
       Skill List parsing from CLI input to Valid Skill Vec
    */
    fn get_skill_list(string_list: Vec<String>) -> Vec<Skill> {
        string_list
            .iter()
            .map(|i| {
                let skill = Self::parse_skill(i)
                    .unwrap_or_else(|e| panic!("ERROR while parsing your skill flag: {}", e));
                skill
            })
            .collect()
    }
    fn parse_skill(s: &str) -> Result<Skill, String> {
        let mut name = None;
        let mut rating = None;

        for part in s.split(',') {
            let mut kv = part.splitn(2, '=');
            let key = kv.next().unwrap_or("").trim();
            let value = kv.next().unwrap_or("").trim();

            match key {
                "skill_name" => name = Some(value.to_string()),
                "rating" => {
                    rating = Some(Skill::format_rating(
                        i32::from_str_radix(value, 10).unwrap(),
                    ))
                }
                _ => return Err(format!("Unknown field: {}", key)),
            }
        }

        Ok(Skill {
            skill_name: name.ok_or("skill_name missing")?,
            rating: rating.ok_or("rating missing")?,
        })
    }
}
