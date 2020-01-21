//! handle queries in a separate thread
use crate::{Event, IMsg, OMsg};
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::thread;
use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
use packybara::traits::*;
use pbgui_vpin::vpin_dialog::LevelMap;
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

pub fn create(
    mut main_ptr: MutPtr<QMainWindow>,
    mut my_conductor: Conductor<Event>,
    sender: Sender<IMsg>,
    to_thread_receiver: Receiver<OMsg>,
) -> i32 {
    let mut result = 0;
    thread::scope(|s| {
        let handle = s.spawn(|_| {
            let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
            let mut db = PackratDb::new(client);
            loop {
                let msg = to_thread_receiver
                    .recv()
                    .expect("Unable to unwrap received msg");
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
                        my_conductor.signal(Event::UpdateRoles);
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
                        my_conductor.signal(Event::UpdateSites);
                    }
                    OMsg::GetLevels => {
                        sender
                            .send(IMsg::Levels(initialize_levelmap()))
                            .expect("Unable to send levelmap");
                        my_conductor.signal(Event::UpdateLevels);
                    }
                    OMsg::Quit => return,
                }
            }
        });
        // the application needs to show and execute before the thread handle is joined
        // so that the scope lives longer than the application
        unsafe {
            main_ptr.show();
            result = QApplication::exec();
        }
        let _res = handle.join().expect("problem joining scoped thread handle");
    })
    .expect("problem with scoped channel");
    result
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
