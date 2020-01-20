//! models message being sent from the application to the secondary thread
pub enum OMsg {
    GetSites,
    GetRoles,
    GetLevels,
    Quit,
}
