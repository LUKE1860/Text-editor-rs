#![windows_subsystem = "windows"]
use fltk::app::App;
use fltk::menu::MenuBar;
use fltk::text::{TextBuffer, TextEditor};
use fltk::window::Window;
use fltk::{enums::*, prelude::*, *};
use fltk_theme::{color_themes, ColorTheme};
use nfde::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
mod buffer;
//trait for save_as

fn main() {
    let mut folder = env::current_dir().unwrap();
    let mut path=PathBuf::from(folder);
    let mut f_owned_path=path.to_owned();
    let mut s_owned_path=path.to_owned();
    let app = App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_label("Text editor")
        .with_size(1920, 1080);
    let theme = ColorTheme::new(color_themes::SHAKE_THEME);
    theme.apply();
    let text = TextBuffer::default();
    let f_owned_text=text.to_owned();
    let s_owned_text=text.to_owned();
    let mut menubar = MenuBar::new(0, 0, 50, 40, "rew");
    let f_owned_menubar=menubar.to_owned();
    let s_owned_menubar=menubar.to_owned();
    let mut txt = TextEditor::default()
        .with_size(1800, 825)
        .center_of_parent();
    txt.set_buffer(text.to_owned());
    wind.size_range(5, 5, 0, 0);
    wind.end();
    wind.show();

    menubar.add(
        "File/New\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        move |_| {
            let moved_text = f_owned_text.to_owned();
            let moved_path = f_owned_path.to_owned();
            let moved_menubar= f_owned_menubar.to_owned();
            buffer::new(moved_text,moved_path,moved_menubar).expect("Error");
        },
    );
    menubar.add(
        "File/Open\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        move |_| {
            let moved_text = s_owned_text.to_owned();
            let moved_path = s_owned_path.to_owned();
            let moved_menubar = s_owned_menubar.to_owned();
            buffer::open_file(moved_text,moved_path,moved_menubar).expect("Error");
        },
    );
    menubar.add(
        "File/Save_as\t",
        Shortcut::Ctrl | 'd',
        menu::MenuFlag::Normal,
        move |_| {
            let moved_text=text.to_owned();
            buffer::save_as(moved_text).expect("Error");
        },
    );
    app.run().unwrap();
}
