use crate::{prelude::*, Event, IMsg};
use crossbeam_channel::Receiver;
use log;
use pbgui_vpin::vpin_dialog;
//use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::rc::Rc;

/// Generate a new signal handler, which is of type `SlotOfQString`.
/// The signal handler is responsible for handling Signals of type Event
pub fn new_event_handler<'a, 'b: 'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    receiver: Receiver<IMsg<'b>>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
        Event::UpdateSites => {
            if let Ok(IMsg::Sites(sites)) = receiver.recv() {
                println!("populating sites");
                dialog.set_sites(sites);
            } else {
                log::error!("Event::UpdateSites IMsg does not match event state");
            }
        }
        Event::UpdateRoles => {
            if let Ok(IMsg::Roles(roles)) = receiver.recv() {
                dialog.set_roles(roles);
            } else {
                log::error!("IMsg does not have Roles")
            }
        }
        Event::UpdateLevels => {
            if let Ok(IMsg::Levels(level_map)) = receiver.recv() {
                dialog.set_levels(level_map);
            } else {
                log::error!("IMsg does not have LevelMap");
            }
        }
    })
}
