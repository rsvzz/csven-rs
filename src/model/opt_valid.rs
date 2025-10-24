use gtk::prelude::*;

use gtk::{GridView, NoSelection};

use crate::model::ItemGame;

pub trait OptValid{
    fn get_next_name_valid(grid: &GridView) -> Option<ItemGame> {
        let model = grid.model().and_downcast::<NoSelection>().unwrap();
        let mut item_game: Option<ItemGame> = None;
        for i in 0..model.n_items() {
            let item = model.item(i).and_downcast::<ItemGame>().unwrap();

            if item.status() == true {
                item_game = Some(item);
                break;
            }
        }
        if item_game != None {
            return item_game;
        }

        None
    }
}