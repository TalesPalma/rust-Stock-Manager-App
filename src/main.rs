mod services;

use gtk::glib::clone;
use gtk::glib::property::PropertySet;
use gtk::{glib, Button, Orientation};
use gtk::{prelude::*, Application, ApplicationWindow};
use services::consume_api;
use std::cell::Cell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

#[tokio::main]
async fn main() -> glib::ExitCode {
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

    button_increment.connect_clicked(move |_| {
        glib::MainContext::default().spawn_local(async move {
            let service_api = consume_api::get_price_coin().await.unwrap();
        });
    });

    //Cria uma box para colocar nosso buttons
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // Inserindo os button dentro da box
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
