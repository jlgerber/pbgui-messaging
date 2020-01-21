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
pub fn new_event_handler<'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    receiver: Receiver<IMsg>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
        Event::UpdateSites => {
            if let Ok(IMsg::Sites(sites)) = receiver.recv() {
                println!("populating sites");
                let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                dialog.set_sites(sites_ref);
            } else {
                log::error!("Event::UpdateSites IMsg does not match event state");
            }
        }
        Event::UpdateRoles => {
            if let Ok(IMsg::Roles(roles)) = receiver.recv() {
                let roles_ref = roles.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                dialog.set_roles(roles_ref);
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
