use ramhorns::Content;
use crate::structures::skill::Skill;
use crate::structures::time_point::TimePoint;

#[derive(Content)]
pub struct CV<'a> {
    //Name
    pub(crate) first_name: &'a str,
    pub(crate) last_name: &'a str,
    //Contact
    pub(crate) phone_number: &'a str,
    pub(crate) email_address: &'a str,
    //programing skills
    pub(crate) skills: Vec<Skill<'a>>,
    //languages
    pub(crate) languages: Vec<Skill<'a>>,
    
    //timepoints
    pub(crate) time_stamps: Vec<TimePoint>,

    //color
    pub(crate) color: &'a str,
}