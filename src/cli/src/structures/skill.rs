use ramhorns::Content;

#[derive(Content)]
pub struct Skill {
    pub(crate) skill_name: String,
    pub(crate) rating: String,
}
impl Skill {
    pub fn format_rating(rating: i32) -> String {
        let filled = "<div class=\"rounded-circle-fill\"> </div>".repeat(rating as usize);
        let empty = "<div class=\"rounded-circle-non-fill\"> </div>".repeat(5 - rating as usize);
        format!("{}{}", filled, empty)
    }
}
