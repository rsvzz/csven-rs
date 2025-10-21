use gtk::glib::random_int_range;
use gtk::prelude::*;
use gtk::{
    Align, Box, Button, Entry, GridView, NoSelection, Orientation, SignalListItemFactory, glib,
};

use crate::model::ItemGame;
use crate::pages::ViewPage;
use gtk::gio::{ListModel, ListStore};

#[derive(Clone)]
pub struct Drag {
    p_box: Box,
    pub edit_word: Entry,
    pub btn_start: Button,
    box_info: Box,
    gv_valid: GridView,
    pub name: String,
    grid_view: GridView,
}

impl Drag {
    pub fn build() -> Self {
        let box_p = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let box_word = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();

        box_word.set_halign(Align::Center);

        let word_et = Entry::builder()
            .placeholder_text("type a word")
            .max_length(40)
            .build();

        let start_btn = Button::builder().icon_name("view-refresh-symbolic").build();

        box_word.append(&word_et);
        box_word.append(&start_btn);

        let grid_view_valid = GridView::builder()
            .margin_top(10)
            .halign(Align::Center)
            .build();

        let view_grid = GridView::builder()
            .margin_top(10)
            .halign(Align::Center)
            .build();

        box_p.append(&box_word);
        box_p.append(&grid_view_valid);
        box_p.append(&view_grid);

        let _name = String::from("");
        Drag {
            p_box: box_p,
            edit_word: word_et,
            btn_start: start_btn,
            box_info: box_word,
            gv_valid: grid_view_valid,
            name: _name,
            grid_view: view_grid,
        }
    }

    pub fn on_change_entry_game(&self) {
        //self is Clone
        if self.edit_word.text_length() == 0 {
            self.btn_start.set_sensitive(false);
        } else if self.btn_start.get_sensitive() == false {
            self.btn_start.set_sensitive(true);
        }
    }

    pub fn set_grid_view_valid(&self, str: &str) {
        let store = ListStore::builder()
            .item_type(ItemGame::static_type())
            .build();

        for (i, c) in str.char_indices() {
            store.append(&ItemGame::new(&c.to_string(), i as i8, true));
        }

        let selectmode = NoSelection::new(Some(store.upcast::<ListModel>()));

        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, list_item| {
            let button = Button::builder().build();
            list_item.set_child(Some(&button));
        });

        factory.connect_bind(|_, list_item| {
            let button = list_item.child().and_downcast::<Button>().unwrap();
            let item = list_item.item().and_downcast::<ItemGame>().unwrap();

            button.set_label(&item.name());

            item.bind_property("name", &button, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();

            item.bind_property("status", &button, "sensitive")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        });

        self.gv_valid.set_factory(Some(&factory));
        self.gv_valid.set_model(Some(&selectmode));
        self.gv_valid.set_halign(Align::Center);
        self.gv_valid.set_min_columns(str.len().try_into().unwrap());
        self.gv_valid.set_max_columns(str.len().try_into().unwrap());

        let store_drag = ListStore::builder()
            .item_type(ItemGame::static_type())
            .build();

        let mut srdown = true;

        let mut n_str = String::from(str);
        let mut idex = 0;
        
        while srdown {
            let num = random_int_range(0, n_str.len() as i32);

            let c_item = n_str.chars().nth(num as usize).unwrap();

            store_drag.append(&ItemGame::new(&c_item.to_string(), idex, true));
            idex += 1;
            n_str.remove(num as usize);
            if store_drag.n_items() == str.len() as u32 {
                srdown = false;
            }
        }

        let selectmode_drag = NoSelection::new(Some(store_drag.upcast::<ListModel>()));

        let factory_drag = SignalListItemFactory::new();

        factory_drag.connect_setup(|_, list_item| {
            let button = Button::builder().build();
            list_item.set_child(Some(&button));
        });

        factory_drag.connect_bind(|_, list_item| {
            let button = list_item.child().and_downcast::<Button>().unwrap();
            let item = list_item.item().and_downcast::<ItemGame>().unwrap();

            button.set_label(&item.name());

            item.bind_property("name", &button, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();

            item.bind_property("status", &button, "sensitive")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        });

        self.grid_view.set_factory(Some(&factory_drag));
        self.grid_view.set_model(Some(&selectmode_drag));
        self.grid_view.set_halign(Align::Center);
        self.grid_view
            .set_min_columns(str.len().try_into().unwrap());
        self.grid_view
            .set_max_columns(str.len().try_into().unwrap());
    }
}

impl ViewPage for Drag {
    fn get_page(&self) -> &Box {
        &self.p_box
    }
}
