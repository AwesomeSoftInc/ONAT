use std::time::{SystemTime, UNIX_EPOCH};

use crate::{config::config, monster::MonsterName};

use super::{Screen, State};
use rand::{thread_rng, Rng};
use raylib::prelude::*;

pub enum MikeDirection {
    Left,
    Right,
}

impl State<'_> {
    pub fn mikepong_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);

        d.clear_background(Color::BLACK);
        d.draw_rectangle(25, self.mikepong_pos as i32 - 100, 25, 200, Color::WHITE);
        d.draw_rectangle(
            config().width() - 25,
            self.mikeopppong_pos as i32 - 100,
            25,
            200,
            Color::WHITE,
        );
        d.draw_texture_v(&*self.textures.mike.mike(), self.mikeball_pos, Color::WHITE);

        let score = format!("{}", self.mike_score);
        let v = d.measure_text(&score, 32);
        d.draw_text(
            &score,
            (config().width() / 2) - (v / 2),
            32,
            32,
            Color::WHITE,
        );
        Ok(())
    }
    pub fn mikepong_input(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if d.is_key_down(KeyboardKey::KEY_W) {
            self.mikepong_pos -= 10.0
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            self.mikepong_pos += 10.0
        }

        Ok(())
    }

    pub fn mike_step(&mut self) {
        let speed = 5.0 + (self.mike_hits as f32);
        match self.mike_dir {
            MikeDirection::Left => {
                self.mikeball_pos.x -= speed;
                self.mikeball_pos.y += self.mikeball_angle;
            }
            MikeDirection::Right => {
                self.mikeball_pos.x += speed;
                self.mikeball_pos.y += self.mikeball_angle;
            }
        }

        if self.mikeball_pos.x <= 25.0 {
            if self.mikeball_pos.y >= self.mikepong_pos - 200.0
                && self.mikeball_pos.y <= self.mikepong_pos + 200.0
            {
                self.mike_dir = MikeDirection::Right;
                self.mikeball_angle = (self.mikeball_pos.y - self.mikepong_pos) / 50.0;
                self.mike_hits += 1;
            }
        }
        if self.mikeball_pos.x >= config().width() as f32 - 50.0 {
            if self.mikeball_pos.y >= self.mikeopppong_pos - 50.0
                && self.mikeball_pos.y <= self.mikeopppong_pos
            {
                self.mike_dir = MikeDirection::Left;
                self.mikeball_angle = (self.mikeopppong_pos - self.mikeball_pos.y) / 50.0;
                self.mike_hits += 1;
            }
        }

        let opp_speed = (self.mike_score as f32 / 10.0).clamp(1.0, 5.0);
        if self.mikeopppong_pos >= self.mikeball_pos.y {
            if self.mikeopppong_speed > -5.0 {
                self.mikeopppong_speed -= 1.0 * opp_speed;
            }
        } else if self.mikeopppong_pos <= self.mikeball_pos.y {
            if self.mikeopppong_speed < 5.0 {
                self.mikeopppong_speed += 1.0 * opp_speed;
            }
        }

        self.mikeopppong_pos += self.mikeopppong_speed;

        if self.mikeball_pos.y <= 0.0 || self.mikeball_pos.y >= config().height() as f32 {
            self.mikeball_angle = -self.mikeball_angle;
        }

        if self.mikeball_pos.x >= config().width() as f32 + 50.0 {
            let rand_y = thread_rng().gen_range(-500.0..500.0);
            self.mikeball_pos = Vector2::new(
                config().width() as f32 / 2.0,
                (config().height() as f32 / 2.0) + rand_y,
            );
            self.mike_score += 1;
            self.mike_hits = self.mike_score / 5;
        }
        if self.mikeball_pos.x <= -50.0 {
            self.screen = Screen::Office;
            self.going_to_office = true;
            self.jumpscarer = MonsterName::Tux;
            self.gameover_time = SystemTime::now();
            self.getting_jumpscared = true;
            self.ingame_time = UNIX_EPOCH;
            self.audio.mike.halt();
            self.laptop_offset_y = config().height() as f64;
        }
    }

    pub fn goto_mikepong(&mut self, d: &mut RaylibHandle) {
        self.mikepong_pos = config().height() as f32 / 2.0;
        self.mikeopppong_pos = config().height() as f32 / 2.0;
        self.mikeopppong_speed = 5.0;
        self.mikeball_pos = Vector2::new(
            config().width() as f32 / 2.0,
            config().height() as f32 / 2.0,
        );
        self.mike_dir = MikeDirection::Left;
        self.mike_score = 0;
        self.mike_hits = 0;
        self.mikeball_angle = 0.0;
        self.screen = Screen::MikePong;
        self.audio.halt_title(self.has_won);
        d.set_target_fps(60);
        self.audio.mike.play_loop().unwrap();
    }
}
