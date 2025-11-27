use gtk::gdk;
use gtk::glib::random_int_range;
use gtk::prelude::*;

use gtk::{
    Align, Box, Builder, Button, CssProvider, DragSource, DropTarget, Entry, EventController,
    GridView, NoSelection, STYLE_PROVIDER_PRIORITY_APPLICATION, SignalListItemFactory, glib,
};

use gdk::ContentProvider;

use crate::model::{ItemGame, OptValid, ValidGridView};
use crate::pages::ViewPage;
use gtk::gio::{ListModel, ListStore};

#[derive(Clone)]
pub struct Drag {
    p_box: Box,
    pub edit_word: Entry,
    pub btn_start: Button,
    gv_valid: GridView,
    pub name: String,
    provider_css: CssProvider,
    grid_view: GridView,
}

impl Drag {
    pub fn build(build: &Builder) -> Self {
        let provider = CssProvider::new();
        provider.load_from_path("/usr/local/share/csven/styles/io.github.rsvzz.csven.css"); //release
        //provider.load_from_path("data/styles/io.github.rsvzz.csven.css"); //devmode

        let box_p: Box = build.object("pdragbox_main").unwrap();
        let word_et: Entry = build.object("entry_drag_word").unwrap();
        let start_btn: Button = build.object("btn_drag_start").unwrap();
        start_btn.add_css_class("btn_reset");
        start_btn
            .style_context()
            .add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        let grid_view_valid: GridView = build.object("gdrag_tittle").unwrap();
        let view_grid = build.object("gv_verb").unwrap();

        let _name = String::new();
        Drag {
            p_box: box_p,
            edit_word: word_et,
            btn_start: start_btn,
            gv_valid: grid_view_valid,
            name: _name,
            provider_css: provider.clone(),
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

    pub fn set_grid_view_valid(&mut self, str: &str) {
        let store = ListStore::builder()
            .item_type(ValidGridView::static_type())
            .build();

        self.name = str.to_string();

        for (i, c) in str.char_indices() {
            let item = ValidGridView::new(
                &self.grid_view,
                &self.gv_valid,
                &ItemGame::new(&c.to_string(), i as i8, true),
                &self.btn_start,
            );
            store.append(&item);
        }

        let selectmode = NoSelection::new(Some(store.upcast::<ListModel>()));

        let factory = SignalListItemFactory::new();

        factory.connect_setup({
            let _provider = self.provider_css.clone();
            move |_, list_item| {
                let button = Button::builder().build();

                button.add_css_class("btn_tittle");
                button
                    .style_context()
                    .add_provider(&_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

                list_item.set_child(Some(&button));
            }
        });

        factory.connect_bind(|_, list_item| {
                let button = list_item.child().and_downcast::<Button>().unwrap();
                let valid_grid = list_item.item().and_downcast::<ValidGridView>().unwrap();
                let item = valid_grid.item_g();

                button.set_label(&item.name());

                item.bind_property("name", &button, "label")
                    .flags(glib::BindingFlags::SYNC_CREATE)
                    .build();

                item.bind_property("status", &button, "sensitive")
                    .flags(glib::BindingFlags::SYNC_CREATE)
                    .build();

                let drop_target =
                    DropTarget::new(ValidGridView::static_type(), gdk::DragAction::COPY);
                drop_target.connect_drop({
                    let btn = button.clone(); //drop button
                    move |_, value, _, _| {
                        if let Ok(value) = value.get::<ValidGridView>() {
                            // 1 index equal valid
                            let model_valid = value
                                .gview_drop()
                                .model()
                                .and_downcast::<NoSelection>()
                                .unwrap();

                            let mut item_game: Option<ItemGame> = None;
                            //let mut status_valid = false;

                            for i in 0..model_valid.n_items() {
                                let item_val =
                                    model_valid.item(i).and_downcast::<ValidGridView>().unwrap();
                                let _item_g = item_val.item_g();

                                if _item_g.status() == true {
                                    item_game = Some(_item_g);
                                    break;
                                }
                            }

                            if item_game != None {
                                let item_arrive = value.item_g();
                                let cp_item = item_game.clone().unwrap();
                                //println!("actual : {}, llega {}", cp_item.name(), item_arrive.name());
                                if item_arrive.name() == cp_item.name()
                                    && btn.label().unwrap() == item_arrive.name()
                                {
                                    cp_item.set_status(false);
                                    item_arrive.set_status(false);

                                    //end word
                                    if cp_item.idex() as u32 == (model_valid.n_items() - 1) {
                                        value.btn_reset().set_sensitive(true);
                                    }
                                }
                            }

                            return true;
                        }
                        false
                    }
                });
                button.add_controller(drop_target.upcast::<EventController>());
        });

        self.gv_valid.set_factory(Some(&factory));
        self.gv_valid.set_model(Some(&selectmode));
        self.gv_valid.set_halign(Align::Center);
        self.gv_valid.set_min_columns(str.len().try_into().unwrap());
        self.gv_valid.set_max_columns(str.len().try_into().unwrap());

        let store_drag = ListStore::builder()
            .item_type(ValidGridView::static_type())
            .build();

        let mut srdown = true;

        let mut n_str = String::from(str);
        let mut idex = 0;

        while srdown {
            let num = random_int_range(0, n_str.len() as i32);

            let c_item = n_str.chars().nth(num as usize).unwrap();
            let item = ValidGridView::new(
                &self.grid_view,
                &self.gv_valid,
                &ItemGame::new(&c_item.to_string(), idex, true),
                &self.btn_start,
            );
            store_drag.append(&item);
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

        factory_drag.connect_bind({
            let _provider = self.provider_css.clone();
            move |_, list_item| {
                let button = list_item.child().and_downcast::<Button>().unwrap();
                let valid_grid = list_item.item().and_downcast::<ValidGridView>().unwrap();
                let item = valid_grid.item_g();
                button.add_css_class("btn_ramdon");
                button
                    .style_context()
                    .add_provider(&_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
                button.set_label(&item.name());

                item.bind_property("name", &button, "label")
                    .flags(glib::BindingFlags::SYNC_CREATE)
                    .build();

                item.bind_property("status", &button, "sensitive")
                    .flags(glib::BindingFlags::SYNC_CREATE)
                    .build();

                // Drag source
                let drag_source = DragSource::new();
                drag_source.set_actions(gdk::DragAction::COPY);
                let provider = ContentProvider::for_value(&valid_grid.clone().to_value());
                drag_source.set_content(Some(&provider));
                button.add_controller(drag_source.upcast::<EventController>());
            }
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

impl OptValid for Drag {}
