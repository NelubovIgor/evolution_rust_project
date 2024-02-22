use ggez::graphics::{self, Rect, Drawable, Mesh, Color, Canvas};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use rand::Rng;
use rand::seq::SliceRandom;
use ggez::Context;
use nalgebra::geometry::Point2;
// use std::cmp::Ordering;

use crate::World;
use crate::Weed;
use crate::constants;

#[derive(Debug, Copy, Clone)]
pub struct Agent {
    pub rect: Rect,
    pub energy: f32,
    vision_area: f32,
    pub color: char,
}

pub enum Return {
    Int(usize),
    NewBot((Point2<f32>, Point2<f32>, Point2<f32>)),
    Weed(Point2<f32>),
    Move(Agent),
    Sleep(Agent),
}

impl Agent {
    pub fn make_agent(world: &mut Vec<World>, x: Option<f32>, y: Option<f32>) -> Agent {
        let x: f32 = match x {
            Some(x) => x,
            None => rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32,
        };
        let y: f32 = match y {
            Some(y) => y,
            None => rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32,
        };
        let possition: u32 = (y * constants::HEIGHT + x) as u32;
        let cell: &mut World = world.get_mut(possition as usize).unwrap();
        cell.color = 'c';
        let energy: f32= 100.0;
        cell.energy = energy;
        let agent: Agent = Agent {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: energy,
            vision_area: 150.0,
            color: 'c',
        };
        agent
    }

    pub fn draw_agent(ctx: &Context, a: Rect, canvas: &mut Canvas) {
        let agent: Mesh = Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(), a, Color::BLUE,
        ).unwrap();
        Drawable::draw(&agent, canvas, graphics::DrawParam::default())
    }

    pub fn do_agent(&mut self, i: i32, world: &mut Vec<World>) -> Return {
        // println!("{:?}", self.rect.x);
        self.energy -= 0.1;
        if self.energy > 0.0 {
            // узнаёт, что чувствует агент
            let (feel_weed, feel_agent, feel_path) = Agent::touch(self, world);            
            // let mut feel_agents: Vec<Agent> = Vec::new();
            
            // for a in feel_agent.iter() {
            //     feel_agents.push(*agents.iter().find(|b| b.pos == a.pos).unwrap());
            // }
            if !feel_weed.is_empty() { // ! - инвертирует запрос is_empty(), который возвращает true если список пуст
                // println!("feel_weed сработал");
                let eat_weed: &World = feel_weed.choose(&mut rand::thread_rng()).unwrap();
                Return::Weed(Agent::eat(self, &eat_weed, world))

            } else if !feel_agent.is_empty() && self.energy > 200.0 && !feel_path.is_empty() {

                let birth_place: &World = feel_path.choose(&mut rand::thread_rng()).unwrap();
                let best_agent: &World = feel_agent.iter().max_by_key(|e| e.energy as i32).unwrap();
                // println!("a1-{:?}, a2-{:?}", self.rect, best_agent.rect);
                let bp = Point2::new(birth_place.rect.x, birth_place.rect.y);
                let ba = Point2::new(best_agent.rect.x, best_agent.rect.y);
                let a = Point2::new(self.rect.x, self.rect.y);
                Return::NewBot((a, ba, bp))

            } else if feel_path.len() >= 3 {

                let (mut see_agent, mut see_weed) = Agent::vision_bot(&self, world);
                if !see_agent.is_empty() && self.energy > 200.0 {
                    // println!("see_agent сработал");
                    Agent::move_bot(self, world, &mut see_agent);
                    Return::Move(*self)
                } else if !see_weed.is_empty() {
                    // println!("see_weed сработал");
                    Agent::move_bot(self, world, &mut see_weed);
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
        let mut cells_around: Vec<&World> = Vec::new();
        for p in constants::POINTS.iter() {
            // Вычисляем новые координаты
            let new_x = self.rect.x as isize + p.x as isize;
            let new_y = self.rect.y as isize + p.y as isize;
    
            // Проверяем, что координаты не выходят за границы массива
            if new_x >= 0 && new_x < constants::WIDTH as isize && new_y >= 0 && new_y < constants::HEIGHT as isize {
                let cell: &World = &cells[(new_y as f32 * constants::HEIGHT + new_x as f32) as usize];
                cells_around.push(cell);
            }
        }
        let feel_weed: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'g').cloned().collect();
        let feel_agent: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'c').cloned().collect();
        let feel_path: Vec<World> = cells_around.clone().into_iter().filter(|cells_around| cells_around.color == 'b').cloned().collect();
        
        (feel_weed, feel_agent, feel_path)
    }

    // fn eat(&mut self, food: &World, world: &mut Vec<World>, weeds: &mut Vec<Weed>) -> Weed {
    //     // println!("eat сработал");
    //     let old_pos: &mut World = &mut world[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
    //     old_pos.color = 'b';
    //     old_pos.energy = 0.0;
    //     let new_pos: &mut World = &mut world[(food.rect.y as f32 * constants::HEIGHT + food.rect.x as f32) as usize];
    //     new_pos.color = 'c';
    //     new_pos.energy = self.energy;
    //     // self.rect.x = food.rect.x.clone();
    //     // self.rect.y = food.rect.y.clone();
    //     self.energy += 50.0;
    //     // println!("и тут код крашится");
    //     weeds.iter().find(|p: &&Weed| p.pos == food.pos).unwrap().clone()
    //     // *weeds.iter().find(|p| p.pos == food.pos).unwrap()
    // } 

    fn eat(&mut self, food: &World, world: &mut Vec<World>) -> Point2<f32> {
        // println!("eat сработал");
        let pos_food: Point2<f32> = Point2::new(food.rect.x, food.rect.y);
        // println!("{:?}", self.rect);
        // Найти индекс старой позиции бота в векторе world
        let old_index = (self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize;
        // Найти индекс новой позиции бота в векторе world
        let new_index = (food.rect.y as f32 * constants::HEIGHT + food.rect.x as f32) as usize;
        // Обменять значения элементов world по индексам
        world.swap(old_index, new_index);
        // Изменить цвет и энергию новой позиции бота
        world[new_index].color = 'c';
        world[new_index].energy = self.energy;
        // Изменить координаты бота
        // println!("{:?}", self.rect);
        self.rect.x = pos_food.x;
        self.rect.y = pos_food.y;
        // println!("{:?}", self.rect);
        // Увеличить энергию бота
        self.energy += 50.0;
        // println!("и тут код крашится");
        // Найти и вернуть сорняк с такой же позицией, как food
        pos_food
        // weeds.iter().find(|p: &&Weed| p.pos == food.pos).unwrap().clone()
        // *weeds.iter().find(|p| p.pos == food.pos).unwrap()
    }     

    fn vision_bot(&self, world: &mut Vec<World>) -> (Vec<World>, Vec<World>) {
        // println!("vision сработал");
        let mut see_weeds: Vec<World> = Vec::new();
        let mut see_agents: Vec<World> = Vec::new();
        let left: f32 = (self.rect.x - self.vision_area).max(0.0); // метод выбора максимального значения
        let right: f32 = (self.rect.x + self.vision_area).min(constants::WIDTH); // метод выбора минимального значения
        let bottom: f32 = (self.rect.y + self.vision_area).min(constants::HEIGHT);
        let top: f32 = (self.rect.y - self.vision_area).max(0.0);
        // println!("{}, {}", left, right);
        for x in left as u32..=right as u32 {
            for y in top as u32..=bottom as u32 {
                if let Some(cell) = world.get((y as f32 * constants::HEIGHT + x as f32) as usize) {
                    // println!("клетка найдена: {:?}", cell);
                    if cell.color == 'g' {
                        // println!("видит траву");
                        see_weeds.push(cell.clone());
                    } else if cell.color == 'c' {
                        // println!("видит агентов"); 
                        see_agents.push(cell.clone());
                    }
                }
            }
        }
        // println!("{}", &see_weeds.len());
        (see_agents, see_weeds)
    }

    // pub fn reproduction(agent1: &mut Agent, agent2: &Agent, birth_place: &World) -> (f32, f32) {
    // fn reproduction(&mut self, agent: &mut Agent, world: &mut Vec<World>, birth_place: &World) -> Agent {
        // let a = agents.iter().find(|p| p.pos == agent.pos);
    //     agent2.energy -= 20.0;
    //     agent1.energy -= 80.0;
    //     (birth_place.rect.x, birth_place.rect.y)
    // }

    // pub fn move_bot(&mut self, world: &mut Vec<World>, target: &mut Vec<World>) {
    //     let old_pos: &mut World = &mut world[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
    //     old_pos.color = 'b';
    //     old_pos.energy = 0.0;
    //     let mut index = 0;
    //     let mut min_distance = f32::MAX;
    //     for (i, w) in target.iter().enumerate() {
    //         let dx: f32 = (self.rect.x - w.rect.x).abs();
    //         let dy: f32 = (self.rect.y - w.rect.y).abs();
    //         let distance: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
    //         if distance < min_distance {
    //             index = i;
    //             min_distance = distance;
    //         }
    //     }
    //     // Вычисляем вектор направления от bot к world[index]
    //     let mut direction_x: f32 = target[index].rect.x - self.rect.x;
    //     let mut direction_y: f32 = target[index].rect.y - self.rect.y;
    //     // Вычисляем длину вектора направления
    //     let length: f32 = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
    //     // Нормализуем вектор направления, деля его на длину
    //     direction_x /= length;
    //     direction_y /= length;
    //     // Прибавляем вектор направления к координатам bot
    //     self.rect.x = (self.rect.x + direction_x).round();
    //     self.rect.y = (self.rect.y + direction_y).round();
    //     let new_pos: &mut World = &mut target[(self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize];
    //     new_pos.color = 'c';
    //     new_pos.energy = self.energy;
    //     println!("a1-{:?}, a2-{:?}", old_pos.rect, new_pos.rect);
    //     println!("a1-{:?}, a2-{:?}", old_pos.color, new_pos.color);
    // }

    pub fn move_bot(&mut self, world: &mut Vec<World>, target: &mut Vec<World>) {
        // Найти индекс ближайшего элемента в target
        let mut index = 0;
        let mut min_distance = f32::MAX;
        for (i, w) in target.iter().enumerate() {
            let dx: f32 = (self.rect.x - w.rect.x).abs();
            let dy: f32 = (self.rect.y - w.rect.y).abs();
            let distance: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
            if distance < min_distance {
                index = i;
                min_distance = distance;
            }
        }
        // Вычислить вектор направления от bot к target[index]
        let mut direction_x: f32 = target[index].rect.x - self.rect.x;
        let mut direction_y: f32 = target[index].rect.y - self.rect.y;
        // Вычислить длину вектора направления
        let length: f32 = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
        // Нормализовать вектор направления, деля его на длину
        direction_x /= length;
        direction_y /= length;
        // Прибавить вектор направления к координатам bot
        self.rect.x = (self.rect.x + direction_x).round();
        self.rect.y = (self.rect.y + direction_y).round();
        // Обменять значения элементов world по индексам
        let old_index = (self.rect.y as f32 * constants::HEIGHT + self.rect.x as f32) as usize;
        let new_index = (target[index].rect.y as f32 * constants::HEIGHT + target[index].rect.x as f32) as usize;
        world.swap(old_index, new_index);
        // Вывести результат
        // println!("a1-{:?}, a2-{:?}", world[old_index].rect, world[new_index].rect);
        // println!("a1-{:?}, a2-{:?}", world[old_index].color, world[new_index].color);
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
        let k_ctx: &&KeyboardContext = &_ctx;
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

