use gtk::{glib, Button};
use gtk::{prelude::*, Application, ApplicationWindow};

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

    let button = Button::builder()
        .label("Clique me")
        .margin_top(12)
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .build();

    //action de quando o button é clicado

    button.connect_clicked(|button| {
        button.set_label("Hello word action button !!! ");
    });

    //build a janela
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Minha janelinha")
        .child(&button)
        .build();

    //apresenta ela
    window.present();
}
