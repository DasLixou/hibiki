use std::path::PathBuf;

use eframe::egui::{self, Ui, Widget};
use rodio::{OutputStreamHandle, Sink, Source};

use crate::{
    scene::{Scene, SceneEntry},
    sound::{Sound, SoundKind, SoundSource},
    trigger::Trigger,
};

pub struct Board {
    sounds: Vec<Sound>,
    stream_handle: OutputStreamHandle,
    selected_controller: Option<usize>,
    scene_path: PathBuf,
}

impl Board {
    pub fn new(scene_path: PathBuf, stream_handle: OutputStreamHandle) -> Self {
        Self {
            sounds: Self::load_sounds(&scene_path, &stream_handle),
            stream_handle,
            selected_controller: None,
            scene_path,
        }
    }

    fn load_sounds(scene_path: &PathBuf, stream_handle: &OutputStreamHandle) -> Vec<Sound> {
        let scene = Scene::load(&scene_path);
        scene
            .entries
            .iter()
            .map(|entry| Sound {
                kind: entry.controller,
                source: SoundSource::from_file(entry.sound_path.clone()).unwrap(),
                sink: Sink::try_new(&stream_handle).unwrap(),
                state: false,
                volume: 1.0,
                pan: 1.0,
            })
            .collect()
    }

    fn pack_scene(&self) -> Scene {
        Scene {
            entries: self
                .sounds
                .iter()
                .map(|sound| SceneEntry {
                    sound_path: sound.source.path.clone(),
                    controller: sound.kind,
                })
                .collect(),
        }
    }

    pub fn selected_controller_mut(&mut self) -> Option<&mut Sound> {
        if let Some(index) = self.selected_controller {
            self.sounds.get_mut(index)
        } else {
            None
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        egui::Window::new("Board").show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label(self.scene_path.file_name().unwrap().to_str().unwrap());
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Open Scene")
                        .add_filter("Hibiki Scene", &["hibiki.ron"])
                        .pick_file()
                    {
                        self.sounds = Self::load_sounds(&path, &self.stream_handle);
                        self.scene_path = path;
                    }
                }
                if ui.button("Reload").clicked() {
                    self.sounds = Self::load_sounds(&self.scene_path, &self.stream_handle);
                }
                if ui.button("Save").clicked() {
                    let scene = self.pack_scene();
                    scene.save(&self.scene_path);
                }
                if ui.button("Save as").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Save scene as")
                        .set_file_name("my_scene.hibiki.ron")
                        .save_file()
                    {
                        let scene = self.pack_scene();
                        scene.save(&path);
                        self.scene_path = path;
                    }
                }
            });
            if ui.button("Add Sound").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Sound File", &["mp3", "wav", "flac", "ogg"])
                    .pick_file()
                {
                    // necessary on Linux as file dialog allows selecting dirs
                    if path.is_file() {
                        self.sounds.push(Sound {
                            kind: SoundKind::Trigger,
                            source: SoundSource::from_file(path).unwrap(),
                            sink: Sink::try_new(&self.stream_handle).unwrap(),
                            state: false,
                            volume: 1.0,
                            pan: 1.0,
                        });
                    }
                }
                // TODO: show error on fail
            }
            ui.horizontal(|ui| {
                // TODO: make this a grid instead of horizontal
                // TODO: can we make the buttons more easily distinguishable?
                for (i, sound) in self.sounds.iter_mut().enumerate() {
                    if Self::sound_trigger(ui, sound, &self.stream_handle) {
                        if self.selected_controller.is_some_and(|index| index == i) {
                            self.selected_controller = None;
                        } else {
                            self.selected_controller = Some(i);
                        }
                    }
                }
            });
        });
    }

    fn sound_trigger(ui: &mut Ui, sound: &mut Sound, stream_handle: &OutputStreamHandle) -> bool {
        let trigger = Trigger {}.ui(ui);
        match sound.kind {
            SoundKind::Trigger if trigger.clicked() => {
                let source = sound.source.source();
                stream_handle.play_raw(source.convert_samples()).unwrap();
            }
            SoundKind::CutItself if trigger.clicked() => {
                let source = sound.source.source();
                sound.sink.clear();
                sound.sink.append(source);
                sound.sink.play();
            }
            SoundKind::Hold if trigger.drag_started() => {
                let source = sound.source.source();
                sound.sink.clear();
                sound.sink.append(source);
                sound.sink.play();
            }
            SoundKind::Hold if trigger.drag_released() => {
                sound.sink.stop();
            }
            SoundKind::HoldRepeat if trigger.drag_started() => {
                let source = sound.source.source();
                sound.sink.clear();
                sound.sink.append(source.repeat_infinite());
                sound.sink.play();
            }
            SoundKind::HoldRepeat if trigger.drag_released() => {
                sound.sink.stop();
            }
            SoundKind::Toggle if trigger.clicked() => {
                if sound.state {
                    sound.state = false;
                    sound.sink.clear();
                } else {
                    sound.state = true;
                    let source = sound.source.source();
                    sound.sink.clear();
                    sound.sink.append(source);
                    sound.sink.play();
                }
            }
            SoundKind::ToggleRepeat if trigger.clicked() => {
                if sound.state {
                    sound.state = false;
                    sound.sink.clear();
                } else {
                    sound.state = true;
                    let source = sound.source.source();
                    sound.sink.clear();
                    sound.sink.append(source.repeat_infinite());
                    sound.sink.play();
                }
            }
            _ => {}
        }
        trigger.secondary_clicked()
    }
}
