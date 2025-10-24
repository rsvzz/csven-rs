use glib::prelude::*;
use glib::{Object, Properties};
use gtk::{Button, GridView, glib, subclass::prelude::*};
use std::cell::RefCell;

use crate::model::ItemGame;

glib::wrapper! {
    pub struct ValidGridView(ObjectSubclass<imp::ValidGridView>);
}

impl ValidGridView {
    pub fn new(
        gview_drag: &GridView,
        grid_drop: &GridView,
        item: &ItemGame,
        btn_reset: &Button,
    ) -> Self {
        Object::builder()
            .property("gview_drop", grid_drop)
            .property("grid_drag", gview_drag)
            .property("item_g", item)
            .property("btn_reset", btn_reset)
            .build()
    }
}

mod imp {
    use gtk::Button;

    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ValidGridView)]
    pub struct ValidGridView {
        #[property(get, set)]
        gview_drop: RefCell<GridView>,
        #[property(get, set)]
        grid_drag: RefCell<GridView>,
        #[property(get, set)]
        item_g: RefCell<ItemGame>,
        #[property(get, set)]
        btn_reset: RefCell<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ValidGridView {
        const NAME: &'static str = "ValidGridView";
        type Type = super::ValidGridView;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ValidGridView {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
