use chrono::prelude::*;
use gtk::glib;
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
        .default_width(240)
        .default_height(180)
        .build();

    let time_label = gtk::Label::new(Some(&get_time()));
    window.add(&time_label);

    window.show_all();

    glib::timeout_add_seconds_local(1, move || {
        time_label.set_text(&get_time());
        glib::Continue(true)
    });
}

fn get_time() -> String {
    return format!(
        "{}:{}:{}",
        Local::now().hour(),
        Local::now().minute(),
        Local::now().second()
    );
}
