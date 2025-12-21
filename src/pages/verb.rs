use adw::prelude::*;
use adw::{EntryRow};
use gtk::{Label, Orientation, Align, Box, Builder,};

use crate::pages::ViewPage;
pub struct BoxItemVerb {
    box_content: Box,
    name: Label,
}

impl BoxItemVerb {
    pub fn new(name: String, content: &Label) -> Self {


        let box_base = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .build();

        box_base.append(&Label::builder().label(name.to_string()).halign(Align::Start).build());
        box_base.append(content);

        BoxItemVerb {
            box_content: box_base,
            name: content.clone(),
        }
    }

    pub fn get_box_content(&self) -> &Box {
        &self.box_content
    }
}

#[derive(Clone)]
pub struct Verb {
    p_box: Box,
    lbl_base: Label,
    lbl_past: Label,
    lbl_participle: Label,
    lbl_ing: Label,
}

impl Verb {
    pub fn build(_build: &Builder) -> Self {

        let box_p = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let box_view = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Center)
            .build();

        let base_lbl = Label::builder().halign(Align::End).label("").build();
        let past_lbl = Label::builder().halign(Align::End).label("").build();
        let v2_lbl = Label::builder().halign(Align::End).label("").build();
        let ing_lbl = Label::builder().halign(Align::End).label("").build();

        box_view.append(BoxItemVerb::new("Verb Name: ".to_string(), &base_lbl).get_box_content());
        box_view.append(BoxItemVerb::new("Verb Past: ".to_string(), &past_lbl).get_box_content());
        box_view.append(BoxItemVerb::new("Verb Participle: ".to_string(), &v2_lbl).get_box_content());
        box_view.append(BoxItemVerb::new("Verb ING: ".to_string(), &ing_lbl).get_box_content());

        let box_data = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::End)
            .margin_end(10)
            .width_request(200)
            .build();

        let et_base = EntryRow::new();
        et_base.set_title("verb name:");
        //et_base.add_prefix(&Image::from_icon_name("contact-new-symbolic"));
        box_data.append(&et_base);

        let et_past = EntryRow::new();
        et_past.set_title("past verb:");
        box_data.append(&et_past);

        let et_v3 = EntryRow::new();
        et_v3.set_title("Participle verb:");
        box_data.append(&et_v3);

        let et_ing = EntryRow::new();
        et_ing.set_title("ING verb:");
        box_data.append(&et_ing);

        //box_body.append(&box_view);
        //box_body.append(&box_data);
        // box_data.set_visible(false);
        box_p.append(&box_view);

        Verb {
            p_box: box_p,
            lbl_base: base_lbl,
            lbl_past: past_lbl,
            lbl_participle: v2_lbl,
            lbl_ing: ing_lbl,
        }
    }
}

impl ViewPage for Verb {
    fn get_page(&self) -> &Box {
        &self.p_box
    }
}
