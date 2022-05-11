pub mod screen;
pub mod view;
pub mod block;
pub mod component;

use std::collections::HashMap;
use std::path::PathBuf;
use crate::LinkedHashMap;
use crate::api::library::{AdMob, Firebase, GoogleMap};
use crate::api::screen::{Event, MoreBlock, Screen, ScreenConstructionError};
use crate::api::view::{flatten_views, parse_raw_layout, ParseLayoutError, View};
use crate::color::Color;
use crate::parser;
use crate::api::component::ComponentKind;
use crate::parser::file::{ActivityOptions, FileItem, FileType, KeyboardSetting, Orientation, Theme};
use crate::parser::logic::component::ComponentPool;
use crate::parser::logic::event::EventPool;
use crate::parser::logic::list_variable::{ListVariable, ListVariablePool};
use crate::parser::logic::more_block::MoreBlockPool;
use crate::parser::logic::ScreenLogic;
use crate::parser::logic::variable::{Variable, VariablePool};
use crate::parser::{ResourceFileWrapper, RawSketchwareProject, ResourceType, SketchwareProjectReconstructionError, ResourceFiles};
use crate::parser::resource::{Resource, ResourceItem};
use crate::parser::SketchwareProject as ParsedSketchwareProject;
use crate::parser::view::Layout;
use crate::parser::view::models::AndroidView;
use thiserror::Error;

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

pub mod library {
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
#[derive(Debug, Clone, PartialEq)]
pub struct Resources {
    images: LinkedHashMap<ResourceId, ResourceFileWrapper>,
    sounds: LinkedHashMap<ResourceId, ResourceFileWrapper>,
    fonts: LinkedHashMap<ResourceId, ResourceFileWrapper>,
}

/// A newtype struct of a resource id, used in [`Resources`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(pub String);

impl Resources {
    /// Retrieves a resource using a [`ResourceId`]
    pub fn get_resource(&self, id: &ResourceId) -> Option<(&ResourceFileWrapper, ResourceType)> {
        if let Some(image) = self.images.get(id) {
            return Some((image, ResourceType::Image));
        }

        if let Some(sound) = self.sounds.get(id) {
            return Some((sound, ResourceType::Sound));
        }

        if let Some(font) = self.fonts.get(id) {
            return Some((font, ResourceType::Font));
        }

        None
    }

    /// Removes a resource from the given [`ResourceId`]
    pub fn remove_resource(&mut self, id: &ResourceId) -> Option<(ResourceFileWrapper, ResourceType)> {
        if let Some(image) = self.images.remove(id) {
            return Some((image, ResourceType::Image));
        }

        if let Some(sound) = self.sounds.remove(id) {
            return Some((sound, ResourceType::Sound));
        }

        if let Some(font) = self.fonts.remove(id) {
            return Some((font, ResourceType::Font));
        }

        None
    }

    /// Puts a given resource to a given resource id and type
    ///
    /// Will not do any path checks (except file existence) on the file given if it is a real file
    /// (as opposed to the TryFrom impl of [`crate::parser::ResourceFiles`])
    ///
    /// `res_type` of the given ResourceFileWrapper will be set to this function's `res_type`
    /// argument
    ///
    /// **BIG NOTE**: `res_full_name` of the given ResourceFileWrapper won't be changed with the
    /// `id` given, make sure you have set it to the correct file name and placed it in a resource
    /// folder where it belongs (images, sounds, fonts)
    pub fn put_resource(
        &mut self,
        id: ResourceId,
        mut file: ResourceFileWrapper,
        res_type: ResourceType
    ) -> Result<(), ResourceAdditionError> {
        if let ResourceType::CustomIcon = res_type {
            panic!("yo can't put a custom icon here (╯°□°）╯︵ ┻━┻"); // small easter egg ig ;)
        }

        // check if file exists if the file is a real file
        if let ResourceFileWrapper::Path(path) = file.to_owned() {
            if !path.exists() {
                return Err(ResourceAdditionError::FileDoesntExist { path });
            }
        }

        // big warning: don't get confused with resource id and file wrapper id

        // re-set the res_type if this file is a StringId
        if let ResourceFileWrapper::StringId { id: file_id, res_full_name, .. } = &file {
            file = ResourceFileWrapper::StringId {
                id: file_id.to_owned(),
                res_full_name: res_full_name.to_owned(),
                res_type
            };
        }

        // re-set the res_type if this file is a U32Id
        if let ResourceFileWrapper::U32Id { id: file_id, res_full_name, .. } = &file {
            file = ResourceFileWrapper::U32Id {
                id: file_id.to_owned(),
                res_full_name: res_full_name.to_owned(),
                res_type
            };
        }

        // check if there is already a resource file with this id
        if self.images.contains_key(&id) {
            return Err(ResourceAdditionError::IdTaken { id });
        }

        match res_type {
            ResourceType::Image => { self.images.insert(id, file); }
            ResourceType::Sound => { self.sounds.insert(id, file); }
            ResourceType::Font  => { self.fonts.insert(id, file); }
            _ => {}
        }

        Ok(())
    }

    pub fn get_images(&self) -> &LinkedHashMap<ResourceId, ResourceFileWrapper> { &self.images }
    pub fn get_sounds(&self) -> &LinkedHashMap<ResourceId, ResourceFileWrapper> { &self.sounds }
    pub fn get_fonts (&self) -> &LinkedHashMap<ResourceId, ResourceFileWrapper> { &self.fonts  }
}

#[derive(Error, Debug)]
pub enum ResourceAdditionError {
    #[error("resource id `{}` is already taken", 0.0)]
    IdTaken { id: ResourceId },

    #[error("file `{path:?}` does not exist")]
    FileDoesntExist { path: PathBuf },
}

/// A sketchware project
#[derive(Debug, Clone, PartialEq)]
pub struct SketchwareProject {
    pub custom_icon: Option<ResourceFileWrapper>,
    pub metadata: Metadata,
    pub colors: Colors,
    pub screens: Vec<screen::Screen>,
    pub custom_views: Vec<CustomView>,
    pub libraries: Libraries,
    pub resources: Resources,

    automatic_res_file_ids: bool
}

// turns a view name to a logic name, something like `main` into `MainActivity`,
// `screen_display` to `ScreenDisplayActivity`
fn view_name_to_logic(s: &str) -> String {
    let mut capitalize = true;

    format!(
        "{}Activity",
        s.chars()
            .into_iter()
            .filter_map(|ch| {
                Some(if ch == '_' {
                    capitalize = true;
                    return None;
                } else if capitalize {
                    capitalize = false;
                    ch.to_ascii_uppercase()
                } else {
                    ch
                })
            })
            .collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use crate::api::view_name_to_logic;

    #[test]
    fn test_view_name_to_logic() {
        assert_eq!("MainActivity", view_name_to_logic("main"));
        assert_eq!("DebugScreenActivity", view_name_to_logic("debug_screen"));
        assert_eq!("VeryLongStringIDontKnowWhyActivity", view_name_to_logic("very_long_string_i_dont_know_why"));
    }
}

impl TryFrom<ParsedSketchwareProject> for SketchwareProject {
    type Error = APISketchwareProjectConversionError;

    fn try_from(mut val: ParsedSketchwareProject) -> Result<Self, Self::Error> {
        macro_rules! library_conv {
            ($str_name:expr, $parsed_field_name:ident, $result:expr) => {{
                match val.library.$parsed_field_name.use_yn.as_str() {
                    "Y" => Some($result),
                    "N" => None,
                    _ => return Err(APISketchwareProjectConversionError::InvalidUseYNValue {
                        library_name: $str_name.to_string(),
                        value: val.library.$parsed_field_name.use_yn.to_string()
                    }),
                }
            }};
        }

        macro_rules! resources_conv {
            ($res_type:ident, $res_type_c:ident) => {{
                val.resource.$res_type
                    .into_iter()
                    .map(|ResourceItem { full_name, name, .. }|{
                        let resource = if let Some(rf) = &mut val.resource_files {
                            rf.$res_type.remove(&full_name)
                                .ok_or_else(||APISketchwareProjectConversionError::MissingResourceFile {
                                    res_name: name.to_owned(),
                                    res_full_name: full_name,
                                    res_type: ResourceType::$res_type_c
                                })?
                        } else {
                            ResourceFileWrapper::make_random_id(full_name, ResourceType::$res_type_c)
                        };

                        Ok((ResourceId(name), resource))
                    })
                    .collect::<Result<LinkedHashMap<_, _>, _>>()?
            }}
        }

        // get the activities
        let activities = val.file.activities.into_iter()
            .try_fold(Vec::new(), |mut acc, file_entry| {
                let name = file_entry.filename.to_owned();

                // get the activity layout and logic
                let layout = val.view.layouts
                    .remove(name.as_str())
                    .unwrap_or_else(||Layout(vec![]));
                    // return an empty layout if there is no layout definition for it

                // if it can't find any logic then create an empty ScreenLogic
                let logic = val.logic.screens
                    .remove(view_name_to_logic(name.as_str()).as_str())
                    .unwrap_or_else(||ScreenLogic::new_empty(file_entry.filename.clone()));

                // get our fab (if we have one)
                let fab = val.view.fabs
                    .remove(name.as_str())
                    .map(View::from);

                acc.push(Screen::from_parsed(
                    file_entry.filename.to_owned(),
                    logic.name.to_owned(),
                    file_entry,
                    layout,
                    logic,
                    fab
                ).map_err(|err| APISketchwareProjectConversionError::ScreenConstructionError {
                    java_screen_name: view_name_to_logic(name.as_str()),
                    layout_screen_name: name,
                    source: err
                })?);

                Ok(acc)
            })?;

        // and get the custom views
        let custom_views = val.file.custom_views.into_iter()
            .try_fold(Vec::new(), |mut acc, file_entry| {
                // retrieve the layout of this custom view
                let layout = val.view.layouts
                    .remove(file_entry.filename.as_str())
                    .ok_or_else(||APISketchwareProjectConversionError::MissingCustomViewLayout {
                        custom_view_id: file_entry.filename.to_owned(),
                    })?;

                acc.push(CustomView {
                    res_name: file_entry.filename.to_owned(),
                    layout: parse_raw_layout(layout)
                        .map_err(|err| APISketchwareProjectConversionError::CustomViewParseLayoutError {
                            custom_view_id: file_entry.filename.to_owned(),
                            source: err
                        })?
                });

                Ok(acc)
            })?;

        Ok(SketchwareProject {
            // there might be a better way of getting a field of T from an Option<T> without needing
            // to own Option<>
            custom_icon: if let Some(rf) = &val.resource_files {
                rf.custom_icon.clone()
            } else { None },
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
                images: resources_conv!(images, Image),
                sounds: resources_conv!(sounds, Sound),
                fonts: resources_conv!(fonts, Font),
            },
            automatic_res_file_ids: val.resource_files.is_none()
        })
    }
}

#[derive(Error, Debug)]
pub enum APISketchwareProjectConversionError {
    #[error("error while reconstructing screen `{layout_screen_name}`")]
    ScreenConstructionError {
        java_screen_name: String,
        layout_screen_name: String,

        source: ScreenConstructionError
    },

    #[error("couldn't find the layout of the custom view `{custom_view_id}`")]
    MissingCustomViewLayout {
        custom_view_id: String
    },

    #[error("error while parsing the layout of custom view `{custom_view_id}`")]
    CustomViewParseLayoutError {
        custom_view_id: String,
        source: ParseLayoutError
    },

    #[error("use_yn of library `{library_name}` has an invalid value `{value}` (expected `Y` or `N`)")]
    InvalidUseYNValue {
        library_name: String,
        value: String
    },

    #[error("couldn't find a resource file matching with id `{res_name}`")]
    MissingResourceFile {
        res_name: String,
        res_full_name: String,
        res_type: ResourceType,
    }
}

impl TryInto<RawSketchwareProject> for SketchwareProject {
    type Error = SketchwareProjectReconstructionError;

    fn try_into(self) -> Result<RawSketchwareProject, Self::Error> {
        // false positive in intellij idea's rust plugin, compiles fine on rust without any warnings
        ParsedSketchwareProject::from(self).try_into()
    }
}

impl From<SketchwareProject> for ParsedSketchwareProject {
    fn from(val: SketchwareProject) -> Self {
        // these hashmaps are filled as the resources are filled
        let mut image_resource_files = HashMap::new();
        let mut sound_resource_files = HashMap::new();
        let mut font_resource_files = HashMap::new();

        macro_rules! resource_conv {
            ($name:ident, $resource_files_hmap:ident) => {{
                val.resources.$name
                    .into_iter()
                    .map(|(id, file)| {
                        let full_name = file.get_full_name();

                        if !val.automatic_res_file_ids {
                            $resource_files_hmap.insert(full_name.to_owned(), file);
                        }

                        ResourceItem {
                            full_name: full_name, name: id.0, r#type: 1
                        }
                    })
                    .collect()
            }}
        }

        let (activities, custom_views): (Vec<FileItem>, Vec<FileItem>) = {
            (val.screens
                .iter()
                .map(|screen| FileItem {
                    filename: screen.layout_name.to_owned(),
                    file_type: FileType::Activity,
                    keyboard_setting: screen.keyboard_setting,
                    options: ActivityOptions {
                        toolbar_enabled: screen.toolbar_enabled,
                        fullscreen_enabled: screen.fullscreen_enabled,
                        drawer_enabled: screen.drawer_enabled,
                        fab_enabled: screen.fab_enabled
                    },
                    orientation: screen.orientation,
                    theme: screen.theme
                }).collect(),
                val.custom_views
                    .iter()
                    .map(|custom_view| FileItem {
                        filename: custom_view.res_name.to_owned(),
                        file_type: FileType::CustomView,
                        keyboard_setting: KeyboardSetting::Unspecified,
                        options: ActivityOptions {
                            toolbar_enabled: false,
                            fullscreen_enabled: false,
                            drawer_enabled: false,
                            fab_enabled: false
                        },
                        orientation: Orientation::Both,
                        theme: Theme::None
                    }).collect()
            )
        };

        fn to_screen_logic(
            logic_name: String,
            variables: LinkedHashMap<String, Variable>,
            list_variables: LinkedHashMap<String, ListVariable>,
            components: LinkedHashMap<String, ComponentKind>,
            more_blocks: LinkedHashMap<String, MoreBlock>,
            events: Vec<Event>,
        ) -> ScreenLogic {
            let mut block_containers = LinkedHashMap::new();

            // separate events to parser event and its blocks
            let parser_events =
                events
                    .into_iter()
                    .filter_map(|event| {
                        let block_container_id = event.get_block_container_id();
                        let (event, blocks) = event.into_parser_event();

                        let block_container = blocks.into();
                        block_containers.insert(block_container_id, block_container);

                        // drop this event if its onCreate, but preserve the block container
                        // see `api::screen::Screen::from_parsed` comments for more explanation
                        if event.event_name == "onCreate" { return None; }

                        Some(event)
                    })
                    .collect::<Vec<_>>();

            // separate moreblocks to parser moreblock and its blocks
            let parser_more_blocks =
                more_blocks
                    .into_iter()
                    .map(|(id, more_block)| {
                        let (more_block, blocks) = more_block.into_parser_more_block();

                        let block_container = blocks.into();
                        block_containers.insert(format!("{}_moreBlock", &id), block_container);

                        (id, more_block)
                    })
                    .collect::<LinkedHashMap<String, _>>();

            ScreenLogic {
                name: logic_name,
                block_containers,
                variables: Some(VariablePool(variables)),
                list_variables: Some(ListVariablePool(list_variables)),
                components: Some(ComponentPool(components
                    .into_iter()
                    .map(|(id, kind)| kind.into_parser_component(id))
                    .collect())),
                events: Some(EventPool(parser_events)),
                more_blocks: Some(MoreBlockPool(parser_more_blocks))
            }
        }

        let (logic_screens, layouts, fabs):
            (LinkedHashMap<String, ScreenLogic>, LinkedHashMap<String, Layout>, LinkedHashMap<String, AndroidView>) = {

            let mut logic_screens = LinkedHashMap::new();
            let mut layouts = LinkedHashMap::new();
            let mut fabs = LinkedHashMap::new();

            for screen in val.screens {
                if let Some(fab) = screen.fab {
                    let mut fab_view = flatten_views(vec![fab], None, None)
                        .remove(0);

                    // these values are specifically set for the fab view
                    fab_view.parent = None;
                    fab_view.parent_type = -1;

                    fabs.insert(screen.layout_name.to_owned(), fab_view);
                }

                logic_screens.insert(screen.java_name.to_owned(), to_screen_logic(
                    screen.java_name,
                    screen.variables,
                    screen.list_variables,
                    screen.components,
                    screen.more_blocks,
                    screen.events,
                ));

                layouts.insert(
                    screen.layout_name,
                    Layout(view::flatten_views(screen.layout, None, None))
                );
            }

            // don't forget about the custom views
            for custom_view in val.custom_views {
                layouts.insert(
                    custom_view.res_name,
                    Layout(view::flatten_views(custom_view.layout, None, None))
                );
            }

            (logic_screens, layouts, fabs)
        };

        ParsedSketchwareProject {
            project: parser::project::Project {
                id: val.metadata.local_id,
                app_name: val.metadata.name,
                workspace_name: val.metadata.workspace_name,
                package_name: val.metadata.package_name,
                version_code: val.metadata.version_code,
                version_name: val.metadata.version_name,
                date_created: val.metadata.time_created,
                custom_icon: val.custom_icon.is_some(),
                color_palette: parser::project::ProjectColorPalette {
                    color_primary: val.colors.color_primary,
                    color_primary_dark: val.colors.color_primary_dark,
                    color_accent: val.colors.color_accent,
                    color_control_normal: val.colors.color_control_normal,
                    color_control_highlight: val.colors.color_control_highlight,
                },
                sketchware_version: val.metadata.sketchware_version,
            },
            file: parser::file::File { activities, custom_views },
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
                images: resource_conv!(images, image_resource_files),
                sounds: resource_conv!(sounds, sound_resource_files),
                fonts: resource_conv!(fonts, font_resource_files)
            },
            view: parser::view::View {
                layouts,
                fabs
            },
            logic: parser::logic::Logic { screens: logic_screens },
            resource_files: val.automatic_res_file_ids.then(|| ResourceFiles {
                custom_icon: val.custom_icon,
                images: image_resource_files,
                sounds: sound_resource_files,
                fonts: font_resource_files
            })
        }
    }
}