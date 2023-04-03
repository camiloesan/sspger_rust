use glib::clone;
use gtk::{prelude::*, Label};
use gtk::{glib, Application, ApplicationWindow, Button, Entry, PasswordEntry};
use queries::data_access;
mod queries;

const APP_ID: &str = "mx.uv.fei.sspger";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let entry_user = Entry::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let entry_password = PasswordEntry::builder()
        .margin_top(10)
        .margin_bottom(20)
        .margin_start(10)
        .margin_end(10)
        .build(); 

    let button_login = Button::builder()
        .label("Login")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let label_access = Label::builder()
        .label("xxx")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let gtk_vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_vbox.append(&entry_user);
    gtk_vbox.append(&entry_password);
    gtk_vbox.append(&button_login);
    gtk_vbox.append(&label_access);

    //gtk_vbox.remove(&entry_user); 

    button_login.connect_clicked(clone!(@weak entry_user, @weak label_access => 
        move |_| {
            if data_access::validate_user(data_access::connection(), entry_user.text().to_string(), entry_password.text().to_string()) {
                label_access.set_label("CORRECTO");
            } else {
                label_access.set_label("INCORRECTO"); 
            }
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SSPGER")
        .child(&gtk_vbox)
        .build();

    window.present();
}
