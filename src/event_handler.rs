use crate::{prelude::*, Event, IMsg, IVpinDialog, VpinDialog};
use crossbeam_channel::Receiver;
use log;
use pbgui_vpin::vpin_dialog;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::rc::Rc;

pub mod vpin_dialog_eh;
use vpin_dialog_eh::match_vpin_dialog;

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
