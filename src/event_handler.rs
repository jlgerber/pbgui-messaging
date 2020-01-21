use crate::{prelude::*, Event, IMsg, IVpinDialog, VpinDialog};
use crossbeam_channel::Receiver;
use log;
use pbgui_vpin::vpin_dialog;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::rc::Rc;

// TODO: move to event_handler/vpin_dialog.rs
pub(crate) mod vpin_dialog_event_handler {
    use super::*;

    pub fn match_vpin_dialog<'a>(
        event: VpinDialog,
        dialog: Rc<vpin_dialog::VpinDialog<'a>>,
        receiver: &Receiver<IMsg>,
    ) {
        match event {
            VpinDialog::UpdateSites => {
                if let Ok(IMsg::VpinDialog(IVpinDialog::Sites(sites))) = receiver.recv() {
                    let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                    dialog.set_sites(sites_ref);
                } else {
                    log::error!("Event::UpdateSites IMsg does not match event state");
                }
            }
            VpinDialog::UpdateRoles => {
                if let Ok(IMsg::VpinDialog(IVpinDialog::Roles(roles))) = receiver.recv() {
                    let roles_ref = roles.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                    dialog.set_roles(roles_ref);
                } else {
                    log::error!("IMsg does not have Roles")
                }
            }
            VpinDialog::UpdateLevels => {
                if let Ok(IMsg::VpinDialog(IVpinDialog::Levels(level_map))) = receiver.recv() {
                    dialog.set_levels(level_map);
                } else {
                    log::error!("IMsg does not have LevelMap");
                }
            }
        }
    }
}
use vpin_dialog_event_handler::match_vpin_dialog;

/// Generate a new event handler, which is of type `SlotOfQString`.
/// The event handler is responsible for handling Signals of type Event
///
/// # Arguments
/// * `dialog` - Rc wrapped VpinDialog
/// * `receiver` - The Receiver of messages from the non-ui thread
///
/// # Returns
/// * Slot which processes messages from the non-ui thread and updates the ui in response
pub fn new_event_handler<'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    receiver: Receiver<IMsg>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
        Event::VpinDialog(vpin_dialog_event) => {
            match_vpin_dialog(vpin_dialog_event, dialog.clone(), &receiver)
        }
        Event::Error => {
            if let Ok(IMsg::Error(error)) = receiver.recv() {
                log::error!("{}", error);
            } else {
                log::error!("unable to transmit error");
            }
        }
    })
}
