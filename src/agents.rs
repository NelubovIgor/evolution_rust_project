use ggez::graphics::{self, Rect, Drawable, Mesh, Color, Canvas};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use rand::Rng;
use rand::seq::SliceRandom;
use ggez::{Context};

use crate::World;
use crate::Weed;
use crate::constants;

#[derive(Debug)]
pub struct Agent {
    pub rect: Rect,
    energy: f32,
    pos: u32,
    vision_area: i8,
    pub color: char,
}

impl Agent {
    pub fn make_agent(world: &mut Vec<World>) -> Agent {
        let x = rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32;
        let y = rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32;
        let possition = (y * constants::HEIGHT + x) as u32;
        let cell = world.get_mut(possition as usize).unwrap();
        cell.color = 'c';
        let agent = Agent {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: 10.0,
            pos: possition,
            vision_area: 10,
            color: 'c',
        };
        agent
    }

    pub fn draw_agent(ctx: &Context, a: Rect, canvas: &mut Canvas) {
        let agent = Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(), a, Color::BLUE,
        ).unwrap();
        Drawable::draw(&agent, canvas, graphics::DrawParam::default())
    }

    pub fn do_agent(&mut self, i: i32, weeds: &Vec<Weed>, dead_bot: &mut Vec<i32>, world: &Vec<World>) {
        self.energy -= 0.1;
        if self.energy > 0.0 {
            // let (feel_weed, feel_agent, feel_path) = Agent::touch(&self, world);
            // if !feel_weed.is_empty() { // ! - инвертирует запрос is_empty(), который возвращает true если список пуст
            //     let eat_weed = feel_weed.choose(&mut rand::thread_rng());
            //     Agent::eat(&self, eat_weed);

            // } else if !feel_agent.is_empty() && self.energy > 300.0 {
            //     Agent::reproduction(&self);

            // } else if feel_path.len() == 8 {
            //     let (see_agent, see_weed) = Agent::vision_bot(&self);
            //     if !see_agent.is_empty() && self.energy > 300.0 {
            //         Agent::::move_bot(&mut self.rect, &see_agent); 
            //     } else if !see_weed.is_empty() {
            //         Agent::::move_bot(&mut self.rect, &see_weed);
            //     }
            // }

            Agent::move_bot(&mut self.rect, weeds);
        } else {
            dead_bot.push(i);
        }
    }

    fn touch(bot: &Agent, cells: &Vec<World>) -> (Vec<World>, Vec<World>, Vec<World>) {
        let mut cells_around = Vec::new();
        for p in constants::POINTS.iter() {
            // Вычисляем новые координаты
            let new_x = bot.rect.x as isize + p.x as isize;
            let new_y = bot.rect.y as isize + p.y as isize;
    
            // Проверяем, что координаты не выходят за границы массива
            if new_x >= 0 && new_x < constants::WIDTH as isize && new_y >= 0 && new_y < constants::HEIGHT as isize {
                let cell = &cells[(new_y as f32 * constants::HEIGHT + new_x as f32) as usize];
                cells_around.push(cell);
            }
        }
        let feel_weed: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'g').cloned().collect();
        let feel_agent: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'c').cloned().collect();
        let feel_path: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'b').cloned().collect();
        
        (feel_weed, feel_agent, feel_path)
    }

    // fn vision_bot() {

    // }

    // fn reproduction() {

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

    // fn eat() {

    // }

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

