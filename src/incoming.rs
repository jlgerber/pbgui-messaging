//! incoming models the message being sent from the secondary thread
//! to the application
use pbgui_vpin::vpin_dialog::LevelMap;
pub enum IMsg {
    Roles(Vec<String>),
    Sites(Vec<String>),
    Levels(LevelMap),
}
