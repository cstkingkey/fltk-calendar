use chrono::prelude::*;
use fltk::{prelude::*, *};
use fltk_calendar::calendar;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut but = button::Button::new(160, 200, 80, 40, "Click");
    win.end();
    win.show();
    but.set_callback(move |_| {
        let cal = calendar::Calendar::default(); // or calendar::Calendar::new(200, 100);
        let date = cal.get_date();
        println!("{:?}", date);
        if let Some(date) = date {
            println!("{:?}", date.year());
            println!("{:?}", date.month());
            println!("{:?}", date.day());
        }
    });
    app.run().unwrap();
}
