use std::fs::File;
use std::path::PathBuf;
use std::io::{prelude::*};
use nfde::*;
use fltk::text::{TextEditor, TextBuffer};
use fltk::{enums::*,prelude::*,*};
use fltk_theme::{ColorTheme,color_themes};
use std::fs::OpenOptions;
use fltk::menu::MenuBar;
use fltk::window::Window;
use fltk::app::App;
//add Subsystem and entry to toml
//trait for save_as
pub trait Ready{        
fn convert(c:TextBuffer){}
}
impl Ready for TextBuffer{
fn convert(c:TextBuffer){
save_as(c).expect("Error!");
}
}
//for creating and saving new file
fn save_as(a:TextBuffer)->Result<(),nfde::Error>{
    let nfd = Nfd::new()?;
    let mut buf1=PathBuf::from(r"C:\Users\lukas\Desktop\Vs rust");    
    buf1.push(r"\eddie");
    buf1.set_file_name("1.txt");
    let res = nfd
        .save_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
         .show();
        match res {
         DialogResult::Ok(buf1) => {
        let buf2=buf1.clone();
        let file=File::create(buf2).expect("Error!");
        let b= a.text();
        let mut options = OpenOptions::new().write(true).open(buf1).expect("cannot open file");
        options.write_all(b.as_bytes()).expect("write failed");
         }
        DialogResult::Cancel => {
        println!("User canceled an action");
        }
         DialogResult::Err(error_str) => {
            println!("{}",error_str);
        }
    }
Ok(())
}
//trait for opening a file
pub trait Right{    
    fn op(c:TextBuffer,b:PathBuf,a:MenuBar){}
    }
    impl Right for TextBuffer{
    fn op(c:TextBuffer,b:PathBuf,a:MenuBar){
    open(c,b,a).expect("Error!");
    }
    }
//opening new file
fn open(a:TextBuffer,b:PathBuf,c:MenuBar)->Result<(),nfde::Error>{
    let nfd = Nfd::new()?;
    let res = nfd
        .open_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
         .show();
        match res {
         DialogResult::Ok(b) => {
        //let buf2=b.clone();
        kind(b,a,c).expect("Error!");
         }
        DialogResult::Cancel => {
        println!("User canceled an action");
        }
         DialogResult::Err(error_str) => {
            println!("{}",error_str);
        }
    }
Ok(())
}
fn kind(a:PathBuf,d:TextBuffer,f:MenuBar)->std::io::Result<()>{
    let c=a.clone();
    let mut m=File::open(a)?;
    let mut s=String::new();
    let r=m.read_to_string(&mut s)?;
    let x=s.clone();
    let t=s.as_str();
    let j=c.clone();
    red(t,d,j,f,x);
    Ok(())    
}

fn red(a:&str,mut b:TextBuffer,c:PathBuf,mut e:MenuBar,r:String){
let d=b.clone();
b.remove(0,1000);
b.append(a);
let f=r.clone();
e.add(
    "File/Save\t",
    Shortcut::Ctrl | 's',
    menu::MenuFlag::Normal,
    move |_|{
    let a=c.clone();
    let c=d.clone();
    let d=f.clone();
    TextBuffer::save(a,c,d);
    });
}
//trait for saving a file
pub trait End{
    fn save(a:PathBuf,b:TextBuffer,c:String){}
}
    impl End for TextBuffer{
        fn save(a:PathBuf,b:TextBuffer,c:String){
            if c.len()<b.length().try_into().unwrap(){
                let mut options = OpenOptions::new().append(true).open(a).expect("cannot open file");
                options.write_all(b.text().as_bytes()).expect("write failed");
                }
                else if c.len()>b.length().try_into().unwrap(){
                    let mut options = OpenOptions::new().truncate(true).write(true).open(a).expect("cannot open file");
                    options.write_all(b.text().as_bytes()).expect("write failed");
                }
    }
    }
//trait for creating a file
pub trait Righty{
    fn read(a:TextBuffer,b:PathBuf,c:MenuBar){}
    }
    impl Righty for TextBuffer{
    fn read(a:TextBuffer,b:PathBuf,c:MenuBar){
    new(a,b,c).expect("Error!");
    }
    }
//creating new file
fn new(a:TextBuffer,b:PathBuf,c:MenuBar)->Result<(),nfde::Error>{
    let nfd = Nfd::new()?;
    let res = nfd
        .save_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
         .show();
        match res {
         DialogResult::Ok(b) => {
            let b3=b.clone();
        File::create(b3).expect("Error!");
        kind(b,a,c).expect("Error!");
         }
         DialogResult::Cancel => {
        println!("User canceled an action");
        }
         DialogResult::Err(error_str) => {
        println!("{}",error_str);
        }
    }
    Ok(())
    }
fn main(){
    let mut path1=PathBuf::from(r"C:\Users\lukas\Desktop\Vs rust");
    path1.push(r"\eddie");
        path1.set_file_name("1.txt");
    let app=App::default().with_scheme(app::Scheme::Gtk);
    let mut wind=Window::default().with_label("Text editor").with_size(1920,1080);
    let theme=ColorTheme::new(color_themes::SHAKE_THEME);
    theme.apply();
    let a=TextBuffer::default();
    let buf=a.clone();
    let b=buf.clone();
    let c=b.clone();
    let mut menubar = MenuBar::new(0, 0, 50, 40, "rew");
    let menu=menubar.clone();
    let r=menu.clone();
    let s=move ||->MenuBar{
    let g=r.clone();
    return g;
    };
    let h=s.clone();
    let path2=path1.clone();
    let mut txt=TextEditor::default().with_size(1800,825).center_of_parent();
    txt.set_buffer(a.clone());
    wind.size_range(5,5,0,0);
    wind.end();
    wind.show();
    a.text();
    menubar.add(
        "File/New\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        move |_|{
        let d=c.clone();
        let e=path2.clone();
        let f=h.clone();
        TextBuffer::read(d,e,f());
        },
    );
    menubar.add(
        "File/Open\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        move |_|{
        let g=path1.clone();
        let d=b.clone();
        let h=s.clone();
        TextBuffer::op(d,g,h());
        },
    );
    menubar.add(
        "File/Save_as\t",
        Shortcut::Ctrl | 'd',
        menu::MenuFlag::Normal,
        move |_|{
        let d=a.clone();
        TextBuffer::convert(d);
        },
    );
    app.run().unwrap();
}