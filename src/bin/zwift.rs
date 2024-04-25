use std::collections::BTreeMap;
use std::path::Path;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    active_session: Option<SessionInfo>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::SessionUpdate]);
        set_selectable(false);
        hide_self();
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.name == "zwift_selection" {
            match pipe_message.payload {
                Some(payload) => {
                    let cwd = Path::new(payload.trim());
                    let name = cwd.file_stem().unwrap().to_str().unwrap();
                    eprintln!("Selected project: {name}, cwd {payload}");
                    if self.active_session.is_none()
                        || self.active_session.as_ref().unwrap().name != name
                    {
                        switch_session_with_layout(
                            Some(name),
                            LayoutInfo::File("dev".into()),
                            Some(cwd.to_path_buf()),
                        );
                    };
                }
                _ => {
                    eprintln!("No payload in message");
                }
            }
        }
        hide_self();
        false
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::SessionUpdate(session_infos, _) = event {
            self.active_session = session_infos
                .into_iter()
                .find(|session_info| session_info.is_current_session);
        }
        hide_self();
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        hide_self();
    }
}
