use fltk::{app, button::Button, prelude::*, window::Window, frame::Frame};

fn main() {
    hello_titlebar();
    hello_titlebar1();
}

fn hello_titlebar1() {
    let app = app::App::default();

    let mut window = Window::new(100, 100, 400, 300, "Hello 2");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut button = Button::new(160, 210, 80, 40, "Button!");

    window.end();
    window.show();

    button.set_callback(move |_| frame.set_label("Hello World"));
    app.run().unwrap();
}

fn hello_titlebar() {
    let app = app::App::default();

    let mut window  = Window::new(1000, 100, 400, 300, "Hello FTLK");

    window.end();
    window.show();
    app.run().unwrap();
}
