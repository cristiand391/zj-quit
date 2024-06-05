use zellij_tile::prelude::*;

use std::collections::BTreeMap;

struct State {
    confirm_key: Key,
    abort_key: Key,
}

impl Default for State {
    fn default() -> Self {
        Self {
            confirm_key: Key::Char('\n'),
            abort_key: Key::Esc,
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        subscribe(&[EventType::Key]);

        if let Some(confirm_key) = configuration.get("confirm_key") {
            self.confirm_key = confirm_key.parse().unwrap_or(self.confirm_key);
        }
        if let Some(abort_key) = configuration.get("abort_key") {
            self.abort_key = abort_key.parse().unwrap_or(self.abort_key);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => {
                if self.confirm_key == key {
                    quit_zellij()
                } else if self.abort_key == key {
                    hide_self();
                }
            }
            _ => (),
        };
        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let confirmation_text = "Are you sure you want to kill the current session?".to_string();
        let confirmation_y_location = (rows / 2) - 2;
        let confirmation_x_location = cols.saturating_sub(confirmation_text.chars().count()) / 2;

        print_text_with_coordinates(
            Text::new(confirmation_text),
            confirmation_x_location,
            confirmation_y_location,
            None,
            None,
        );

        let help_text = "Help: <ENTER> - Confirm, <ESC> - Cancel";
        let help_text_y_location = rows - 1;
        let help_text_x_location = cols.saturating_sub(help_text.chars().count()) / 2;

        print_text_with_coordinates(
            Text::new(help_text)
                .color_range(3, 6..13)
                .color_range(3, 25..30),
            help_text_x_location,
            help_text_y_location,
            None,
            None,
        );
    }
}
