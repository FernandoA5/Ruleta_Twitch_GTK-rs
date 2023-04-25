use gtk::gdk::Display;
use gtk::{prelude::*, Label};
use gtk::{glib, Application, ApplicationWindow};

pub async fn app() -> glib::ExitCode{
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_startup(|_| load_css());

    app.connect_activate(move |app| {
        build_ui(&app);
    });

    tokio::spawn(async move {
        connection().await;
    });

    app.run()
}

fn build_ui(app: &Application){

    let ruleta = draw_ruleta();
    ruleta.set_widget_name("ruleta");

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(640)
        .default_height(480)
        .title("Ruleta")
        .child(&ruleta)
        .build();
    
    draw_ruleta();


    window.present();
}

fn draw_ruleta() -> Label{
    Label::builder()
        .label("Ruleta")
        .build()
}

fn load_css(){
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("ruleta.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}