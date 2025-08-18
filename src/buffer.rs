use fltk::menu::MenuBar;
use fltk::text::TextBuffer;
use fltk::{enums::*, prelude::*, *};
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
        let buf1 = PathBuf::from(path);
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
