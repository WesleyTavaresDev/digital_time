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
        .default_width(240)
        .default_height(180)
        .build();

    window.add(&gtk::Label::new(Some(&get_time())));

    window.show_all();
}

fn get_time() -> String {
    return format!(
        "{}:{}:{}",
        Local::now().hour(),
        Local::now().minute(),
        Local::now().second()
    );
}
