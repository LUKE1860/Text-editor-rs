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
pub fn new(text: TextBuffer, path: PathBuf, menu: MenuBar) -> Result<(), nfde::Error> {
    let nfd = Nfd::new()?;
    let res = nfd
        .save_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();
    match res {
        DialogResult::Ok(b) => {
            let b3 = b.clone();
            File::create(b3).expect("Error!");
            read_file_to_open(text, path, menu).expect("Error!");
        }
        DialogResult::Cancel => {
            println!("User canceled an action");
        }
        DialogResult::Err(error_str) => {
            println!("{}", error_str);
        }
    }
    Ok(())
}
    pub fn open_file(buffer:TextBuffer,path:PathBuf,menu:MenuBar)->Result<(),nfde::Error>{
        let nfd = Nfd::new()?;
        let res = nfd
            .open_file()
            .add_filter("Source code", "c,cpp,cc")?
            .add_filter("Headers", "h,hpp")?
            .show();
        match res {
            DialogResult::Ok(new_path) => {
                //let buf2=b.clone();
                read_file_to_open(buffer,new_path,menu).expect("Error!");
            }
            DialogResult::Cancel => {
            }
            DialogResult::Err(error_str) => {
                println!("{}", error_str);
            }
        }
        Ok(())
    }
    pub fn save(buffer: TextBuffer,path: PathBuf, text: String) {
        if text.len() < buffer.length().try_into().unwrap() {
            let mut options = OpenOptions::new()
                .write(true)
                .open(path)
                .expect("cannot open file");
            options
                .write_all(buffer.text().as_bytes())
                .expect("write failed");
        } else if text.len() > buffer.length().try_into().unwrap() {
            let mut options = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(path)
                .expect("cannot open file");
            options
                .write_all(buffer.text().as_bytes())
                .expect("write failed");
        }
    }
    pub fn read_file_to_open(mut buffer:TextBuffer,path: PathBuf,mut menu: MenuBar) -> std::io::Result<()> {
        let reserve_path=path.to_owned();
        let mut content = File::open(path)?;
        let mut base_string = String::new();
        let read_string = content.read_to_string(&mut base_string)?;
        buffer.remove(0, 1000);
        buffer.append(base_string.as_str());
    menu.add(
        "File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        move |_| {
            let f_path = reserve_path.to_owned();
            let f_buffer= buffer.to_owned();
            let f_read_string =base_string.to_owned();
            save(f_buffer,f_path,f_read_string);
        },
    );
        Ok(())
    }
    pub fn save_as(buffer: TextBuffer) -> Result<(), nfde::Error> {
        let path=env::current_dir().unwrap();
        let nfd = Nfd::new()?;
        let mut buf1 = PathBuf::from(path);
        let res = nfd
            .save_file()
            .add_filter("Source code", "c,cpp,cc")?
            .add_filter("Headers", "h,hpp")?
            .show();
        match res {
            DialogResult::Ok(buf1) => {
                let buf2 = buf1.clone();
                let file = File::create(buf2).expect("Error!");
                let text_buffer = buffer.text();
                let mut options = OpenOptions::new()
                    .write(true)
                    .open(buf1)
                    .expect("cannot open file");
                options.write_all(text_buffer.as_bytes()).expect("write failed");
            }
            DialogResult::Cancel => {
            }
            DialogResult::Err(error_str) => {
                println!("{}", error_str);
            }
        }
        Ok(())
    }
/*
pub trait Ready {
    fn convert(c: TextBuffer) {}
}
impl Ready for TextBuffer {
    fn convert(c: TextBuffer) {
        save_as(c).expect("Error!");
    }
}
//for creating and saving new file

fn save_as(a: TextBuffer) -> Result<(), nfde::Error> {
    let path=env::current_dir().unwrap();
    let nfd = Nfd::new()?;
    let mut buf1 = PathBuf::from(path);
    //buf1.push(r"\eddie");
    //buf1.set_file_name("1.txt");
    let res = nfd
        .save_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();
    match res {
        DialogResult::Ok(buf1) => {
            let buf2 = buf1.clone();
            let file = File::create(buf2).expect("Error!");
            let b = a.text();
            let mut options = OpenOptions::new()
                .write(true)
                .open(buf1)
                .expect("cannot open file");
            options.write_all(b.as_bytes()).expect("write failed");
        }
        DialogResult::Cancel => {
            println!("User canceled an action");
        }
        DialogResult::Err(error_str) => {
            println!("{}", error_str);
        }
    }
    Ok(())
}
//trait for opening a file
pub trait Right {
    fn op(c: TextBuffer, b: PathBuf, a: MenuBar) {}
}
impl Right for TextBuffer {
    fn op(c: TextBuffer, b: PathBuf, a: MenuBar) {
        open(c, b, a).expect("Error!");
    }
}
//opening new file
fn open(a: TextBuffer, b: PathBuf, c: MenuBar) -> Result<(), nfde::Error> {
    let nfd = Nfd::new()?;
    let res = nfd
        .open_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();
    match res {
        DialogResult::Ok(b) => {
            //let buf2=b.clone();
            kind(b, a, c).expect("Error!");
        }
        DialogResult::Cancel => {
            println!("User canceled an action");
        }
        DialogResult::Err(error_str) => {
            println!("{}", error_str);
        }
    }
    Ok(())
}
fn kind(a: PathBuf, d: TextBuffer, f: MenuBar) -> std::io::Result<()> {
    let c = a.clone();
    let mut m = File::open(a)?;
    let mut s = String::new();
    let r = m.read_to_string(&mut s)?;
    let x = s.clone();
    let t = s.as_str();
    let j = c.clone();
    red(t, d, j, f, x);
    Ok(())
}

fn red(a: &str, mut b: TextBuffer, c: PathBuf, mut e: MenuBar, r: String) {
    let d = b.clone();
    b.remove(0, 1000);
    b.append(a);
    let f = r.clone();
    e.add(
        "File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        move |_| {
            let a = c.clone();
            let c = d.clone();
            let d = f.clone();
            TextBuffer::save(a, c, d);
        },
    );
}
//trait for saving a file
//used in save and new
pub trait End {
    fn save(a: PathBuf, b: TextBuffer, c: String) {}
}
impl End for TextBuffer {
    fn save(a: PathBuf, b: TextBuffer, c: String) {
        if c.len() < b.length().try_into().unwrap() {
            let mut options = OpenOptions::new()
                .write(true)
                .open(a)
                .expect("cannot open file");
            options
                .write_all(b.text().as_bytes())
                .expect("write failed");
        } else if c.len() > b.length().try_into().unwrap() {
            let mut options = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(a)
                .expect("cannot open file");
            options
                .write_all(b.text().as_bytes())
                .expect("write failed");
        }
    }
}
//trait for creating a file
pub trait Righty {
    fn read(a: TextBuffer, b: PathBuf, c: MenuBar) {}
}
impl Righty for TextBuffer {
    fn read(a: TextBuffer, b: PathBuf, c: MenuBar) {
        new(a, b, c).expect("Error!");
    }
}
//creating new file
fn new(a: TextBuffer, b: PathBuf, c: MenuBar) -> Result<(), nfde::Error> {
    let nfd = Nfd::new()?;
    let res = nfd
        .save_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();
    match res {
        DialogResult::Ok(b) => {
            let b3 = b.clone();
            File::create(b3).expect("Error!");
            kind(b, a, c).expect("Error!");
        }
        DialogResult::Cancel => {
            println!("User canceled an action");
        }
        DialogResult::Err(error_str) => {
            println!("{}", error_str);
        }
    }
    Ok(())
}
*/