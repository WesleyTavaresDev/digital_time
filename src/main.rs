use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder().application_id("org.time").build();

    app.connect_activate(setup);
    app.run();
}

fn setup(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("time")
        .default_width(180)
        .default_height(240)
        .build();

    window.present();
}
