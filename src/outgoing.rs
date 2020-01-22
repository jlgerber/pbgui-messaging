//! models message being sent from the application to the secondary thread
pub mod ovpin_dialog;
pub use ovpin_dialog::OVpinDialog;

///
pub trait ToOMsg {
    fn to_omsg(self) -> OMsg;
}

#[derive(Debug, PartialEq)]
pub enum OMsg {
    VpinDialog(OVpinDialog),
    Quit,
}
