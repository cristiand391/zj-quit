use zellij_tile::prelude::*;

use std::collections::BTreeMap;

struct State {
    confirm_key: Key,
    cancel_key: Key,
}

impl Default for State {
    fn default() -> Self {
        Self {
            confirm_key: Key::Char('\n'),
            cancel_key: Key::Esc,
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
        if let Some(abort_key) = configuration.get("cancel_key") {
            self.cancel_key = abort_key.parse().unwrap_or(self.cancel_key);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => {
                if self.confirm_key == key {
                    quit_zellij()
                } else if self.cancel_key == key {
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

        let help_text = format!(
            "Help: <{}> - Confirm, <{}> - Cancel",
            key_name(self.confirm_key),
            key_name(self.cancel_key),
        );
        let help_text_y_location = rows - 1;
        let help_text_x_location = cols.saturating_sub(help_text.chars().count()) / 2;

        let confirm_key_length = key_name(self.confirm_key).chars().count();
        let abort_key_length = key_name(self.cancel_key).chars().count();

        print_text_with_coordinates(
            Text::new(help_text)
                .color_range(3, 6..8 + confirm_key_length)
                .color_range(
                    3,
                    20 + confirm_key_length..22 + confirm_key_length + abort_key_length,
                ),
            help_text_x_location,
            help_text_y_location,
            None,
            None,
        );
    }
}

fn key_name(key: Key) -> String {
    match key {
        Key::Char('\n') => "Enter".to_owned(),
        _ => key.to_string(),
    }
}
