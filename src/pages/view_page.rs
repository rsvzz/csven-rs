use gtk::{Box};

pub trait ViewPage {
    fn get_page(&self) -> &Box;
}