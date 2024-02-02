use ggez::graphics::{self, Rect, Drawable, Mesh, Color, Canvas};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use rand::Rng;
use ggez::{Context};

use crate::Weed;
use crate::constants;

#[derive(Debug)]
pub struct Agent {
    pub rect: Rect,
    energy: f32,
    // vision_area: i8,
}

impl Agent {
    pub fn make_agent() -> Agent {
        let x = rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32;
        let y = rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32;
        Agent {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: 100.0,
            // touch_area: Rect::new(),
            // vision_area: 3,
        }
    }

    pub fn draw_agent(ctx: &Context, a: Rect, canvas: &mut Canvas) {
        let agent = Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(), a, Color::BLUE,
        ).unwrap();
        Drawable::draw(&agent, canvas, graphics::DrawParam::default())
    }

    pub fn do_agent(&mut self, i: i32, weeds: &Vec<Weed>, dead_bot: &mut Vec<i32>) {
        self.energy -= 0.1;
        if self.energy > 0.0 {
            Agent::move_bot(&mut self.rect, weeds);
        } else {
            dead_bot.push(i);
        }
    }

    // fn touch() {
    //     let cells =
    // }

    pub fn move_bot(bot: &mut Rect, weeds: &Vec<Weed>) {
        let mut index = 0;
        let mut min_distance = f32::MAX;
        for (i, w) in weeds.iter().enumerate() {
            let dx = (bot.x - w.rect.x).abs();
            let dy = (bot.y - w.rect.y).abs();
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            if distance < min_distance {
                index = i;
                min_distance = distance;
            }
        }
        // Вычисляем вектор направления от bot к weeds[index]
        let mut direction_x = weeds[index].rect.x - bot.x;
        let mut direction_y = weeds[index].rect.y - bot.y;
        // Вычисляем длину вектора направления
        let length = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
        // Нормализуем вектор направления, деля его на длину
        direction_x /= length;
        direction_y /= length;
        // Прибавляем вектор направления к координатам bot
        bot.x += direction_x;
        bot.y += direction_y;
    }

    pub fn check_collision(agent: &Rect, weeds: &Vec<Weed>) -> Vec<usize> {
        let player_rect = agent;
        let mut indexes_to_remove = Vec::new();
        for (i, gr) in weeds.iter().enumerate() {
            let rectangle_rect = gr.rect;
            if player_rect.overlaps(&rectangle_rect) {
                indexes_to_remove.push(i);
            }
        }
        indexes_to_remove
    }

    pub fn move_agent(rect: &mut Rect, _ctx: &KeyboardContext) {
        let k_ctx = &_ctx;
        if k_ctx.is_key_pressed(KeyCode::D) {
            rect.x += 1.0;
        }
        else if k_ctx.is_key_pressed(KeyCode::A) {
            rect.x -= 1.0;
        }
        else if k_ctx.is_key_pressed(KeyCode::S) {
            rect.y += 1.0;
        }
        else if k_ctx.is_key_pressed(KeyCode::W) {
            rect.y -= 1.0;
        }
    }
}

