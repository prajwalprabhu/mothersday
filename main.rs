use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let application = Application::new(
        Some("com.github.prajwalprabhu.mothersday"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let greet: Vec<&str> = vec![
            "\t\t\" It may be possible to gild pure Gold\t\t",
            "\t\tBut who can make \t\t",
            "\t\tmother more beautiful \"\t\t",
            "\t\t\n\n\n\t\t",
            "\t\tThis is a small wish from \t\t",
            "\t\tT Prajwal Prabhu &\t\t",
            "\t\tT Parthiksha Prabhu \t\t",
            "\t\tto thier mom on\t\t",
            "\t\t😍 MOTHERS DAY 😍\t\t",
        ];
        let window = ApplicationWindow::new(app);
        window.set_title("Mothers day");
        // window.set_default_size(350, 70);
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        for i in greet {
            let label = gtk::Label::new(Some(i));
            layout.add(&label);
        }
        let show = gtk::Button::with_label("Show");
        layout.add(&show);
        show.connect_clicked(move |_| {
            let words: Vec<&str> = vec![
                "\t\t\"  If love is a sweet as a flower \t\t",
                "\t\tthen my mother is that sweet flower of love\"\t\t",
                "\t\t\n\n\n\t\t",
                "\t\t\"Mother is like a flower beautiful and unique\t\t",
                "\t\tIf I made a list of the People I admire\t\t",
                "\t\t MOM would probably fill up half of it ,\t\t",
                "\t\tShe could be anything and everything\"\t\t",
                "\t\t\n\n\n\t\t",
                "\t\t 🥰 \"Happy Mothers for my sweet mom\" 😘\t\t",
                "\t\t   🙏🙏  \t\t",
            ];
            let top = gtk::Window::new(gtk::WindowType::Toplevel);
            let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
            for i in words {
                let label = gtk::Label::new(Some(i));
                layout.add(&label);
            }
            top.add(&layout);
            top.show_all();
        });

        window.add(&layout);

        window.show_all();
    });

    application.run(&[]);
}
