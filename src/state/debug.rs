use std::time::SystemTime;

use parking_lot::Mutex;
use raylib::prelude::*;

use crate::{
    config::config,
    enums::Room,
    monster::{Monster, MonsterName},
};

use super::{Screen, State};

impl State<'_> {
    pub fn debug_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let s = Mutex::new(self);
        d.start_imgui(|ui| {
            ui.window("Debug")
                .size(
                    [config().width() as f32 / 4.0, 0.0],
                    ::imgui::Condition::FirstUseEver,
                )
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());
                    let mut se = s.lock();
                    ui.menu("Monsters", || {
                        ui.menu("Penny", || {
                            ui.set_window_font_scale(config().ui_scale());
                            ui.menu("Place in Hallway", || {
                                if ui.button("Stage 1") {
                                    se.gang.penny.set_room(Room::Room3);
                                    se.gang.penny.set_progress_to_hallway(0);
                                }
                                if ui.button("Stage 2") {
                                    se.gang.penny.set_room(Room::Room3);
                                    se.gang.penny.set_progress_to_hallway(1);
                                }
                                if ui.button("Stage 3") {
                                    se.gang.penny.set_room(Room::Room3);
                                    se.gang.penny.set_progress_to_hallway(2);
                                }
                            });
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::Penny);
                            }
                        });
                        ui.menu("Beastie", || {
                            ui.menu("Place in Hallway", || {
                                if ui.button("Stage 1") {
                                    se.gang.beastie.set_room(Room::Room5);
                                    se.gang.beastie.set_progress_to_hallway(0);
                                }
                                if ui.button("Stage 2") {
                                    se.gang.beastie.set_room(Room::Room5);
                                    se.gang.beastie.set_progress_to_hallway(1);
                                }
                                if ui.button("Stage 3") {
                                    se.gang.beastie.set_room(Room::Room5);
                                    se.gang.beastie.set_progress_to_hallway(2);
                                }
                            });
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::Beastie);
                            }
                        });
                        ui.menu("Wilber", || {
                            if se.gang.wilber.active() {
                                if ui.button("Deactivate") {
                                    se.gang.wilber.time_since_appeared = Some(SystemTime::now());
                                    se.gang.wilber.deactivate();
                                }
                            } else {
                                if ui.button("Activate") {
                                    se.gang.wilber.time_since_appeared = Some(SystemTime::now());
                                    se.gang.wilber.activate();
                                }
                            }
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::Wilber);
                            }
                            ui.separator();
                            let mut rage = se.gang.wilber.rage() as i32;
                            ui.slider("Rage Meter", 0, 100, &mut rage);
                            if rage as f32 != se.gang.wilber.rage() {
                                se.gang.wilber.set_rage(rage as f32);
                            }
                            ui.text(format!("Stage: {}", se.gang.wilber.stage));
                        });
                        ui.menu("GoGopher", || {
                            if se.gang.gogopher.active() {
                                if ui.button("Deactivate") {
                                    se.gang.gogopher.deactivate();
                                }
                            } else {
                                if ui.button("Activate") {
                                    se.gang.gogopher.activate();
                                }
                            }
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::GoGopher);
                            }
                            if ui.button("Place in Vent") {
                                se.gang.gogopher.set_room(Room::Room4)
                            }
                        });
                        ui.menu("Tux", || {
                            if se.gang.tux.active() {
                                if ui.button("Deactivate") {
                                    se.gang.tux.deactivate();
                                }
                            } else {
                                if ui.button("Activate") {
                                    se.gang.tux.activate();
                                }
                            }
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::Tux);
                            }
                        });
                        ui.menu("Golden Tux", || {
                            if se.gang.golden_tux.active() {
                                if ui.button("Deactivate") {
                                    se.gang.golden_tux.appeared = SystemTime::now();
                                    se.gang.golden_tux.deactivate();
                                }
                            } else {
                                if ui.button("Activate") {
                                    se.gang.golden_tux.appeared = SystemTime::now();
                                    se.gang.golden_tux.activate();
                                }
                            }
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::GoldenTux);
                            }
                        });
                        // Sure fuck it why not
                        ui.menu("Nolok", || {
                            if se.gang.nolok.active() {
                                if ui.button("Deactivate") {
                                    se.gang.nolok.deactivate();
                                }
                            } else {
                                if ui.button("Activate") {
                                    se.gang.nolok.activate();
                                }
                            }
                            if ui.button("Jumpscare") {
                                se.debug_set_jumpscarer(MonsterName::Nolok);
                            }
                        });
                    });

                    ui.slider("Battery", 0.0, 100.0, &mut se.camera_timer);
                    ui.slider("Tainted", 0.0, 100.0, &mut se.tainted);
                    if se.camera_timer == 0.0 {
                        se.camera_booting = true;
                        se.sel_camera = Room::Room1;
                        se.screen = Screen::Office;
                    }
                });
        });
        Ok(())
    }

    fn debug_set_jumpscarer(&mut self, mons: MonsterName) {
        if self.jumpscarer == MonsterName::None {
            self.going_to_office = true;
            self.jumpscarer = mons;
            self.gameover_time = SystemTime::now();
            self.getting_jumpscared = true;
        }
    }
}
