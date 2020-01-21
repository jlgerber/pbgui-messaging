//! handle queries in a separate thread
use crate::{
    client_proxy::{ClientProxy, ConnectParams},
    Event, IMsg, OMsg,
};
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::thread;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::Slot;
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{cpp_core::MutPtr, QApplication, QMainWindow};

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
    connect_params: ConnectParams,
    mut main_window: MutPtr<QMainWindow>,
    mut conductor: Conductor<Event>,
    sender: Sender<IMsg>,
    receiver: Receiver<OMsg>,
) -> i32 {
    let mut result = 0;
    thread::scope(|s| {
        let handle = s.spawn(|_| {
            let client = match ClientProxy::connect(connect_params) {
                Ok(client) => client,
                Err(err) => {
                    sender
                        .send(IMsg::Error(err.to_string()))
                        .expect("unable to send roles");
                    conductor.signal(Event::Error);
                    panic!("unable to connect to database");
                }
            };
            let mut db = PackratDb::new(client);
            loop {
                let msg = receiver.recv().expect("Unable to unwrap received msg");
                match msg {
                    OMsg::GetRoles => {
                        let roles = match db.find_all_roles().query() {
                            Ok(roles) => roles,
                            Err(err) => {
                                sender
                                    .send(IMsg::Error(format!(
                                        "Unable to get roles from db: {}",
                                        err
                                    )))
                                    .expect("unable to send error msg");
                                conductor.signal(Event::Error);
                                continue;
                            }
                        };
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
                        let sites = match db.find_all_sites().query() {
                            Ok(sites) => sites,
                            Err(e) => {
                                sender
                                    .send(IMsg::Error(format!(
                                        "Unable to get sites from db: {}",
                                        e
                                    )))
                                    .expect("unable to send error msg");
                                conductor.signal(Event::Error);
                                continue;
                            }
                        };
                        //.expect("unable to get sites from db");
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

                    OMsg::GetLevels(ref show) => {
                        /*
                        Rhe query will return a result like this:
                        -------------
                        foo
                        foo.aa
                        foo.aa.0001
                        foo.aa.0002
                        foo.rd
                        foo.rd.0001
                        foo.rd.9999
                        ...
                        -------------
                        */
                        let levels = match db.find_all_levels().show(show).query() {
                            Ok(levels) => levels,
                            Err(e) => {
                                sender
                                    .send(IMsg::Error(format!(
                                        "Unable to get levels from db for {}: {}",
                                        show, e
                                    )))
                                    .expect("unable to send error msg");
                                conductor.signal(Event::Error);
                                continue;
                            }
                        };
                        let mut level_map = LevelMap::new();
                        // If we dont have any sequences or shots, then only the show will be returned.
                        // The length of the returned vec will be 1. We can return an empty map and continue.
                        if levels.len() == 1 {
                            sender
                                .send(IMsg::Levels(level_map))
                                .expect("Unable to send levelmap");
                            conductor.signal(Event::UpdateLevels);
                            continue;
                        }
                        // Now we get rid of the show name
                        let levels = &levels[1..];
                        // initialize a blank key (sequence)
                        let mut key = "".to_string();
                        // and an empty vec for shots
                        let mut shots: Vec<String> = Vec::new();
                        for level in levels {
                            let pieces = level.level.split(".").collect::<Vec<_>>();
                            let pieces_len = pieces.len();
                            // if we have two pieces, they are show and sequence.
                            if pieces_len == 2 {
                                // if the key is blank, then we have only just begun
                                if &key == "" {
                                    key = pieces[1].to_string();
                                } else {
                                    // we must have a previous sequence. It is time to insert
                                    // whatever sequence and shots we have collected thus far, and
                                    // set them up for the new sequence
                                    let old_shots = std::mem::replace(&mut shots, Vec::new());
                                    level_map.insert(key.clone(), old_shots);
                                    // and the new sequence is in the second spot in the vector
                                    key = pieces[1].to_string();
                                }
                            // we are in a shot
                            } else if pieces_len == 3 {
                                shots.push(pieces[2].to_string());
                            } else {
                                // if we are not in a show sequence or shot then what is going on?
                                panic!("Incorrect number of pieces from get_all_levels");
                            }
                        }
                        // we need to account for the last sequence and potential shots
                        // as they will never get inserted in the previous loop
                        // Of course, there is always the possiblity that we have no sequences
                        // or shots. So we guard against that.
                        if &key != "" {
                            level_map.insert(key, shots);
                        }
                        // now lets send our work
                        sender
                            .send(IMsg::Levels(level_map))
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
