use super::*;

#[derive(Debug, PartialEq)]
pub enum OChooseAltDist {
    GetDistributions { package: String },
}

impl ToOMsg for OChooseAltDist {
    fn to_omsg(self) -> OMsg {
        OMsg::ChooseAltDist(self)
    }
}
