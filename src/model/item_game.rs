use glib::prelude::*;
use glib::{Object, Properties};
use gtk::{GridView, glib, subclass::prelude::*};
use std::cell::RefCell;

use crate::model::ValidGridView;

glib::wrapper! {
    pub struct ItemGame(ObjectSubclass<imp::ItemGame>);
}

impl ItemGame {
    pub fn build(name: &str, idex: i8, status: bool, gridview: &GridView) -> Self {
        Object::builder()
            .property("name", name)
            .property("idex", idex)
            .property("status", status)
            .property("grid_valid", gridview)
            .build()
    }

    pub fn new(name: &str, idex: i8, status: bool) -> Self {
        Object::builder()
            .property("name", name)
            .property("idex", idex)
            .property("status", status)
            .build()
    }
}

impl Default for ItemGame {
      fn default() -> Self {
        // Aquí defines cómo se ve un ItemGame "vacío" o por defecto
       Object::builder()
            .property("name", "")
            .property("idex", 0 as i8)
            .property("status", false)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemGame)]
    pub struct ItemGame {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        idex: RefCell<i8>,
        #[property(get, set)]
        status: RefCell<bool>,
        #[property(get, set)]
        grid_valid: RefCell<GridView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemGame {
        const NAME: &'static str = "ItemGame";
        type Type = super::ItemGame;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemGame {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
