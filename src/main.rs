use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ViewStack, ViewSwitcher};

use gtk::{Box, Label, MenuButton, Orientation, gio};

mod model;
mod pages;
use pages::Drag;

use pages::{Game, ViewPage};

fn main() {
    let app = Application::builder()
        .application_id("io.github.rsvzz.csven")
        .build();

    app.connect_activate(|app| {
        // Crear la ventana principal
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(700)
            .default_height(600)
            .build();

        let view_stack = ViewStack::builder().build();

        let p_game = Rc::new(RefCell::new(Game::new()));

        p_game.borrow_mut().btn_start.connect_clicked({
            let game = Rc::clone(&p_game);
            move |_| {
                //game.borrow_mut().on_clicked_button_start();
                //self is Clone
                let mut str = game.borrow().add_word.text();
                if str.len() == 0 {
                    str = game.borrow().name.to_string().into();
                }

                game.borrow().btn_start.set_sensitive(false);

                game.borrow_mut().set_grid_view_valid(&str);

                game.borrow_mut().set_name_game_for_gridview(&str);
                game.borrow().add_word.set_text("");
            }
        });

        p_game.borrow_mut().add_word.connect_changed({
            let game = Rc::clone(&p_game);
            move |_| {
                game.borrow().on_change_entry_game();
            }
        });

        let page1 = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();

        page1.append(ViewPage::get_page(&*p_game.borrow()));

        let page2 = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();

        let p_drag = Drag::build();

        p_drag.edit_word.connect_changed({
            let drag = p_drag.clone();
            move |_| {
                drag.on_change_entry_game();
            }
        });

          p_drag.btn_start.connect_clicked({
            let drag = p_drag.clone();
            move |_| {
                let mut str = drag.edit_word.text();

                if str.len() == 0 {
                    str = drag.name.to_string().into();
                }

                drag.btn_start.set_sensitive(false);
                drag.set_grid_view_valid(&str);
                drag.edit_word.set_text("");
            }
        });

        page2.append(ViewPage::get_page(&p_drag));

        let page3 = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();

        page3.append(&Label::builder().label("Contenido de la Página 3").build());

        view_stack.add_titled_with_icon(&page1, Some("page1"), "Game", "view-app-grid-symbolic");
        view_stack.add_titled_with_icon(
            &page2,
            Some("page2"),
            "Drag Game",
            "list-drag-handle-symbolic",
        );
        view_stack.add_titled_with_icon(
            &page3,
            Some("page3"),
            "Página 3",
            "input-keyboard-symbolic",
        );

        let view_switcher = ViewSwitcher::builder().stack(&view_stack).build();

        let header_bar = HeaderBar::builder().title_widget(&view_switcher).build();

        let menu = gio::Menu::new();
        menu.append(Some("About"), Some("app.about"));

        // Crear acciones
        let about_opt = gio::SimpleAction::new("about", None);
        about_opt.connect_activate(|_, _| {
            println!("About option");
        });

        app.add_action(&about_opt);

        let button_menu = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&menu)
            .build();

        header_bar.pack_end(&button_menu);

        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();

        main_box.append(&header_bar);
        main_box.append(&view_stack);

        window.set_content(Some(&main_box));
        window.present();
    });

    app.run();
}
