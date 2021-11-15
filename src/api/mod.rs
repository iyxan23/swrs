use crate::color::Color;
use crate::error::SWRSError;
use crate::parser::RawSketchwareProject;
use crate::parser::SketchwareProject as ParsedSketchwareProject;

pub struct Blocks {

}

pub struct Layout {

}

pub struct Screen {
    pub layout: Layout,
    pub blocks: Blocks,
}

pub struct CustomView {

}

pub struct Metadata {
    pub name: String,
    pub project_name: String,
}

pub struct Colors {
    pub color_primary: Color,
    pub color_primary_dark: Color,
    pub color_accent: Color,
    pub color_control_normal: Color,
    pub color_control_highlight: Color,
}

pub struct Libraries {
    pub app_compat_enabled: bool,
    pub firebase: Option<library::Firebase>,
    pub ad_mob: Option<library::AdMob>,
    pub google_map: Option<library::GoogleMap>,
}

mod library {
    pub struct Firebase {
        pub project_id: String,     // key: data
        pub app_id: String,         // key: reserved1
        pub api_key: String,        // key: reserved2
        pub storage_bucket: String, // key: reserved3
    }

    pub struct AdMob {
        pub ad_units: Vec<ad_mob::AdUnit>,  // key: adUnits
        pub test_devices: Vec<String>,      // key: testDevices
    }

    pub mod ad_mob {
        pub struct AdUnit {
            pub id: String,
            pub name: String,
        }
    }

    pub struct GoogleMap {
        pub api_key: String,        // key: data
    }
}

pub struct Resources {

}

pub struct SketchwareProject {
    pub metadata: Metadata,
    pub colors: Colors,
    pub screens: Vec<Screen>,
    pub custom_views: Vec<CustomView>,
    pub libraries: Libraries,
    pub resources: Resources,
}

impl TryFrom<RawSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_from(val: RawSketchwareProject) -> Result<Self, Self::Error> {
        SketchwareProject::try_from(ParsedSketchwareProject::parse_from(val)?)
    }
}

impl TryFrom<ParsedSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_from(val: ParsedSketchwareProject) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        ParsedSketchwareProject::parse_from(SketchwareProject::try_into()?)
    }
}

impl TryInto<ParsedSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_into(self) -> Result<ParsedSketchwareProject, Self::Error> {
        todo!()
    }
}