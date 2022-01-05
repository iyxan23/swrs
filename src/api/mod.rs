pub mod screen;
pub mod view;
pub mod block;
pub mod component;

use crate::LinkedHashMap;
use crate::api::library::{AdMob, Firebase, GoogleMap};
use crate::api::screen::Screen;
use crate::api::view::{parse_raw_layout, View};
use crate::color::Color;
use crate::error::SWRSError;
use crate::{parser, SWRSResult};
use crate::parser::logic::ScreenLogic;
use crate::parser::RawSketchwareProject;
use crate::parser::resource::{Resource, ResourceItem};
use crate::parser::SketchwareProject as ParsedSketchwareProject;
use crate::parser::view::Layout;

/// A model that holds a metadata of a project. like its name, package name, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    /// The local ID of this project, should not be used for transferring sketchware projects to
    /// other devices
    ///
    /// Has a maximum value of 999, can go down to 0 but starts at 600
    pub local_id: u16,

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
#[derive(Debug, Clone, PartialEq)]
pub struct Colors {
    pub color_primary: Color,
    pub color_primary_dark: Color,
    pub color_accent: Color,
    pub color_control_normal: Color,
    pub color_control_highlight: Color,
}

/// A model that stores libraries' information of a project
#[derive(Debug, Clone, PartialEq)]
pub struct Libraries {
    pub app_compat_enabled: bool,
    pub firebase: Option<library::Firebase>,
    pub ad_mob: Option<library::AdMob>,
    pub google_map: Option<library::GoogleMap>,
}

mod library {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Firebase {
        pub project_id: String,     // key: data
        pub app_id: String,         // key: reserved1
        pub api_key: String,        // key: reserved2
        pub storage_bucket: String, // key: reserved3
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AdMob {
        pub ad_units: Vec<crate::parser::library::AdUnit>,  // key: adUnits
        pub test_devices: Vec<String>,      // key: testDevices
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GoogleMap {
        pub api_key: String,        // key: data
    }
}

/// A model that represents a custom view
#[derive(Debug, Clone, PartialEq)]
pub struct CustomView {
    pub res_name: String,
    pub layout: Vec<View>,
}

/// A model that stores data of resources
///
/// Each `LinkedHashMap`s are a map of resource name (the name defined in the res folder) and
/// resource full name (the actual filename)
// todo: actually implement a resource system
#[derive(Debug, Clone, PartialEq)]
pub struct Resources {
    pub images: LinkedHashMap<String, String>,
    pub sounds: LinkedHashMap<String, String>,
    pub fonts: LinkedHashMap<String, String>,
}

/// A sketchware project
#[derive(Debug, Clone, PartialEq)]
pub struct SketchwareProject {
    pub metadata: Metadata,
    pub colors: Colors,
    pub screens: Vec<screen::Screen>,
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

    fn try_from(mut val: ParsedSketchwareProject) -> Result<Self, Self::Error> {
        macro_rules! library_conv {
            ($str_name:expr, $parsed_field_name:ident, $result:expr) => {{
                match val.library.$parsed_field_name.use_yn.as_str() {
                    "Y" => Some($result),
                    "N" => None,
                    _ => return Err(SWRSError::ParseError(format!(
                        "use_yn of {} library contains an invalid value: {}", $str_name, val.library.firebase_db.use_yn
                    ))),
                }
            }};
        }

        macro_rules! resources_conv {
            ($res_name:ident) => {{
                val.resource.$res_name
                    .drain(..)
                    .map(|ResourceItem { full_name, name, .. }| (name, full_name))
                    .collect()
            }}
        }

        // get the activities
        let activities = val.file.activities.into_iter()
            .map(|file_entry| {
                let name = file_entry.filename.to_owned();

                // get the activity layout and logic
                let layout = val.view.layouts
                    .remove(name.as_str())
                    .unwrap_or_else(||Layout(vec![]));
                    // return an empty layout if there is no layout definition for it

                // if it can't find any logic then create an empty ScreenLogic
                let logic = val.logic.screens
                    .remove(name.as_str())
                    .unwrap_or_else(||ScreenLogic::new_empty(file_entry.filename.clone()));

                // get our fab (if we have one)
                let fab = val.view.fabs
                    .remove(name.as_str())
                    .map(|view| View::try_from(view))

                    // flip from Option<Result<>> to Result<Option<>>
                    .map_or(Ok(None), |v| v.map(Some))?;

                Screen::from_parsed(
                    file_entry.filename.to_owned(),
                    logic.name.to_owned(),
                    file_entry,
                    layout,
                    logic,
                    fab
                ).map_err(|err|SWRSError::ParseError(format!(
                    "Failed to convert a raw screen of {} to screen:\n{}", name, err
                )))
            })
            .collect::<SWRSResult<Vec<Screen>>>()?;

        // and get the custom views
        let custom_views = val.file.custom_views.into_iter()
            .map(|file_entry| {
                // retrieve the layout of this custom view
                let layout = val.view.layouts
                    .remove(file_entry.filename.as_str())
                    .ok_or_else(||SWRSError::ParseError(format!(
                        "Unable to find layout of custom view {}", file_entry.filename
                    )))?;

                Ok(CustomView {
                    res_name: file_entry.filename.to_owned(),
                    layout: parse_raw_layout(layout)
                        .map_err(|err|SWRSError::ParseError(format!(
                            "Failed to convert raw layout into a single view of customview {}:\n{}",
                            file_entry.filename, err
                        )))?
                })
            })
            .collect::<SWRSResult<Vec<CustomView>>>()?;

        Ok(SketchwareProject {
            metadata: Metadata {
                local_id: val.project.id,
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
            screens: activities,
            custom_views,
            libraries: Libraries {
                app_compat_enabled: val.library.compat.use_yn == "Y",
                firebase: library_conv!("firebase", firebase_db, Firebase {
                    project_id: val.library.firebase_db.data,
                    app_id: val.library.firebase_db.reserved1,
                    api_key: val.library.firebase_db.reserved2,
                    storage_bucket: val.library.firebase_db.reserved3,
                }),
                ad_mob: library_conv!("admob", admob, AdMob {
                    ad_units: val.library.admob.ad_units,
                    test_devices: val.library.admob.test_devices,
                }),
                google_map: library_conv!("google map", google_map, GoogleMap {
                    api_key: val.library.google_map.data,
                }),
            },
            resources: Resources {
                images: resources_conv!(images),
                sounds: resources_conv!(sounds),
                fonts: resources_conv!(fonts),
            }
        })
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SWRSError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        ParsedSketchwareProject::try_from(self)?.try_into()
    }
}

impl TryFrom<SketchwareProject> for ParsedSketchwareProject {
    type Error = SWRSError;

    fn try_from(val: SketchwareProject) -> Result<Self, Self::Error> {
        macro_rules! resource_conv {
            ($name:ident) => {{
                val.resources.$name
                    .drain()
                    .map(|(full_name, name)| ResourceItem { full_name, name, r#type: 1 })
                    .collect()
            }}
        }

        Ok(ParsedSketchwareProject {
            project: parser::project::Project {
                id: val.metadata.local_id,
                app_name: val.metadata.name,
                workspace_name: val.metadata.workspace_name,
                package_name: val.metadata.package_name,
                version_code: val.metadata.version_code,
                version_name: val.metadata.version_name,
                date_created: val.metadata.time_created,
                custom_icon: todo!("figure out where the custom icon is, and provide a field that stores the custom icon"),
                color_palette: parser::project::ProjectColorPalette {
                    color_primary: val.colors.color_primary,
                    color_primary_dark: val.colors.color_primary_dark,
                    color_accent: val.colors.color_accent,
                    color_control_normal: val.colors.color_control_normal,
                    color_control_highlight: val.colors.color_control_highlight,
                },
                sketchware_version: val.metadata.sketchware_version,
            },
            file: parser::file::File { activities: vec![todo!()], custom_views: vec![todo!()] },
            library: parser::library::Library {
                firebase_db: match val.libraries.firebase {
                    Some(val) => parser::library::LibraryItem {
                        ad_units: vec![],
                        data: val.project_id,
                        lib_type: 0,
                        reserved1: val.app_id,
                        reserved2: val.api_key,
                        reserved3: val.storage_bucket,
                        test_devices: vec![],
                        use_yn: "Y".to_string()
                    },
                    None => parser::library::LibraryItem {
                        ad_units: vec![],
                        data: "".to_string(),
                        lib_type: 0,
                        reserved1: "".to_string(),
                        reserved2: "".to_string(),
                        reserved3: "".to_string(),
                        test_devices: vec![],
                        use_yn: "N".to_string()
                    }
                },
                compat: parser::library::LibraryItem {
                    ad_units: vec![],
                    data: "".to_string(),
                    lib_type: 1,
                    reserved1: "".to_string(),
                    reserved2: "".to_string(),
                    reserved3: "".to_string(),
                    test_devices: vec![],
                    use_yn: if val.libraries.app_compat_enabled { "Y" } else { "N" }.to_string()
                },
                admob: match val.libraries.ad_mob {
                    Some(val) => parser::library::LibraryItem {
                        ad_units: val.ad_units,
                        data: "".to_string(),
                        lib_type: 2,
                        reserved1: "".to_string(),
                        reserved2: "".to_string(),
                        reserved3: "".to_string(),
                        test_devices: val.test_devices,
                        use_yn: "Y".to_string()
                    },
                    None => parser::library::LibraryItem {
                        ad_units: vec![],
                        data: "".to_string(),
                        lib_type: 2,
                        reserved1: "".to_string(),
                        reserved2: "".to_string(),
                        reserved3: "".to_string(),
                        test_devices: vec![],
                        use_yn: "N".to_string()
                    }
                },
                google_map: match val.libraries.google_map {
                    Some(val) => parser::library::LibraryItem {
                        ad_units: vec![],
                        data: val.api_key,
                        lib_type: 3,
                        reserved1: "".to_string(),
                        reserved2: "".to_string(),
                        reserved3: "".to_string(),
                        test_devices: vec![],
                        use_yn: "Y".to_string()
                    },
                    None => parser::library::LibraryItem {
                        ad_units: vec![],
                        data: "".to_string(),
                        lib_type: 3,
                        reserved1: "".to_string(),
                        reserved2: "".to_string(),
                        reserved3: "".to_string(),
                        test_devices: vec![],
                        use_yn: "N".to_string()
                    }
                },
            },
            resource: Resource {
                images: resource_conv!(images),
                sounds: resource_conv!(sounds),
                fonts: resource_conv!(fonts)
            },
            view: parser::view::View { layouts: Default::default(), fabs: Default::default() },
            logic: parser::logic::Logic { screens: Default::default() },
        })
    }
}