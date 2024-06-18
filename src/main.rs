use gtk::glib::clone;
use gtk::{glib, Button, Orientation};
use gtk::{prelude::*, Application, ApplicationWindow};
use std::cell::Cell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    //Vamos criar o button agora e add com o child
    let button_increment = Button::builder()
        .label("Increment")
        .margin_top(12)
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .build();

    let button_decrement = Button::builder()
        .label("Decrement")
        .margin_top(12)
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .build();

    let number = Rc::new(Cell::new(1));
    //action de quando o button é clicado

    //Button que que incrementa e seta no outro button kkkkk
    button_increment.connect_clicked(clone!(@weak number,@weak button_decrement =>
        move |_| {
            number.set(number.get() + 1 );
            button_decrement.set_label(&number.get().to_string())
        }
    ));

    button_decrement.connect_clicked(clone!(@weak button_increment =>
    move |_| {
            number.set(number.get() - 1);
            button_increment.set_label(&number.get().to_string())
        }
    ));

    //Cria uma box para colocar noss buttons
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    // Inserindo os button dentro da box
    gtk_box.append(&button_decrement);
    gtk_box.append(&button_increment);

    //build a janela
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Minha janelinha")
        .child(&gtk_box)
        .build();

    //apresenta ela
    window.present();
}
