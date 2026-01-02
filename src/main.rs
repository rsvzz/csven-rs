use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::{AboutDialog, Application, ApplicationWindow, Dialog, EntryRow, ViewStack};

use gtk::{
    Box, Builder, Button, CssProvider, Label, MenuButton, gio,
};

use std::env;
mod model;
mod pages;

use model::ChangeChar;
use pages::{Drag, Game};

fn main() {
    let _ = gtk::init(); //need CssProvider
    let provider = CssProvider::new();
    let path = env::current_exe().expect("No path exe");

    let css_file = "data/styles/io.github.rsvzz.csven.css"; //devmode
    provider.load_from_path(css_file); //devmode
   
/* 
    let css_file = "../share/csven/styles/io.github.rsvzz.csven.css";
    provider.load_from_path(
        path.parent()
            .unwrap()
            .join(css_file)
            .to_string_lossy()
            .to_string(),
    ); //release
*/
    

     gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(), 
            &provider, 
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

    let app = Application::builder()
        .application_id("io.github.rsvzz.csven")
        .build();

    app.connect_activate({
        let dir = path.clone();
        move |app| {
            // Create main page.
            let verb_ui = "../../data/ui/verb.ui";
            let csven_ui = "../../data/ui/csven.ui";

            //release
            //let verb_ui = "../share/csven/ui/verb.ui";
            //let csven_ui = "../share/csven/ui/csven.ui";
            
            let build = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(csven_ui) //devmode ../share/csven/ui/csven.ui
                    .to_string_lossy()
                    .to_string(),
            );

       

            let window: ApplicationWindow = build.object("app").unwrap();
            window.set_default_width(700);
            window.set_default_height(600);
            window.set_application(Some(app));

            let view_stack: ViewStack = build.object("view_stack").unwrap();

            let p_game = Rc::new(RefCell::new(Game::new(&build)));

            //enter edit word.
            p_game.borrow_mut().add_word.connect_activate({
                let game = Rc::clone(&p_game);
                move |entry| {
                    let mut str = entry.text();
                    if str.len() == 0 {
                        str = game.borrow().name.to_string().into();
                    }

                    if str.len() != 0 {
                        game.borrow().btn_start.set_sensitive(false);
                        game.borrow_mut().set_grid_view_valid(&str);
                        game.borrow_mut().set_name_game_for_gridview(&str);
                        game.borrow().add_word.set_text("");
                    }
                }
            });

            p_game.borrow_mut().btn_start.connect_clicked({
                let game = Rc::clone(&p_game);
                move |_| {
                    let mut str = game.borrow().add_word.text();
                    if str.len() == 0 {
                        str = game.borrow().name.to_string().into();
                    }

                    if str.len() != 0 {
                        game.borrow().btn_start.set_sensitive(false);
                        game.borrow_mut().set_grid_view_valid(&str);
                        game.borrow_mut().set_name_game_for_gridview(&str);
                        game.borrow().add_word.set_text("");
                    }
                }
            });

            p_game.borrow_mut().add_word.connect_changed({
                let game = Rc::clone(&p_game);
                move |_| {
                    game.borrow().on_change_entry_game();
                }
            });

            let p_drag = Rc::new(RefCell::new(Drag::build(&build)));

            p_drag.borrow().edit_word.connect_changed({
                let drag = Rc::clone(&p_drag);
                move |_| {
                    drag.borrow().on_change_entry_game();
                }
            });
            p_drag.borrow_mut().edit_word.connect_activate({
                let drag = Rc::clone(&p_drag);
                move |_| {
                    let mut str = drag.borrow().edit_word.text();

                    if str.len() == 0 {
                        str = drag.borrow().name.to_string().into();
                    }

                    if str.len() != 0 {
                        drag.borrow().btn_start.set_sensitive(false);
                        drag.borrow_mut().set_grid_view_valid(&str);
                        drag.borrow().edit_word.set_text("");
                    }
                }
            });

            p_drag.borrow_mut().btn_start.connect_clicked({
                let drag = Rc::clone(&p_drag);
                move |_| {
                    let mut str = drag.borrow().edit_word.text();

                    if str.len() == 0 {
                        str = drag.borrow().name.to_string().into();
                    }

                    if str.len() != 0 {
                        drag.borrow().btn_start.set_sensitive(false);
                        drag.borrow_mut().set_grid_view_valid(&str);
                        drag.borrow().edit_word.set_text("");
                    }
                }
            });

            let btn_add: Button = build.object("btn_add_header").unwrap();
            let stack_view: ViewStack = build.object("view_stack").unwrap();

            //let verb_ui: Builder = Builder::from_file("../share/csven/ui/verb.ui"); //release
 
            let verb_ui: Builder = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(verb_ui)
                    .to_string_lossy()
                    .to_string(),
            ); //devmode
            btn_add.connect_clicked({
                let _app = window.clone();
                let _verb = verb_ui.clone();
                let _build_ui = build.clone();
                let _provider = provider.clone();
                move |_| {
                    let dialog: Dialog = _verb.object("verb_dialog").unwrap();
                    let btn_save: Button = _verb.object("btn_save").unwrap();
                    let btn_cancel: Button = _verb.object("btn_cancel").unwrap();
                    btn_save.connect_clicked({
                        let _build = _build_ui.clone();
                        let _bverb = _verb.clone();
                        let _dialog = dialog.clone();
                        let __provider = _provider.clone();
                        move |_| {
                            let et_verb: EntryRow = _bverb.object("et_verb").unwrap();
                            let et_past: EntryRow = _bverb.object("et_past").unwrap();
                            let et_v3: EntryRow = _bverb.object("et_v3").unwrap();
                            let et_ing: EntryRow = _bverb.object("et_ing").unwrap();

                            if et_verb.text_length() > 0
                                && et_ing.text_length() > 0
                                && et_v3.text_length() > 0
                                && et_ing.text_length() > 0
                            {
                                let lbl_verb: Label = _build.object("lbl_verb").unwrap();
                                let lbl_past: Label = _build.object("lbl_past").unwrap();
                                let lbl_v3: Label = _build.object("lbl_v3").unwrap();
                                let lbl_ing: Label = _build.object("lbl_ing").unwrap();

                                lbl_verb.add_css_class("label_tittle");
                                lbl_past.add_css_class("label_tittle");
                                lbl_v3.add_css_class("label_tittle");
                                lbl_ing.add_css_class("label_tittle");

                                let compare: ChangeChar = ChangeChar::new();

                                let _verb_base = compare.get_change_name_for_char(
                                    et_past.text().to_string(),
                                    et_verb.text().to_string(),
                                    '*',
                                );

                                let _verb_v3 = compare.get_change_name_for_char(
                                    et_v3.text().to_string(),
                                    et_past.text().to_string(),
                                    '^',
                                );

                                let _verb_ing = compare.get_change_name_for_char(
                                    et_ing.text().to_string(),
                                    et_verb.text().to_string(),
                                    '*',
                                );

                                lbl_verb.set_label(&et_verb.text());
                                lbl_past.set_label(&_verb_base);
                                lbl_v3.set_label(&_verb_v3);
                                lbl_ing.set_label(&_verb_ing);

                                et_verb.set_text("");
                                et_past.set_text("");
                                et_v3.set_text("");
                                et_ing.set_text("");
                                _dialog.close();
                            }
                        }
                    });
                    btn_cancel.connect_clicked({
                        let _dialog = dialog.clone();
                        move |_| {
                            _dialog.close();
                        }
                    });
                    dialog.present(Some(&_app));
                }
            });
            stack_view.connect_notify_local(Some("visible-child"), {
                let btn = btn_add.clone();
                move |stack, _| {
                    if let Some(child) = stack.visible_child_name() {
                        if child == "verb" {
                            btn.set_visible(true); // show button  add for dialog
                        } else {
                            btn.set_visible(false); //hidden button add
                        }
                    }
                }
            });

            let menu = gio::Menu::new();
            menu.append(Some("About"), Some("app.about"));

            let about_opt = gio::SimpleAction::new("about", None);

            about_opt.connect_activate({
                let _win = window.clone();
                let _dir = dir.clone();
                move |_, _| {
                    let about_ui = "../../data/ui/about.ui"; //devmode
                    
                    //let about_ui = "../share/csven/ui/about.ui"; //release

                    let about_build = Builder::from_file(
                        _dir.parent()
                            .unwrap()
                            .join(about_ui)
                            .to_string_lossy()
                            .to_string(),
                    );
                    
                    let _dialog: AboutDialog = about_build.object("about_dialog").unwrap();

                    _dialog.present(Some(&_win));
                }
            });

            app.add_action(&about_opt);

            let button_menu: MenuButton = build.object("menu_option").unwrap();
            button_menu.set_menu_model(Some(&menu));

            let main_box: Box = build.object("bax_main").unwrap();

            main_box.append(&view_stack);

            window.present();
        }
    });

    app.run();
}
