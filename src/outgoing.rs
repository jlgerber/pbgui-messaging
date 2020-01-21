//! models message being sent from the application to the secondary thread

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
