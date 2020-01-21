//! models message being sent from the application to the secondary thread

pub trait ToOMsg {
    fn to_omsg(self) -> OMsg;
}

#[derive(Debug, PartialEq)]
pub enum OMsg {
    VpinDialog(OVpinDialog),
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum OVpinDialog {
    GetSites,
    GetRoles,
    GetLevels(String),
}

impl ToOMsg for OVpinDialog {
    fn to_omsg(self) -> OMsg {
        OMsg::VpinDialog(self)
    }
}
