use ramhorns::Content;

#[derive(Content)]
pub struct Skill<'a> {
    pub(crate) skill_name: &'a str,
    pub(crate) rating: String,
}