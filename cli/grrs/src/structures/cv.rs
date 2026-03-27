use crate::structures::skill::Skill;
use crate::structures::time_point::TimePoint;
use ramhorns::Content;

#[derive(Content)]
pub struct CV {
    //Name
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    //Contact
    pub(crate) phone_number: String,
    pub(crate) email_address: String,
    pub(crate) website: String,

    //CV value
    pub(crate) job_experience: String,
    pub(crate) about_me: String,

    //programing skills
    pub(crate) skills: Vec<Skill>,
    //languages
    pub(crate) languages: Vec<Skill>,

    //timepoints
    pub(crate) time_stamps: Vec<TimePoint>,

    //cover letter
    pub(crate) cover_letter: String,
    pub(crate) cover_merged: bool,
    pub(crate) job: String,

    //color
    pub(crate) color: String,
}
