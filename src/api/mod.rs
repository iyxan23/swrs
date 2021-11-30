pub mod screen;
pub mod view;
pub mod block;
pub mod component;

use std::collections::HashMap;
use crate::api::view::View;
use crate::color::Color;
use crate::error::SWRSError;
use crate::parser::RawSketchwareProject;
use crate::parser::SketchwareProject as ParsedSketchwareProject;

/// A model that holds a metadata of a project. like its name, package name, etc.
pub struct Metadata {
    /// The app name of the project
    pub name: String,

    /// The workspace name of the project (also known as a project name); The name that appears on
    /// the actionbar on the editor when you're editing the project
    pub workspace_name: String,
    pub package_name: String,

    /// Timestamp of the time this project was created, in seconds
    pub time_created: u64,

    pub sketchware_version: u8,
    pub version_name: String,
    pub version_code: u16,
}

/// A model that stores color values of a project
pub struct Colors {
    pub color_primary: Color,
    pub color_primary_dark: Color,
    pub color_accent: Color,
    pub color_control_normal: Color,
    pub color_control_highlight: Color,
}

/// A model that stores libraries' information of a project
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

/// A model that represents a custom view
pub struct CustomView {
    pub res_name: String,
    pub view: View,
}

/// A sketchware project
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SketchwareProject {
    pub metadata: Metadata,
    pub colors: Colors,
    pub screens: Vec<screen::Screen>,
    pub custom_views: Vec<CustomView>,
    pub libraries: Libraries,

    /// A map of resource name (the name defined in the res folder) and resource full name (the
    /// actual filename)
    ///
    /// todo: actually implement a resource system
    pub resources: HashMap<String, String>,
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
        // todo: screens, customviews, libraries, resources
        Ok(SketchwareProject {
            metadata: Metadata {
                name: val.project.app_name,
                workspace_name: val.project.workspace_name,
                package_name: val.project.package_name,
                time_created: val.project.date_created,
                sketchware_version: val.project.sketchware_version,
                version_name: val.project.version_name,
                version_code: val.project.version_code
            },
            colors: Colors {
                color_primary: val.project.color_palette.color_primary,
                color_primary_dark: val.project.color_palette.color_primary_dark,
                color_accent: val.project.color_palette.color_accent,
                color_control_normal: val.project.color_palette.color_control_normal,
                color_control_highlight: val.project.color_palette.color_control_highlight,
            },
            screens: vec![],
            custom_views: vec![],
            libraries: Libraries {
                app_compat_enabled: val.library.compat.use_yn == "Y",
                firebase: None,
                ad_mob: None,
                google_map: None
            },
            resources: HashMap::new()
        })
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