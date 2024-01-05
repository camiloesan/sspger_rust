use gtk::{glib, Application, prelude::{ApplicationExt, ApplicationExtManual}};
mod gui;
mod queries;

const APP_ID: &str = "mx.uv.fei.sspger";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(gui::interfaces::build_login_ui);
    app.run()
}