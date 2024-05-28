use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    current_session_name: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::SessionUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::SessionUpdate(session_infos, _) => {
                if let Some(current_session) = session_infos.iter().find(|s| s.is_current_session) {
                    self.current_session_name = current_session.name.clone();
                    should_render = true;
                }
            }
            Event::Key(key) => {
                if let Key::Char('\n') = key {
                    kill_sessions(&[self.current_session_name.clone()])
                } else if let Key::Esc = key {
                    hide_self();
                }
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        if !self.current_session_name.is_empty() {
            println!(
                "Are you sure you want to kill the current session ({})?",
                self.current_session_name
            );
            println!("Press Enter to confirm.");
            println!("Press Esc to close this pane.");
        }
    }
}
