use ramhorns::Content;

#[derive(Debug, Clone, Content)]
pub struct TimePoint {
    time_point_type: i8,

    pub(crate) titel: String,
    pub(crate) description: String,
    pub(crate) date: String,
    pub(crate) location: String,
    pub(crate) space: bool,
    time_point_svg: String,
    
}

impl TimePoint {
    pub const SVG_TYPE_1: &'static str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20px\" height=\"20px\" viewBox=\"0 0 24 24\" fill=\"none\">
                <path d=\"M22 22L2 22\" stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>
                <path d=\"M2 11L6.06296 7.74968M22 11L13.8741 4.49931C12.7784 3.62279 11.2216 3.62279 10.1259 4.49931L9.34398 5.12486\"
                      stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>
                <path d=\"M15.5 5.5V3.5C15.5 3.22386 15.7239 3 16 3H18.5C18.7761 3 19 3.22386 19 3.5V8.5\"
                      stroke=\"#1C274C\"
                      stroke-width=\"1.5\" stroke-linecap=\"round\"/>
                <path d=\"M4 22V9.5\" stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>
                <path d=\"M20 9.5V13.5M20 22V17.5\" stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\"/>
                <path d=\"M15 22V17C15 15.5858 15 14.8787 14.5607 14.4393C14.1213 14 13.4142 14 12 14C10.5858 14 9.87868 14 9.43934 14.4393M9 22V17\"
                      stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>
                <path d=\"M14 9.5C14 10.6046 13.1046 11.5 12 11.5C10.8954 11.5 10 10.6046 10 9.5C10 8.39543 10.8954 7.5 12 7.5C13.1046 7.5 14 8.39543 14 9.5Z\"
                      stroke=\"#1C274C\" stroke-width=\"1.5\"/>
            </svg>";
    pub const SVG_TYPE_2: &'static str = " <svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20px\" height=\"20px\" viewBox=\"0 0 24 24\" fill=\"none\">
                <path d=\"M6 11.4999H7M6 15.4999H7M17 15.4999H18M17 11.4999H18M11.5 11.4999H12.5M10 20.9999V16.9999C10 15.8954 10.8954 14.9999 12 14.9999C13.1046 14.9999 14 15.8954 14 16.9999V20.9999M17 7.49995L18.5761 7.89398C19.4428 8.11064 19.8761 8.21898 20.1988 8.46057C20.4834 8.67373 20.7061 8.95895 20.8439 9.28682C21 9.65843 21 10.1051 21 10.9984V17.7999C21 18.9201 21 19.4801 20.782 19.9079C20.5903 20.2843 20.2843 20.5902 19.908 20.782C19.4802 20.9999 18.9201 20.9999 17.8 20.9999H6.2C5.0799 20.9999 4.51984 20.9999 4.09202 20.782C3.71569 20.5902 3.40973 20.2843 3.21799 19.9079C3 19.4801 3 18.9201 3 17.7999V10.9984C3 10.1051 3 9.65843 3.15613 9.28682C3.29388 8.95895 3.51657 8.67373 3.80124 8.46057C4.12389 8.21898 4.55722 8.11064 5.42388 7.89398L7 7.49995L9.85931 4.92657C10.6159 4.2456 10.9943 3.90512 11.4221 3.77598C11.799 3.66224 12.201 3.66224 12.5779 3.77598C13.0057 3.90512 13.3841 4.2456 14.1407 4.92657L17 7.49995Z\"
                      stroke=\"#1C274C\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>
            </svg>";
    pub const SVG_TYPE_3: &'static str = "          <svg xmlns=\"http://www.w3.org/2000/svg\" style=\"display: inline\" width=\"20px\" height=\"20px\"
                 viewBox=\"0 0 32 32\" data-name=\"Layer 1\" id=\"Layer_1\">
                <defs>
                    <style>.cls-1 {
                        fill: #1C274C;
                    }</style>
                </defs>
                <title/>
                <path class=\"cls-1\"
                      d=\"M28,8H21V6a2,2,0,0,0-2-2H13a2,2,0,0,0-2,2V8H4a2,2,0,0,0-2,2V26a2,2,0,0,0,2,2H28a2,2,0,0,0,2-2V10A2,2,0,0,0,28,8ZM13,6h6V8H13Zm15,4v9H4V10ZM4,26V21H28v5Z\"/>
                <path class=\"cls-1\" d=\"M15,18h2a1,1,0,0,0,0-2H15a1,1,0,0,0,0,2Z\"/>
            </svg>";

    pub fn new(time_point_type: i8, titel: String, description: String, date: String, location: String, space: bool) -> Self {
        TimePoint {
            time_point_type,
            titel,
            description,
            date,
            time_point_svg: "NULL".to_string(),
            location,
            space
        }
    }

    pub fn convert_type_to_svg(&self) -> TimePoint {
        let mut time_point_type_as_svg: String = String::new();

        if self.time_point_type == 0 {
            time_point_type_as_svg = Self::SVG_TYPE_1.parse().unwrap();
        }
        if self.time_point_type == 1 {
            time_point_type_as_svg = Self::SVG_TYPE_2.parse().unwrap();
        }

        if self.time_point_type == 2 {
            time_point_type_as_svg = Self::SVG_TYPE_3.parse().unwrap();
        }

        TimePoint {
            time_point_type: self.time_point_type,
            time_point_svg: time_point_type_as_svg,
            titel: self.titel.clone(),
            description: self.description.clone(),
            date: self.date.clone(),
            location: self.location.clone(),
            space: self.space.clone(),
        }
    }
}
