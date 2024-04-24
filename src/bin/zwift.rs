use std::collections::BTreeMap;
use std::path::Path;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        hide_self();
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.name == "zwift_selection" {
            match pipe_message.payload {
                Some(payload) => {
                    let cwd = Path::new(payload.trim());
                    let name = cwd.file_stem().unwrap().to_str().unwrap();
                    eprintln!("Selected project: {name}, cwd {payload}");
                    switch_session_with_layout(
                        Some(name),
                        LayoutInfo::File("dev".into()),
                        Some(cwd.to_path_buf()),
                    );
                }
                _ => {
                    eprintln!("No payload in message");
                }
            }
        }
        close_self();
        false
    }

    fn update(&mut self, _event: Event) -> bool {
        close_self();
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        close_self();
    }
}
