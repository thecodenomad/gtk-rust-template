/* window.rs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::config::APP_ID;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use once_cell::sync::OnceCell;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type=super::GtkRustTemplateWindow)]
    #[template(file = "src/widgets/window.blp")]
    pub struct GtkRustTemplateWindow {
        pub settings: OnceCell<gio::Settings>,
        #[template_child]
        pub button: TemplateChild<gtk::Button>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkRustTemplateWindow {
        const NAME: &'static str = "GtkRustTemplateWindow";
        type Type = super::GtkRustTemplateWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.install_action("win.about", None, move |win, _, _| {
                win.show_about_dialog();
            });

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GtkRustTemplateWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.set_icon_name(Some(APP_ID));
            obj.setup_settings();
            obj.load_window_size();

            if APP_ID.ends_with("Devel") {
                obj.add_css_class("devel");
            }

            let button = obj.button();
            let toast_overlay = obj.imp().toast_overlay.clone();
            button.connect_clicked(move |_button| {
                println!("Hello World");
                let toast = adw::Toast::new("Hello World");
                toast_overlay.add_toast(toast);
            });
        }
    }
    impl WidgetImpl for GtkRustTemplateWindow {}
    impl WindowImpl for GtkRustTemplateWindow {
        fn close_request(&self) -> glib::Propagation {
            self.obj()
                .save_window_size()
                .expect("able to save window state");
            glib::Propagation::Proceed
        }
    }
    impl ApplicationWindowImpl for GtkRustTemplateWindow {}
    impl AdwApplicationWindowImpl for GtkRustTemplateWindow {}
}

glib::wrapper! {
    pub struct GtkRustTemplateWindow(ObjectSubclass<imp::GtkRustTemplateWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow, gtk::Constraint,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GtkRustTemplateWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn button(&self) -> gtk::Button {
        self.imp().button.get()
    }

    fn setup_settings(&self) {
        let settings = gio::Settings::new("com.example.gtk_rust_template");
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` has not been set");
    }

    fn settings(&self) -> &gio::Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` has been set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let size = self.default_size();

        self.settings().set_int("window-width", size.0)?;
        self.settings().set_int("window-height", size.1)?;
        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let width = self.settings().int("window-width");
        let height = self.settings().int("window-height");
        let is_maximized = self.settings().boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    fn show_about_dialog(&self) {
        let about = adw::AboutDialog::new();
        about.set_application_name("gtk-rust-template");
        about.set_version("0.1.0");
        about.set_license_type(gtk::License::Gpl30);
        about.set_comments("A GNOME/GTK application template written in Rust.");

        about.present(Some(
            self.root().unwrap().downcast_ref::<gtk::Window>().unwrap(),
        ));
    }
}
