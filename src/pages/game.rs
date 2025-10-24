use crate::model::ItemGame;
use crate::model::OptValid;
use crate::pages::ViewPage;
use gtk::gio::prelude::*;
use gtk::gio::{ListModel, ListStore};
use gtk::glib::random_int_range;

use gtk::{Align, Box, Button, Entry, GridView, NoSelection, Orientation, SignalListItemFactory};
use gtk::{glib, prelude::*};

#[derive(Clone)]
pub struct Game {
    p_box: Box,
    pub add_word: Entry,
    pub btn_start: Button,
    box_word: Box,
    grid_view: GridView,
    grid_view_valid: GridView,
    pub name : String,
}

impl Game {
    pub fn new() -> Self {
        let p_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let box_word = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();

        box_word.set_halign(Align::Center);

        let add_word = Entry::builder().placeholder_text("type a word").max_length(40).build();

        let btn_start = Button::builder().icon_name("view-refresh-symbolic").build();

        box_word.append(&add_word);
        box_word.append(&btn_start);

        let grid_view = GridView::builder()
            .margin_top(10)
            .halign(Align::Center)
            .min_columns(7)
            .max_columns(7)
            .build();

        let grid_view_valid = GridView::builder()
            .margin_top(10)
            .halign(Align::Center)
            .build();

        p_box.append(&box_word);
        p_box.append(&grid_view_valid);
        p_box.append(&grid_view);

        let store = ListStore::builder()
            .item_type(ItemGame::static_type())
            .build();

        for i in 0..49 {
            store.append(&ItemGame::build("", i, false, &grid_view_valid));
        }

        let selectmode = NoSelection::new(Some(store.upcast::<ListModel>()));

        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, list_item| {
            let button = Button::builder().build();
            list_item.set_child(Some(&button));
        });
        
        factory.connect_bind({
            let btn_start_cl = btn_start.clone();
            move |_, list_item| {
             let button = list_item.child().and_downcast::<Button>().unwrap();
            let item = list_item.item().and_downcast::<ItemGame>().unwrap();
            let name = item.name();
            button.set_label(&name);
            button.connect_clicked({
                let c_item = item.clone();
                let btn = button.clone();
                let btn_str = btn_start_cl.clone();
                move |_| {
                    let model_valid = c_item
                        .grid_valid()
                        .model()
                        .and_downcast::<NoSelection>()
                        .unwrap();

                    let item_c = btn.label();

                    let mut item_game: Option<ItemGame> = None;
                    let mut status_valid = false;

                    for i in 0..model_valid.n_items() {
                        let _item = model_valid.item(i).and_downcast::<ItemGame>().unwrap();

                        if _item.status() == true {
                            item_game = Some(_item);
                            if i == (model_valid.n_items() - 1){
                                 status_valid = true;
                            }
                            break;
                        }

                    }

                    if status_valid{
                        btn_str.set_sensitive(true);
                    }

                    if item_game != None {
                        let item_v = item_game.unwrap();
                        if item_v.name() == item_c.unwrap().to_string() {
                            btn.set_sensitive(false);
                            item_v.set_status(false);
                        }
                    }
                }
            });

               item.bind_property("name", &button, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();

            item.bind_property("status", &button, "sensitive")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        }});

        grid_view.set_factory(Some(&factory));
        grid_view.set_model(Some(&selectmode));
        let name = String::from("");
        Game {
            p_box,
            add_word,
            btn_start,
            box_word,
            grid_view,
            grid_view_valid,
            name
        }
    }

    pub fn set_grid_view_valid(&mut self, str: &str) {
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

        self.grid_view_valid.set_factory(Some(&factory));
        self.grid_view_valid.set_model(Some(&selectmode));

        self.grid_view_valid.set_factory(Some(&factory));
        self.grid_view_valid.set_model(Some(&selectmode));
        self.grid_view_valid.set_halign(Align::Center);
        self.grid_view_valid
            .set_min_columns(str.len().try_into().unwrap());
        self.grid_view_valid
            .set_max_columns(str.len().try_into().unwrap());
    }

    pub fn on_change_entry_game(&self) {
        //self is Clone
        if self.add_word.text_length() == 0 {
            self.btn_start.set_sensitive(false);
        } else if self.btn_start.get_sensitive() == false {
            self.btn_start.set_sensitive(true);
        }
    }

    pub fn set_name_game_for_gridview(&mut self, name: &str) {
        let model = self
            .grid_view
            .model()
            .and_downcast::<NoSelection>()
            .unwrap();
        
        self.name = name.to_string();
        for i in 0..model.n_items() {
            let item = model.item(i).and_downcast::<ItemGame>().unwrap();
            item.set_name("");
            item.set_status(false);
        }

        for (p, c) in name.char_indices() {
            let mut status = false;

            while !status {
                let num = random_int_range(0, 49);

                for i in 0..model.n_items() {
                    let item = model.item(i).and_downcast::<ItemGame>().unwrap();

                    if num == i as i32 && !item.status() {
                        item.set_name(c.to_string());
                        item.set_status(true);
                        status = true;
                        break;
                    }
                }
            }
        }
    }
}

impl ViewPage for Game {
    fn get_page(&self) -> &Box {
        &self.p_box
    }
}

impl OptValid for Game {
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
