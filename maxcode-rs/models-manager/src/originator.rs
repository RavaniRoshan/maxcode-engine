//! Bounded catalog partitions for callers migrated to CCA. Other callers keep the default catalog.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModelCatalogOriginator {
    Work,
    WorkWeb,
    WorkDesktop,
    WorkMobile,
    Chat,
}

impl ModelCatalogOriginator {
    pub fn from_originator(originator: &str) -> Option<Self> {
        match originator {
            "maxcode_work_cca" => Some(Self::Work),
            "maxcode_work_web" => Some(Self::WorkWeb),
            "maxcode_work_desktop" => Some(Self::WorkDesktop),
            "maxcode_work_mobile" => Some(Self::WorkMobile),
            "chatgpt_cca" => Some(Self::Chat),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Work => "maxcode_work_cca",
            Self::WorkWeb => "maxcode_work_web",
            Self::WorkDesktop => "maxcode_work_desktop",
            Self::WorkMobile => "maxcode_work_mobile",
            Self::Chat => "chatgpt_cca",
        }
    }
}
