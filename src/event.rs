//! Event is used to formalize the qt signal that triggers an
//! update application side.
//! The messaging to the application from the db is split between the Event and the IMsg.
//! The Event signals that a given state has changed.
//! THe IMsg provides the details of the state change.
use qt_core::QString;
use qt_thread_conductor::traits::*;
use qt_widgets::cpp_core::{CppBox, Ref};

#[derive(Debug, PartialEq)]
pub enum VpinDialog {
    UpdateRoles,
    UpdateSites,
    UpdateLevels,
}

pub trait ToEvent {
    fn to_event(self) -> Event;
}

impl ToEvent for VpinDialog {
    fn to_event(self) -> Event {
        Event::VpinDialog(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Event {
    VpinDialog(VpinDialog),
    Error,
}
/*
In order to scale, I want to do something like this:

pub enum VpinDialog {
    UpdateRoles,
    UpdateSites,
    UpdateLevels
}
pub enum Event {
    VpinDialog(VpinDialog)
}

impl ToQString for Event {
    match & self {
        &Event::VpinDialog(VpinDialog::UpdateRoles) => QString::from_std_str("VpinDialog::UpdateRoles"),
        &Event::VpinDialog(VpinDialog::UpdateSites) => QString::from_std_str("VpinDialog::UpdateSites"),
        &Event::VpinDialog(VpinDialog::UpdateLevels) => QString::from_std_str("VpinDialog::UpdateLevels"),
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "VpinDialog::UpdateRoles" => Event::VpinDialog(VpinDialog::UpdateRoles),
            "VpinDialog::UpdateSites" => Event::VpinDialog(VpinDialog::UpdateSites),
            "VpinDialog::UpdateLevels" => Event::VpinDialog(VpinDialog::UpdateLevels),
            _ => panic!("Unable to convert to Event"),
        }
    }
}
*/

impl ToQString for VpinDialog {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &VpinDialog::UpdateRoles => QString::from_std_str("VpinDialog::UpdateRoles"),
            &VpinDialog::UpdateSites => QString::from_std_str("VpinDialog::UpdateSites"),
            &VpinDialog::UpdateLevels => QString::from_std_str("VpinDialog::UpdateLevels"),
        }
    }
}

impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::VpinDialog(VpinDialog::UpdateRoles) => {
                QString::from_std_str("VpinDialog::UpdateRoles")
            }
            &Event::VpinDialog(VpinDialog::UpdateSites) => {
                QString::from_std_str("VpinDialog::UpdateSites")
            }
            &Event::VpinDialog(VpinDialog::UpdateLevels) => {
                QString::from_std_str("VpinDialog::UpdateLevels")
            }
            &Event::Error => QString::from_std_str("Error"),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "VpinDialog::UpdateRoles" => Event::VpinDialog(VpinDialog::UpdateRoles),
            "VpinDialog::UpdateSites" => Event::VpinDialog(VpinDialog::UpdateSites),
            "VpinDialog::UpdateLevels" => Event::VpinDialog(VpinDialog::UpdateLevels),
            "Error" => Event::Error,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
