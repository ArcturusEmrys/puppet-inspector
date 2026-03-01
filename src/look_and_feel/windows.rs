use gtk4;

use windows::Foundation::TypedEventHandler;
use windows::UI::Color;
use windows::UI::ViewManagement::{UIColorType, UISettings};

use std::cell::RefCell;

trait ColorExt {
    fn into_css_hex(self) -> String;
}

impl ColorExt for Color {
    fn into_css_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.R, self.G, self.B, self.A)
    }
}

pub struct LAFProvider {
    ui_settings: UISettings,
    color_css: gtk4::CssProvider,
    laf_css: gtk4::CssProvider,
}

thread_local! {
    static LAF_SINGLETON: RefCell<Option<LAFProvider>> = RefCell::new(None);
}

impl LAFProvider {
    pub fn init() {
        if LAF_SINGLETON.with(|laf| laf.borrow().is_some()) {
            return;
        }

        let laf_css = gtk4::CssProvider::new();
        laf_css.load_from_resource("/live/arcturus/puppet-inspector/windows-look-and-feel.css");

        let color_css = gtk4::CssProvider::new();

        let display = gdk4::Display::default().expect("display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &color_css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        gtk4::style_context_add_provider_for_display(
            &display,
            &laf_css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        LAF_SINGLETON.with(|laf| {
            *laf.borrow_mut() = Some(Self {
                ui_settings: UISettings::new().expect("working windows environment"),
                color_css,
                laf_css,
            });
        });

        Self::with_singleton(|laf| {
            laf.update_color_css();

            laf.ui_settings
                .ColorValuesChanged(Some(&TypedEventHandler::new(|_, _| {
                    glib::idle_add_once(|| {
                        LAFProvider::with_singleton(|laf| laf.update_color_css());
                    });
                    Ok(())
                })))
                .expect("validly registered handler");
        });
    }

    fn with_singleton<T, F: FnOnce(&LAFProvider) -> T>(the_fn: F) -> T {
        Self::init();

        LAF_SINGLETON.with(|laf| {
            the_fn(
                laf.borrow()
                    .as_ref()
                    .expect("globals must stay initialized"),
            )
        })
    }

    fn update_color_css(&self) {
        let bg_color = self
            .ui_settings
            .GetColorValue(UIColorType::Background)
            .expect("bg color")
            .into_css_hex();
        let fg_color = self
            .ui_settings
            .GetColorValue(UIColorType::Foreground)
            .expect("fg color")
            .into_css_hex();
        let accent_color = self
            .ui_settings
            .GetColorValue(UIColorType::Accent)
            .expect("accent color")
            .into_css_hex();

        //I'm surprised GTK doesn't have a convenience method to define CSS variables.
        self.color_css.load_from_data(&format!(
            ":root {{
            --Windows-background: {bg_color};
            --Windows-foreground: {fg_color};
            --Windows-accent: {accent_color};
        }}"
        ));
    }
}

pub fn init() {
    LAFProvider::init();
}
