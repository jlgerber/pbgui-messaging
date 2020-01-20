//! incoming models the message being sent from the secondary thread
//! to the application
use pbgui_vpin::vpin_dialog::LevelMap;
pub enum IMsg<'a> {
    Roles(Vec<&'a str>),
    Sites(Vec<&'a str>),
    Levels(LevelMap),
}
