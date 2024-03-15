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
    pub type_cell: char,
    birth_day: u32,
    pub pos: Point2<f32>,
    pub rect: Rect,
    pub energy: f32,
    vision_area: f32,
    // gen: Vec<i8, i8, i8, i8, i8>,
    // color: i8,
    // steps: i8,
    // offspring: i8,
    // offspring_energy: i8,
    // sleep: i8,
}

pub enum Return {
    Int(usize),
    NewBot((Point2<f32>, Point2<f32>, Point2<f32>)),
    Weed(Point2<f32>),
    Move(Agent),
    Sleep(Agent),
}

impl Agent {
    pub fn make_agent(world: &mut Vec<World>, x: Option<f32>, y: Option<f32>, cycles: u32) -> Agent {
        let x: f32 = match x {
            Some(x) => x,
            None => rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32,
        };
        let y: f32 = match y {
            Some(y) => y,
            None => rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32,
        };

        let _pos: Point2<f32> = Point2::new(x, y);
        let _energy: f32 = 100.0;

        let cell: &mut World = world.iter_mut().find(|w| w.pos == _pos).unwrap();
        // cell.celltype = Agent;
        cell.type_cell = 'c';        
        cell.energy = _energy;

        let agent: Agent = Agent {
            birth_day: cycles,
            pos: _pos,
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: _energy,
            vision_area: 20.0,
            type_cell: 'c',
        };
        agent
    }

    pub fn draw_agent(ctx: &Context, a: Rect, canvas: &mut Canvas) {
        let agent: Mesh = Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(), a, Color::BLUE,
        ).unwrap();
        Drawable::draw(&agent, canvas, graphics::DrawParam::default())
    }


    
    pub fn do_agent(&mut self, i: i32, world: &mut Vec<World>, cycles: u32) -> Return {
        // println!("{:?}", self.rect.x);
        self.energy -= 0.1;
        if self.energy > 0.0 && (cycles - self.birth_day) > 10 {
            // узнаёт, что чувствует агент
            let (feel_weed, feel_agent, feel_path) = Agent::touch(self, world);            

            if !feel_weed.is_empty() { // ! - инвертирует запрос is_empty(), который возвращает true если список пуст
                // println!("{:?}", feel_weed);
                // println!("feel_weed сработал");
                let eat_weed: &World = feel_weed.choose(&mut rand::thread_rng()).unwrap();
                Return::Weed(Agent::eat(self, eat_weed))

            } else if !feel_agent.is_empty() && self.energy > 200.0 && !feel_path.is_empty() {

                let birth_place: &World = feel_path.choose(&mut rand::thread_rng()).unwrap();
                let best_agent: &World = feel_agent.iter().max_by_key(|e| e.energy as i32).unwrap();
                // println!("a1-{:?}, a2-{:?}", self.rect, best_agent.rect);
                let bp: nalgebra::OPoint<f32, nalgebra::Const<2>> = Point2::new(birth_place.rect.x, birth_place.rect.y);
                let ba: nalgebra::OPoint<f32, nalgebra::Const<2>> = Point2::new(best_agent.rect.x, best_agent.rect.y);
                let a: nalgebra::OPoint<f32, nalgebra::Const<2>> = Point2::new(self.rect.x, self.rect.y);
                Return::NewBot((a, ba, bp))

            } else if feel_path.len() >= 3 {
                let mut see_agent_result = None;
                let mut see_weed_result = None;
                
                let (see_agent, see_weed) = Agent::vision_bot(self, world);
                if !see_agent.is_empty() && self.energy > 200.0 {
                    see_agent_result = Some(see_agent);
                } else if !see_weed.is_empty() {
                    see_weed_result = Some(see_weed);
                }
            

                // println!("свободный путь");
                if let Some(see_agent) = see_agent_result {
                    Agent::move_bot(self, world, see_agent);
                    Return::Move(*self)
                } else if let Some(see_weed) = see_weed_result {
                    Agent::move_bot(self, world, see_weed);
                    Return::Move(*self)
                } else {
                    //sleep
                    Return::Sleep(*self)
                }

            } else {
                //sleep
                Return::Sleep(*self)
            }

        } else if self.energy > 0.0 {
            Return::Sleep(*self)

        } else {
            Return::Int(i.try_into().unwrap())
        }
    }

    fn touch(&mut self, cells: &mut Vec<World>) -> (Vec<World>, Vec<World>, Vec<World>) {
        // println!("touch сработал");
        let mut cells_around: Vec<World> = Vec::new();
        for p in constants::POINTS.iter() {
            // Вычисляем новые координаты
            let new_x = self.rect.x + p.x as f32;
            let new_y = self.rect.y + p.y as f32;
    
            // Проверяем, что координаты не выходят за границы массива
            if new_x >= 0.0 && new_x < constants::WIDTH && new_y >= 0.0 && new_y < constants::HEIGHT {
                let cell: &World = cells.iter().find(|w| w.rect.x == new_x && w.rect.y == new_y).unwrap();
                cells_around.push(cell.clone());
            }
        }

        let mut feel_weed: Vec<World> = Vec::new();
        let mut feel_agent: Vec<World> = Vec::new();
        let mut feel_path: Vec<World> = Vec::new();

        // println!("\n{:?}", self);
        // println!("cells_around");
        // println!("{:?}", cells_around);
        for c in cells_around {
            if c.type_cell == 'g' {
                feel_weed.push(c);
            } else if c.type_cell == 'c' {
                feel_agent.push(c);
            } else if c.type_cell == 'b' {
                feel_path.push(c);
            }
        }
        (feel_weed, feel_agent, feel_path)
    }

    fn eat(&mut self, food: &World) -> Point2<f32> {
        // println!("eat сработал");
        // println!("{:?}", food);
        let pos_food: Point2<f32> = Point2::new(food.rect.x, food.rect.y);
        // println!("{:?}", self.rect);
        // Увеличить энергию бота
        self.energy += 50.0;
        pos_food
    }

    fn vision_bot<'a>(&self, world: &'a [World]) -> (Vec<&'a World>, Vec<&'a World>) {
        // println!("vision сработал");
        let mut see_weeds: Vec<&'a World> = Vec::new();
        let mut see_agents: Vec<&'a World> = Vec::new();
        let left = (self.rect.x - self.vision_area).max(0.0) as i32; // метод выбора максимального значения
        let right = (self.rect.x + self.vision_area).min(constants::WIDTH) as i32; // метод выбора минимального значения
        let bottom = (self.rect.y + self.vision_area).min(constants::HEIGHT) as i32;
        let top = (self.rect.y - self.vision_area).max(0.0) as i32;
        // println!("{}, {}", left, right);
        // println!("{}, {}", top, bottom);
        // println!("{}, {}", self.rect.x, self.rect.y);
        for x in left ..=right {
            for y in top ..=bottom {
                if (self.rect.x != x as f32) && (self.rect.x != y as f32) {
                    let _indx: usize = world.iter().position(|w| w.rect.x == x as f32 && w.rect.y == y as f32).unwrap();
                    let cell: &World = &world[_indx as usize];
                    // println!("клетка найдена: {:?}", cell);
                    if cell.type_cell == 'g' {
                        // println!("видит траву");
                        see_weeds.push(cell);
                    } else if cell.type_cell == 'c' {
                        // println!("видит агентов"); 
                        see_agents.push(cell);
                    }  
                }
            }
        }
        // println!("сколько травы увидел: {}", &see_weeds.len());
        (see_agents, see_weeds)
    }

    // pub fn reproduction(agent1: &mut Agent, agent2: &Agent, birth_place: &World) -> (f32, f32) {
    // fn reproduction(&mut self, agent: &mut Agent, world: &mut Vec<World>, birth_place: &World) -> Agent {
    //     let a = agents.iter().find(|p| p.pos == agent.pos);
    //     agent2.energy -= 20.0;
    //     agent1.energy -= 80.0;
    //     (birth_place.rect.x, birth_place.rect.y)
    // }


    pub fn move_bot(&mut self, world: &mut Vec<World>, target: Vec<&World>) {
        // println!("move сработал");
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

        // Обменять значения элементов world по индексам
        let old_index = world.iter().position(|w| w.rect.x == self.rect.x && w.rect.y == self.rect.y).unwrap();
        let new_index = world.iter().position(|w| w.rect.x == target[index].rect.x && w.rect.y == target[index].rect.y).unwrap();
        world.swap(old_index, new_index);

        // Прибавить вектор направления к координатам bot
        self.rect.x = (self.rect.x + direction_x).round();
        self.rect.y = (self.rect.y + direction_y).round();
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

