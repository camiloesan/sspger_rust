pub mod interfaces {
    use crate::{glib::clone, queries::data_access};
    use gtk::*;
    use traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt};
    const WINDOW_WIDTH: i32 = 800;
    const WINDOW_HEIGHT: i32 = 600;
    const HORIZONTAL_MARGIN: i32 = WINDOW_WIDTH / 3;
    const VERTICAL_MARGIN: i32 = WINDOW_HEIGHT / 3;

    pub fn build_login_ui(app: &Application) {
        let gtk_vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(15)
            .build();
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(WINDOW_WIDTH)
            .default_height(WINDOW_HEIGHT)
            .title("SSPGER")
            .child(&gtk_vbox)
            .build();
        let entry_user = Entry::builder()
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .margin_top(VERTICAL_MARGIN)
            .build();
        let entry_password = PasswordEntry::builder()
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .build();
        entry_password.set_show_peek_icon(true);
        let button_login = Button::builder()
            .label("Login")
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .build();
        let label_access = Label::builder()
            .label("xxx")
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .build();
        gtk_vbox.append(&entry_user);
        gtk_vbox.append(&entry_password);
        gtk_vbox.append(&button_login);
        gtk_vbox.append(&label_access);
        window.present();
        button_login.connect_clicked(clone!(@weak entry_user, @weak label_access =>
            move |_| {
                if data_access::validate_user(data_access::connection(),
                entry_user.text().to_string(),
                entry_password.text().to_string()) {
                    label_access.set_label("CORRECTO");
                } else {
                    label_access.set_label("INCORRECTO");
                }
                window.set_child(Some(&main_menu_box()));
        }));
    }

    fn main_menu_box() -> gtk::Box {
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(15)
            .build();
        let sidebar = gtk::StackSidebar::builder().build();

        let vbox_developers = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(25)
            .build();
        let title_developers = gtk::Label::builder()
            .label("Desarrolladores")
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .margin_top(VERTICAL_MARGIN)
            .build();
        vbox_developers.append(&title_developers);
        let list_box_developers = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .build();

        //adding items
        data_access::get_all_developers().iter().for_each(|developer| {
            let hbox = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(5)
                .build();
            hbox.append(&Label::new(Some(&format!("{}", developer.0))));
            hbox.append(&Label::new(Some(&format!("{}", developer.1))));
            hbox.append(&Label::new(Some(&format!("{}", developer.2))));

            let row = ListBoxRow::builder()
                .child(&hbox)
                .halign(gtk::Align::Center)
                .hexpand(true)
                .margin_bottom(2)
                .margin_top(2)
                .build();
            list_box_developers.append(&row);
        });
        vbox_developers.append(&list_box_developers);

        let vbox_projects = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(25)
            .build();
        let title_projects = gtk::Label::builder()
            .label("Proyectos")
            .margin_start(HORIZONTAL_MARGIN)
            .margin_end(HORIZONTAL_MARGIN)
            .margin_top(VERTICAL_MARGIN)
            .build();
        vbox_projects.append(&title_projects);

        let stack = gtk::Stack::builder().build();

        stack.add_titled(
            &vbox_developers,
            Some("they dont know me son"),
            &format!("Desarrolladores"),
        );
        stack.add_titled(
            &vbox_projects,
            Some("they dont know me son2"),
            &format!("Proyectos"),
        );
        sidebar.set_stack(&stack);

        hbox.append(&sidebar);
        hbox.append(&stack);

        hbox
    }
}
