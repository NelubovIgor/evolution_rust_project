use ggez::graphics::{self, Rect, Drawable, Mesh, Color, Canvas};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use rand::Rng;
use rand::seq::SliceRandom;
use ggez::{Context};
// use std::cmp::Ordering;

use crate::World;
use crate::Weed;
use crate::constants;

#[derive(Debug, Copy, Clone)]
pub struct Agent {
    pub rect: Rect,
    pub energy: f32,
    pub pos: u32,
    vision_area: f32,
    pub color: char,
}

pub enum Return {
    Int(usize),
    NewBot((f32, f32)),
    Weed(Weed),
    Move(Agent),
    Sleep(Agent),
}

impl Agent {
    pub fn make_agent(world: &mut Vec<World>, x: Option<f32>, y: Option<f32>) -> Agent {
        let x = match x {
            Some(x) => x,
            None => rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32,
        };
        let y = match y {
            Some(y) => y,
            None => rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32,
        };
        let possition = (y * constants::HEIGHT + x) as u32;
        let cell = world.get_mut(possition as usize).unwrap();
        cell.color = 'c';
        cell.energy = 100.0;
        let agent = Agent {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: 100.0,
            pos: possition,
            vision_area: 100.0,
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

    pub fn do_agent(&mut self, i: i32, weeds: &mut Vec<Weed>, world: &mut Vec<World>) -> Return {
        self.energy -= 0.1;
        if self.energy > 0.0 {
            // узнаёт, что чувствует агент
            let (feel_weed, feel_agent, feel_path) = Agent::touch(self, world);            
            // let mut feel_agents: Vec<Agent> = Vec::new();
            
            // for a in feel_agent.iter() {
            //     feel_agents.push(*agents.iter().find(|b| b.pos == a.pos).unwrap());
            // }
            if !feel_weed.is_empty() { // ! - инвертирует запрос is_empty(), который возвращает true если список пуст
                println!("feel_weed сработал");
                let eat_weed = feel_weed.choose(&mut rand::thread_rng()).unwrap();
                Return::Weed(Agent::eat(self, &eat_weed, world, weeds))

            } else if !feel_agent.is_empty() && self.energy > 200.0 && !feel_path.is_empty() {
                println!("feel_agent сработал");
                let birth_place = feel_path.choose(&mut rand::thread_rng()).unwrap();
                // let agent_sex = feel_agents.iter().max_by_key(|e| e.energy);
                // let mut agent_sex = feel_agent.iter_mut().max_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap_or(Ordering::Equal));
                Return::NewBot(Agent::reproduction(self, birth_place))

            } else if feel_path.len() == 8 {

                let (mut see_agent, mut see_weed) = Agent::vision_bot(&self, world);
                if !see_agent.is_empty() && self.energy > 200.0 {
                    println!("see_agent сработал");
                    Agent::move_bot(self, &mut see_agent);
                    Return::Move(*self)
                } else if !see_weed.is_empty() {
                    println!("see_weed сработал");
                    Agent::move_bot(self, &mut see_weed);
                    Return::Move(*self)
                } else {
                    //sleep
                    Return::Sleep(*self)
                }

            } else {
                //sleep
                Return::Sleep(*self)
            }

            // Agent::move_bot(&mut self.rect, weeds);
        } else {
            Return::Int(i.try_into().unwrap())
        }
    }

    fn touch(&mut self, cells: &mut Vec<World>) -> (Vec<World>, Vec<World>, Vec<World>) {
        // println!("touch сработал");
        let mut cells_around = Vec::new();
        for p in constants::POINTS.iter() {
            // Вычисляем новые координаты
            let new_x = self.rect.x as isize + p.x as isize;
            let new_y = self.rect.y as isize + p.y as isize;
    
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

    fn eat(&mut self, food: &World, world: &mut Vec<World>, weeds: &mut Vec<Weed>) -> Weed {
        let old_pos = &mut world[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
        old_pos.color = 'b';
        old_pos.energy = 0.0;
        let new_pos = &mut world[(food.rect.y as f32 * constants::HEIGHT + food.rect.x as f32) as usize];
        new_pos.color = 'c';
        new_pos.energy = self.energy;
        self.rect.x = food.rect.x;
        self.rect.y = food.rect.y;
        self.energy += 30.0;
        weeds.iter().find(|p| p.pos == food.pos).unwrap().clone()
        // *weeds.iter().find(|p| p.pos == food.pos).unwrap()
    } 

    fn vision_bot(&self, cells: &mut Vec<World>) -> (Vec<World>, Vec<World>) {
        let mut see_weeds = Vec::new();
        let mut see_agents = Vec::new();
        let left = (self.rect.x - self.vision_area).max(0.0); // метод выбора максимального значения
        let right = (self.rect.x + self.vision_area).min(constants::WIDTH); // метод выбора минимального значения
        let bottom = (self.rect.y + self.vision_area).min(constants::HEIGHT);
        let top = (self.rect.y - self.vision_area).max(0.0);
        for x in left as u32..=right as u32 {
            for y in bottom as u32..=top as u32 {
                let cell = cells.iter().find(|c| c.x == x && c.y == y).unwrap();
                if cell.color == 'g' {
                    see_weeds.push(cell.clone());
                } else if cell.color == 'c' {
                    see_agents.push(cell.clone());
                }
            }
        }
        // println!("{}", &see_weeds.len());
        (see_agents, see_weeds)
    }
    fn reproduction(agent1: &mut Agent, birth_place: &World) -> (f32, f32) {
    // fn reproduction(&mut self, agent: &mut Agent, world: &mut Vec<World>, birth_place: &World) -> Agent {
        // let a = agents.iter().find(|p| p.pos == agent.pos);
        // agent2.energy -= 20.0;
        agent1.energy -= 80.0;
        (birth_place.rect.x, birth_place.rect.y)
    }

    pub fn move_bot(&mut self, world: &mut Vec<World>) {
        let old_pos = &mut world[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
        old_pos.color = 'b';
        old_pos.energy = 0.0;
        let mut index = 0;
        let mut min_distance = f32::MAX;
        for (i, w) in world.iter().enumerate() {
            let dx = (self.rect.x - w.rect.x).abs();
            let dy = (self.rect.y - w.rect.y).abs();
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            if distance < min_distance {
                index = i;
                min_distance = distance;
            }
        }
        // Вычисляем вектор направления от bot к world[index]
        let mut direction_x = world[index].rect.x - self.rect.x;
        let mut direction_y = world[index].rect.y - self.rect.y;
        // Вычисляем длину вектора направления
        let length = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
        // Нормализуем вектор направления, деля его на длину
        direction_x /= length;
        direction_y /= length;
        // Прибавляем вектор направления к координатам bot
        self.rect.x += direction_x;
        self.rect.y += direction_y;
        let new_pos = &mut world[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
        new_pos.color = 'c';
        new_pos.energy = self.energy;
    }

// старая версия кода движения ботов:    
    // pub fn move_bot(bot: &mut Rect, weeds: &Vec<Weed>) {
    //     let mut index = 0;
    //     let mut min_distance = f32::MAX;
    //     for (i, w) in weeds.iter().enumerate() {
    //         let dx = (bot.x - w.rect.x).abs();
    //         let dy = (bot.y - w.rect.y).abs();
    //         let distance = (dx.powi(2) + dy.powi(2)).sqrt();
    //         if distance < min_distance {
    //             index = i;
    //             min_distance = distance;
    //         }
    //     }
    //     // Вычисляем вектор направления от bot к weeds[index]
    //     let mut direction_x = weeds[index].rect.x - bot.x;
    //     let mut direction_y = weeds[index].rect.y - bot.y;
    //     // Вычисляем длину вектора направления
    //     let length = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
    //     // Нормализуем вектор направления, деля его на длину
    //     direction_x /= length;
    //     direction_y /= length;
    //     // Прибавляем вектор направления к координатам bot
    //     bot.x += direction_x;
    //     bot.y += direction_y;
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

