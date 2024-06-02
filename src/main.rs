use gdk_pixbuf::PixbufSimpleAnim;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const FOX: &'static [u8] = include_bytes!("../res/animation.webp");
const EXPLOSION: &'static [u8] = include_bytes!("../res/explosion.gif");

const FOX_FRAME_RATE: f32 = 24.0; // TODO determine from WEBP?

fn load_fox() -> Result<PixbufSimpleAnim, String> {
    let decoder = webp::AnimDecoder::new(FOX);
    let decoded = decoder.decode()?;
    let frame = decoded.get_frame(0).unwrap(); // TODO don't unwrap
    let width = frame.width() as i32;
    let height = frame.height() as i32;

    let simple_anim = PixbufSimpleAnim::new(width, height, FOX_FRAME_RATE);
    
    for frame in decoded.get_frames(0..decoded.len()).unwrap() { // TODO don't unwrap
        let pixbuf_data = frame.get_image();
    }

    Ok(simple_anim)
}

fn main() {
    let decoder = webp::AnimDecoder::new(FOX);
    let decoded = decoder.decode();
    match decoded {
        Ok(img) => {
            println!("Image loaded: {} frames.", img.len());
        }
        Err(err) => println!("Failed loading image: {}", err),
    };

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

        

        win.show_all();
    });

    app.run();
}
