use gio::prelude::*;
use gtk4::prelude::*;

mod document;
mod io;
mod stage;
mod window;

#[derive(Default)]
pub struct CookieManager {
    next: u32,
}

impl CookieManager {
    fn next(&mut self) -> u32 {
        let out = self.next;
        self.next += 1;

        out
    }
}

fn main() -> glib::ExitCode {
    let (io_send, io_recv) = io::start();
    let mut cookie_manager = CookieManager::default();

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

    let ret = app.run();

    io_send
        .send_blocking(io::IoMessage::Exit(cookie_manager.next()))
        .unwrap();

    ret
}
