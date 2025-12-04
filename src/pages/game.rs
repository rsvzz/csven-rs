use crate::model::ItemGame;
use crate::pages::ViewPage;
use gtk::gio::prelude::*;
use gtk::gio::{ListModel, ListStore};
use gtk::glib::random_int_range;

use gtk::{
    Align, Box, Builder, Button, CssProvider, Entry, GridView, NoSelection,
    STYLE_PROVIDER_PRIORITY_APPLICATION, SignalListItemFactory,
};
use gtk::{glib, prelude::*};
use std::{env};

#[derive(Clone)]
pub struct Game {
    p_box: Box,
    pub add_word: Entry,
    pub btn_start: Button,
    grid_view: GridView,
    grid_view_valid: GridView,
    provider_css: CssProvider,
    pub name: String,
}

impl Game {
    pub fn new(build: &Builder) -> Self {
        let provider = CssProvider::new();
        let path = env::current_exe().expect("No path exe");

        provider.load_from_path(
            path.parent()
                .unwrap()
                .join("../share/csven/styles/io.github.rsvzz.csven.css")
                .to_string_lossy()
                .to_string(),
        ); //release
        //provider.load_from_path("data/styles/io.github.rsvzz.csven.css"); //devmode

        let p_box: Box = build.object("box_game_main").unwrap();
        let add_word: Entry = build.object("entry_game_word").unwrap();
        let btn_start: Button = build.object("btn_game_start").unwrap();
        
        btn_start.add_css_class("btn_reset");
        btn_start
            .style_context()
            .add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        let grid_view: GridView = build.object("gv_game").unwrap();

        let grid_view_valid: GridView = build.object("g_tittle").unwrap();
        let store = ListStore::builder()
            .item_type(ItemGame::static_type())
            .build();

        for i in 0..49 {
            store.append(&ItemGame::build("", i, false, &grid_view_valid));
        }

        let selectmode = NoSelection::new(Some(store.upcast::<ListModel>()));

        let factory = SignalListItemFactory::new();

        factory.connect_setup({
            let _provider = provider.clone();
            move |_, list_item| {
                let button = Button::builder()
                    .width_request(40)
                    .height_request(40)
                    .build();
                button.add_css_class("btn_ramdon");
                button
                    .style_context()
                    .add_provider(&_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

                list_item.set_child(Some(&button));
            }
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
                                if i == (model_valid.n_items() - 1) {
                                    status_valid = true;
                                }
                                break;
                            }
                        }

                        if status_valid {
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
            }
        });

        grid_view.set_factory(Some(&factory));
        grid_view.set_model(Some(&selectmode));
        let name = String::from("");
        Game {
            p_box,
            add_word,
            btn_start,
            grid_view,
            grid_view_valid,
            provider_css: provider.clone(),
            name,
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

        factory.connect_setup({
            let _provider = self.provider_css.clone();
            move |_, list_item| {
                let button = Button::builder()
                    .width_request(40)
                    .height_request(40)
                    .build();

                button.add_css_class("btn_tittle");
                button
                    .style_context()
                    .add_provider(&_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

                list_item.set_child(Some(&button));
            }
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

        for (_, c) in name.char_indices() {
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
