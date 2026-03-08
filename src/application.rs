/* application.rs
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

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;
    use crate::widgets::GtkRustTemplateWindow;

    #[derive(Debug, Default)]
    pub struct GtkRustTemplateApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for GtkRustTemplateApplication {
        const NAME: &'static str = "GtkRustTemplateApplication";
        type Type = super::GtkRustTemplateApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GtkRustTemplateApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_resource_base_path(Some("/com/example/gtk_rust_template/"));
        }
    }

    impl ApplicationImpl for GtkRustTemplateApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = GtkRustTemplateWindow::new(&*application);
                window.upcast()
            };

            window.present();
        }
    }

    impl GtkApplicationImpl for GtkRustTemplateApplication {}
    impl AdwApplicationImpl for GtkRustTemplateApplication {}
}

glib::wrapper! {
    pub struct GtkRustTemplateApplication(ObjectSubclass<imp::GtkRustTemplateApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GtkRustTemplateApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        self.add_action_entries([quit_action]);

        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("window.close", &["<Ctrl>W"]);
    }
}
