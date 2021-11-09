use crate::error::{SWRSError, SWRSResult};
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

}

pub struct Libraries {

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