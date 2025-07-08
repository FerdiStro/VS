use ramhorns::Content;
use crate::structures::skill::Skill;
use crate::structures::time_point::TimePoint;

#[derive(Content)]
pub struct CV<'a> {
    //Name
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    //Contact
    pub(crate) phone_number: String,
    pub(crate) email_address: String,
    //programing skills
    pub(crate) skills: Vec<Skill<'a>>,
    //languages
    pub(crate) languages: Vec<Skill<'a>>,
    
    //timepoints
    pub(crate) time_stamps: Vec<TimePoint>,

    //color
    pub(crate) color: String,
}