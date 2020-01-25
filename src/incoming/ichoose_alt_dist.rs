use super::*;

pub enum IChooseAltDist {
    Distributions(Vec<String>),
}

impl ToIMsg for IChooseAltDist {
    fn to_imsg(self) -> IMsg {
        IMsg::ChooseAltDist(self)
    }
}
