//! Event is used to formalize the qt signal that triggers an
//! update application side.
//! The messaging to the application from the db is split between the Event and the IMsg.
//! The Event signals that a given state has changed.
//! THe IMsg provides the details of the state change.
use qt_core::QString;
use qt_thread_conductor::traits::*;
use qt_widgets::cpp_core::{CppBox, Ref};

#[derive(Debug, PartialEq)]
pub enum Event {
    UpdateRoles,
    UpdateSites,
    UpdateLevels,
}

/*
In order to scale, I want to do something like this:

pub enum Vpin {
    UpdateRoles,
    UpdateSites,
    UpdateLevels
}
pub enum Event {
    Vpin(Vpin)
}

impl ToQString for Event {
    match & self {
        &Event::Vpin(Vpin::UpdateRoles) => QString::from_std::str("Vpin::UpdateRoles"),
        &Event::Vpin(Vpin::UpdateSites) => QString::from_std::str("Vpin::UpdateSites"),
        &Event::Vpin(Vpin::UpdateLevels) => QString::from_std::str("Vpin::UpdateLevels"),
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "Vpin::UpdateRoles" => Event::Vpin(Vpin::UpdateRoles),
            "Vpin::UpdateSites" => Event::Vpin(Vpin::UpdateSites),
            "Vpin::UpdateLevels" => Event::Vpin(Vpin::UpdateLevels),
            _ => panic!("Unable to convert to Event"),
        }
    }
}
*/
impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::UpdateRoles => QString::from_std_str("UpdateRoles"),
            &Event::UpdateSites => QString::from_std_str("UpdateSites"),
            &Event::UpdateLevels => QString::from_std_str("UpdateLevels"),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "UpdateRoles" => Event::UpdateRoles,
            "UpdateSites" => Event::UpdateSites,
            "UpdateLevels" => Event::UpdateLevels,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
