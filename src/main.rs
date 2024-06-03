use gtk::gdk_pixbuf::PixbufAnimation;
use gtk::prelude::*;
use gtk::glib::Error;
use gtk::Image;
use gtk::{Application, ApplicationWindow};

//const FOX: &'static [u8] = include_bytes!("../res/animation.webp");
//const EXPLOSION: &'static [u8] = include_bytes!("../res/explosion.gif");

fn load_fox() -> Result<PixbufAnimation, Error> {
    gtk::gdk_pixbuf::PixbufAnimation::from_file("res/animation.gif")
}

fn load_explosion() -> Result<PixbufAnimation, Error> {
    gtk::gdk_pixbuf::PixbufAnimation::from_file("res/explosion.gif")
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello World!")
            .build();

        let fox = load_fox();
        match fox {
            Ok(pixbuf) => {
                println!("Fox loaded.");
                let anim_image = Image::from_animation(&pixbuf);
                win.add(&anim_image);
            },
            Err(err) => println!("Fox failed to load: {:?}", err),
        } 

        let explosion = load_explosion();   
        match explosion {
            Ok(pixbuf) => {
                println!("Explosion loaded.");
                let anim_image = Image::from_animation(&pixbuf);
                win.add(&anim_image);
            },
            Err(err) => println!("Explosion failed to load: {:?}", err),
        }     

        win.show_all();
    });

    app.run();
}
