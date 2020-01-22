use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use env_logger::Env;
use pbgui_messaging::{
    client_proxy::ConnectParams, event::Event, new_event_handler, thread as pbthread, IMsg, OMsg,
    OVpinDialog,
};
use pbgui_vpin::vpin_dialog;
use qt_core::Slot;
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{cpp_core::MutPtr, QApplication, QMainWindow, QPushButton};
use rustqt_utils::enclose;
use std::rc::Rc;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver): (Sender<IMsg>, Receiver<IMsg>) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<OMsg>, Receiver<OMsg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();

    QApplication::init(|app| unsafe {
        let mut main = QMainWindow::new_0a();
        let main_ptr = main.as_mut_ptr();
        let mut button = QPushButton::new();
        let button_ptr = button.as_mut_ptr();
        main.set_central_widget(button.into_ptr());
        // wire up message to terminate secondary thread
        let _quit_slot = pbthread::create_quit_slot(to_thread_sender_quit, app.clone());

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
        let my_conductor = Conductor::<Event>::new(&app_update);
        pbthread::create(
            ConnectParams::default(),
            main_ptr,
            my_conductor,
            sender,
            to_thread_receiver,
        )
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
        .send(OMsg::VpinDialog(OVpinDialog::GetRoles))
        .expect("unable to get roles");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetSites))
        .expect("unable to get sites");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetLevels(
            "dev02".to_string(),
        )))
        .expect("unable to get levels");
}
