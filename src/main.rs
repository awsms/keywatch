use std::collections::BTreeSet;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "KeyWatch",
        options,
        Box::new(|_cc| Ok(Box::new(KeyWatchApp::default()))),
    )
}

#[derive(Default)]
struct KeyWatchApp {
    held: BTreeSet<egui::Key>,
    // held_physical: BTreeSet<egui::Key>,
    last_pressed: Option<egui::Key>,
    last_released: Option<egui::Key>,
    last_released_count: u32,
    last_text: String,
    last_physical: String,
}

impl eframe::App for KeyWatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let events = ctx.input(|i| i.events.clone());

        for ev in events {
            match ev {
                egui::Event::Key {
                    key,
                    pressed,
                    repeat: _,
                    modifiers: _,
                    physical_key,
                } => {
                    // logical key (layout-aware-ish)
                    if pressed {
                        self.held.insert(key);
                        self.last_pressed = Some(key);
                    } else {
                        self.held.remove(&key);

                        // count consecutive releases of the same key
                        match self.last_released {
                            Some(prev) if prev == key => {
                                self.last_released_count += 1;
                            }
                            _ => {
                                self.last_released = Some(key);
                                self.last_released_count = 1;
                            }
                        }
                    }

                    // physical key (if available). helps debug layout vs scancode-ish behavior.
                    // physical_key is an Option in recent egui.
                    if let Some(pk) = physical_key {
                        self.last_physical = format!("{pk:?}");
                    }
                }
                egui::Event::Text(s) => {
                    self.last_text = s;
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("KeyWatch");
            ui.label("Click this window, then press/hold keys.");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Held keys (logical):");
                if self.held.is_empty() {
                    ui.monospace("(none)");
                } else {
                    let held = self
                        .held
                        .iter()
                        .map(|k| format!("{k:?}"))
                        .collect::<Vec<_>>()
                        .join(" + ");
                    ui.monospace(held);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Last pressed:");
                ui.monospace(match self.last_pressed {
                    Some(k) => format!("{k:?}"),
                    None => "(none)".to_string(),
                });
            });

            ui.horizontal(|ui| {
                ui.label("Last released:");
                ui.monospace(match self.last_released {
                    Some(k) => {
                        if self.last_released_count > 1 {
                            format!("{k:?} (x{})", self.last_released_count)
                        } else {
                            format!("{k:?}")
                        }
                    }
                    None => "(none)".to_string(),
                });
            });

            ui.horizontal(|ui| {
                ui.label("Last text typed:");
                if self.last_text.is_empty() {
                    ui.monospace("(none)");
                } else {
                    ui.monospace(&self.last_text);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Last physical key:");
                if self.last_physical.is_empty() {
                    ui.monospace("(none)");
                } else {
                    ui.monospace(&self.last_physical);
                }
            });

            ui.separator();
            ui.small("Tip: releasing the same key multiple times will increase the counter (x2, x3, etc)");
        });

        ctx.request_repaint();
    }
}
