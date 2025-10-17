use gtk::{GridView};

use crate::model::ItemGame;

pub trait OptValid{
    fn get_next_name_valid(&self, grid : &GridView) -> Option<ItemGame>;
}