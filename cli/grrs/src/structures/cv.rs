use ramhorns::Content;
use crate::structures::skill::Skill;
use crate::structures::time_point::TimePoint;

#[derive(Content)]
pub struct CV {
    //Name
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    //Contact
    pub(crate) phone_number: String,
    pub(crate) email_address: String,
    //programing skills
    pub(crate) skills: Vec<Skill>,
    //languages
    pub(crate) languages: Vec<Skill>,
    
    //timepoints
    pub(crate) time_stamps: Vec<TimePoint>,
    
    //cover letter
    pub(crate) cover_letter: String,
    pub(crate) job: String,

    //color
    pub(crate) color: String,
}