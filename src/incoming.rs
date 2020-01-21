//! incoming models the message being sent from the secondary thread
//! to the application
use pbgui_vpin::vpin_dialog::LevelMap;

pub trait ToIMsg {
    fn to_imsg(self) -> IMsg;
}

pub enum IMsg {
    VpinDialog(IVpinDialog),
    Error(String),
}

pub mod ivpin_dialog;
pub use ivpin_dialog::IVpinDialog;
