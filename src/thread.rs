//! handle queries in a separate thread
use crate::{Event, IMsg, OMsg};
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::thread;
use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
use packybara::traits::*;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::Slot;
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{cpp_core::MutPtr, QApplication, QMainWindow};

pub struct ClientProxy {}

impl ClientProxy {
    pub fn connect() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )?;
        Ok(client)
    }
}

/// Create the thread that handles requests for data from the ui. The thread
/// receives messages via the `receiver`, matches against them, and sends data
/// back to the UI via the `sender`. Finally, triggering an appropriate update
/// via the `conductor`. The `conductor` and `sender` work as a team. The `sender`
/// handles complex data, and the `conductor` notifies QT.
///
/// # Arguments
/// * `main_window` - Mutable MutPtr wrapped QMainWindow instance
/// * `conductor` - Mutable instance of the Conductor<Event>, responsible for signaling
///                 to QT
/// * sender - Sends IMsg's to the UI thread
/// * receiver - Receives OMsg's from the UI thread
///
/// # Returns
/// * i32 - The status
pub fn create(
    mut main_window: MutPtr<QMainWindow>,
    mut conductor: Conductor<Event>,
    sender: Sender<IMsg>,
    receiver: Receiver<OMsg>,
) -> i32 {
    let mut result = 0;
    thread::scope(|s| {
        let handle = s.spawn(|_| {
            let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
            let mut db = PackratDb::new(client);
            loop {
                let msg = receiver.recv().expect("Unable to unwrap received msg");
                match msg {
                    OMsg::GetRoles => {
                        let roles = db
                            .find_all_roles()
                            .query()
                            .expect("unable to get roles from db");
                        let roles = roles
                            .into_iter()
                            .map(|mut x| std::mem::replace(&mut x.role, String::new()))
                            .collect::<Vec<_>>();
                        sender
                            .send(IMsg::Roles(roles))
                            .expect("unable to send roles");
                        conductor.signal(Event::UpdateRoles);
                    }
                    OMsg::GetSites => {
                        let sites = db
                            .find_all_sites()
                            .query()
                            .expect("unable to get sites from db");
                        // we use std::mem::replace because this should be a bit more efficient
                        // than clone, and certainly more
                        let sites = sites
                            .into_iter()
                            .map(|mut x| std::mem::replace(&mut x.name, String::new()))
                            .collect::<Vec<_>>();
                        sender
                            .send(IMsg::Sites(sites))
                            .expect("unable to send sites");
                        conductor.signal(Event::UpdateSites);
                    }
                    OMsg::GetLevels => {
                        sender
                            .send(IMsg::Levels(initialize_levelmap()))
                            .expect("Unable to send levelmap");
                        conductor.signal(Event::UpdateLevels);
                    }
                    OMsg::Quit => return,
                }
            }
        });
        // the application needs to show and execute before the thread handle is joined
        // so that the scope lives longer than the application
        unsafe {
            main_window.show();
            result = QApplication::exec();
        }
        let _res = handle.join().expect("problem joining scoped thread handle");
    })
    .expect("problem with scoped channel");
    result
}
/// Create the slot that handles terminating the secondary thread when
/// the application is about to quit. This function will also wire up
/// the appropriate signal & slot to handle this.
///
/// # Arguments
/// * `to_thread_sender` - the sender responsible for signaling the secondary thread.
/// * `app` - A MutPtr to the QApplication instance.
///
/// # Returns
/// * the slot designed to terminate the secondary thread
pub fn create_quit_slot<'a>(to_thread_sender: Sender<OMsg>, app: MutPtr<QApplication>) -> Slot<'a> {
    let quit_slot = Slot::new(move || {
        to_thread_sender.send(OMsg::Quit).expect("couldn't send");
    });
    unsafe {
        app.about_to_quit().connect(&quit_slot);
    }
    quit_slot
}

fn initialize_levelmap() -> LevelMap {
    let mut lm = LevelMap::new();
    lm.insert(
        "RD".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "9999".to_string(),
        ],
    );
    lm.insert(
        "AA".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "0004".to_string(),
        ],
    );
    lm
}
