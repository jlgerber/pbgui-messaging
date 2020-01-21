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

pub enum IVpinDialog {
    Roles(Vec<String>),
    Sites(Vec<String>),
    Levels(LevelMap),
}

impl ToIMsg for IVpinDialog {
    fn to_imsg(self) -> IMsg {
        IMsg::VpinDialog(self)
    }
}
