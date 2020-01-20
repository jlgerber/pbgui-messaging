use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use crossbeam_utils::thread;

use pbgui_messaging::event::Event;
use pbgui_messaging::{new_event_handler, IMsg, OMsg};
use pbgui_vpin::vpin_dialog;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::Slot;
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::cpp_core::MutPtr;
use qt_widgets::QApplication;
use qt_widgets::{QMainWindow, QPushButton};
use rustqt_utils::enclose;
use std::rc::Rc;

fn main() {
    //let mut handles = Vec::new();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver): (Sender<IMsg>, Receiver<IMsg>) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<OMsg>, Receiver<OMsg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();

    QApplication::init(|app| unsafe {
        let mut main = QMainWindow::new_0a();
        let mut main_ptr = main.as_mut_ptr();
        let mut button = QPushButton::new();
        let button_ptr = button.as_mut_ptr();
        main.set_central_widget(button.into_ptr());
        let quit_slot = Slot::new(move || {
            to_thread_sender_quit
                .send(OMsg::Quit)
                .expect("couldn't send");
        });
        app.about_to_quit().connect(&quit_slot);

        let dialog = Rc::new(create_dialog("DEV01", "modelpublish-1.2.0", main_ptr));
        init_dialog(to_thread_sender.clone());
        // we create a slot that is triggered when OK is pressed to act only in the event
        // that the user has requested action.
        let accepted_slot = Slot::new(enclose! { (dialog) move || {
            if let Some(roles) = dialog.selected_roles() {
                println!("roles: {:?}", roles);
            } else {
                println!("roles: any");
            }
            if let Some(selected_level) = dialog.selected_level() {
                println!("level: {:?}", selected_level);
            } else {
                println!("level: {}", dialog.show_name());
            }
            match dialog.selected_site(){
                Some(site) => println!(
                    "site:  {}", site
                ),
                None => println!("site:  Any"),
            }
            dialog.accept();
        }});

        // Connect the accepted signal to the accepted slot
        dialog.accepted().connect(&accepted_slot);

        let exec_dialog_slot = Slot::new(enclose! { (dialog) move || {
            let result = dialog.dialog_mut().exec(); //
            println!("exec_dialog_slot triggered by button result -> {}", result);
        }});

        button_ptr.pressed().connect(&exec_dialog_slot);
        //
        // This Slot handles processessing incoming Events and Messages
        //
        let app_update = new_event_handler(dialog.clone(), receiver);
        let mut my_conductor = Conductor::<Event>::new(&app_update);
        let mut result = 0;
        // we use a scoped channel so that we can avoid needing 'static lifetimes on our
        // message components. This reduces the number of allocations we need to make.
        thread::scope(|s| {
            let handle = s.spawn(|_| loop {
                let msg = to_thread_receiver
                    .recv()
                    .expect("Unable to unwrap received msg");
                match msg {
                    OMsg::GetRoles => {
                        sender
                            .send(IMsg::Roles(vec![
                                "anim", "integ", "model", "fx", "cfx", "light", "comp", "roto",
                            ]))
                            .expect("unable to send roles");
                        my_conductor.signal(Event::UpdateRoles);
                    }
                    OMsg::GetSites => {
                        sender
                            .send(IMsg::Sites(vec![
                                "hyderabad",
                                "montreal",
                                "playa",
                                "vancouver",
                            ]))
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
            });
            // the application needs to show and execute before the thread handle is joined
            // so that the scope lives longer than the application
            main_ptr.show();
            result = QApplication::exec();
            let _res = handle.join().expect("problem joining scoped thread handle");
        })
        .expect("problem with scoped channel");
        result
    })
}

unsafe fn create_dialog<'a, I: Into<String>>(
    name: I,
    distribution: &'a str,
    main_ptr: MutPtr<QMainWindow>,
) -> vpin_dialog::VpinDialog<'a> {
    let dialog = vpin_dialog::VpinDialog::create(name, distribution, main_ptr);
    dialog.set_default_stylesheet();
    dialog
}
fn init_dialog(to_thread_sender: Sender<OMsg>) {
    to_thread_sender
        .send(OMsg::GetRoles)
        .expect("unable to get roles");
    to_thread_sender
        .send(OMsg::GetSites)
        .expect("unable to get sites");
    to_thread_sender
        .send(OMsg::GetLevels)
        .expect("unable to get levels");
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
