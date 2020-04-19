#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub mod browser;
pub mod button;
pub mod dialog;
pub mod draw;
pub mod fl;
pub mod frame;
pub mod group;
pub mod image;
pub mod input;
pub mod menu;
pub mod misc;
pub mod output;
pub mod table;
pub mod text;
pub mod valuator;
pub mod widget;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
