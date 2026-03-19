use gio::prelude::*;
use gtk4::prelude::*;

mod io;
mod window;

fn main() -> glib::ExitCode {
    let (io_send, io_recv) = io::start();

    //io_send.send_blocking(io::IoMessage::ConnectVTSTracker())

    gio::resources_register_include!("resources.gresource").expect("valid resource file");
    gtk4::init().expect("valid gtk4 state");

    //look_and_feel::init();

    let app = gtk4::Application::builder()
        .application_id("live.arcturus.ningyotsukai")
        .build();

    app.connect_activate(|app| {
        let window = window::WindowController::new(app);

        window.present();
    });

    app.run()
}