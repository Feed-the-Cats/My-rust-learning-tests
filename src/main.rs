use eframe::egui;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Simple Text Editor",
        options,
        Box::new(|_cc| Box::new(TextEditor::default())),
    )
}

struct TextEditor {
    content: String,
    current_file: Option<PathBuf>,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            content: String::new(),
            current_file: None,
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Ajouter une barre de menus
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open File").clicked() {
                        self.open_file();
                    }
                    if ui.button("Save").clicked() {
                        self.save();
                    }
                    if ui.button("Save File").clicked() {
                        self.save_as();
                    }
                });
            });
        });

        // Détecter les raccourcis clavier
        ctx.input(|i| {
            if i.key_pressed(egui::Key::S) && i.modifiers.command && !i.modifiers.shift {
                self.save();
            }
            if i.key_pressed(egui::Key::S) && i.modifiers.command && i.modifiers.shift {
                self.save_as();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            /* ui.horizontal(|ui| {
                if ui.button("Open File").clicked() {
                    self.open_file();
                }

                if ui.button("Save File").clicked() {
                    self.save_as();
                }

                if ui.button("Save").clicked() {
                    self.save();
                }
            });

            ui.separator(); */

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.content).font(egui::TextStyle::Monospace),
                );
            });
        });
    }
}

impl TextEditor {
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("text", &["txt"])
            .pick_file()
        {
            if let Ok(content) = fs::read_to_string(&path) {
                self.content = content;
                self.current_file = Some(path.clone()); // Clone the path here
                println!("File loaded, content length: {}", self.content.len());
            }
        }
    }

    fn save(&mut self) {
        if let Some(path) = &self.current_file {
            if let Ok(mut file) = fs::File::create(path) {
                if let Err(e) = file.write_all(self.content.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                } else {
                    println!("File saved successfully!");
                }
            }
        } else {
            self.save_as(); // Call save_as if no file is currently open
        }
    }

    fn save_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("text", &["txt"])
            .save_file()
        {
            if let Ok(mut file) = fs::File::create(&path) {
                // Use a reference to path here
                if let Err(e) = file.write_all(self.content.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                } else {
                    println!("File saved successfully!");
                    self.current_file = Some(path); // Move the path here
                }
            }
        }
    }
}
