#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibPatchId {
    pub package: String,
    pub component: String,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibMinorId {
    pub package: String,
    pub component: String,
    pub major: u16,
    pub minor: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibMajorId {
    pub package: String,
    pub component: String,
    pub major: u16,
}

#[derive(Hash, PartialEq, Eq)]
pub struct ComponentLibNameOnlyId {
    pub package: String,
    pub component: String,
}

impl ComponentLibPatchId {
    pub fn into_minor(self) -> ComponentLibMinorId {
        ComponentLibMinorId {
            package: self.package,
            component: self.component,
            major: self.major,
            minor: self.minor,
        }
    }
}
