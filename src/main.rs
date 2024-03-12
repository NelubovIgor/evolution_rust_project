use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Drawable, Mesh, Rect};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use nalgebra::geometry::Point2;
// use rand::Rng;

mod constants;
mod agents;
use agents::{Agent, Return};
mod weeds;
use weeds::Weed;
mod world;
use world::World;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Evolution", "Igor")
        .window_mode(ggez::conf::WindowMode::default().dimensions(constants::MAIN_WIDTH, constants::HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    paused: bool,
    cycles: u32,
    agent: Agent,
    agents: Vec<Agent>,
    weeds: Vec<Weed>,
    world: Vec<World>,
    dead_bot: Vec<i32>,
    eating_weed: Vec<Point2<f32>>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let paused = true;

        let mut world: Vec<World> = Vec::new();
        for x in 0..constants::WIDTH as u32 {
            for y in 0..constants::HEIGHT as u32 {
                let cell: World = world::World::make_cells(Some(x as f32), Some(y as f32));
                world.push(cell);
            }
        }
        let cycles: u32 = 0;
        let agent: Agent = agents::Agent::make_agent(&mut world, None, None, cycles);
        let eating_weed: Vec<Point2<f32>> = Vec::new();
        let dead_bot: Vec<i32> = Vec::new();

        let mut agents: Vec<Agent> = Vec::new();
        let mut weeds: Vec<Weed> = Vec::new();



        // weeds.push(weeds::Weed::make_weed(&mut world, Some(70.0), Some(150.0)));
        weeds.push(weeds::Weed::make_weed(&mut world, Some(90.0), Some(150.0)));
        // agents.push(agents::Agent::make_agent(&mut world, Some(150.0), Some(150.0), cycles));
        agents.push(agents::Agent::make_agent(&mut world, Some(250.0), Some(150.0), cycles));


        // while 1 > agents.len() {
        //     agents.push(agents::Agent::make_agent(&mut world, None, None));
        // }


        // while 100 > weeds.len() {
        //     weeds.push(weeds::Weed::make_weed(&mut world, None, None));
        // }

        MyGame {
            paused,
            cycles,
            world,
            agent,
            agents,
            weeds,
            dead_bot,
            eating_weed,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {

        if self.paused {
            return  Ok(());
        }
        self.cycles += 1;
        // let mut dead_bot: Vec<i32> = Vec::new();
        // let mut eating_weed: Vec<Weed> = Vec::new();
        let mut new_life: Vec<(Point2<f32>, Point2<f32>, Point2<f32>)> = Vec::new();

        if self.cycles % 10 == 0 {
            self.weeds.push(weeds::Weed::make_weed(&mut self.world, None, None));
        }

        // let mut grow_plat = Vec::new();

        // if self.cycle_count % 200 == 0 {
        //     for w in self.weeds.iter() {
        //         let plant: Option<Weed> = weeds::Weed::grow_weed(w, &mut self.world);
        //         if let Some(weed) = plant {
        //             grow_plat.push(weed);
        //         }
        //     }
        // }

        // if !grow_plat.is_empty() {
        //     self.weeds.append(&mut grow_plat);
        // }

        // if self.eating_weed.is_empty() && self.dead_bot.is_empty() {
            for (i, a) in self.agents.iter_mut().enumerate() {
                let result: Return = agents::Agent::do_agent(a, i.try_into().unwrap(), &mut self.world, self.cycles);

                match result {
                    Return::Int(i) => self.dead_bot.push(i.try_into().unwrap()),
                    Return::Weed(w) => self.eating_weed.push(w),
                    Return::NewBot(b) => new_life.push(b),
                    Return::Move(mut m) => m.energy -= 1.0,
                    Return::Sleep(mut s) => s.energy += 0.08,
                }
            }
        // }

        if !new_life.is_empty() {
            for n in new_life {
                let (a, ba, pa) = n;
                let agent1: Option<&mut Agent> = self.agents.iter_mut().find(|a1: &&mut Agent| a1.rect.x == a.x && a1.rect.y == a.y);
                if let Some(agent1) = agent1 {
                    agent1.energy -= 80.0;
                    let cell: &mut World = &mut self.world[(agent1.rect.x * constants::HEIGHT + agent1.rect.y) as usize];
                    cell.energy -= 80.0;
                }
                let agent2: Option<&mut Agent> = self.agents.iter_mut().find(|a1: &&mut Agent| a1.rect.x == ba.x && a1.rect.y == ba.y);
                if let Some(agent2) = agent2 {
                    agent2.energy -= 20.0;
                    let cell: &mut World = &mut self.world[(agent2.rect.x * constants::HEIGHT + agent2.rect.y) as usize];
                    cell.energy -= 20.0;
                }
                // println!("a1-{:?} a2-{:?}", a, ba);
                // agents::Agent::reproduction(agent1, agent2, birth_place)
                self.agents.push(agents::Agent::make_agent(&mut self.world, Some(pa.x), Some(pa.y), self.cycles));
            }
        }

        if !self.eating_weed.is_empty() {
            let weed_dead = self.eating_weed.remove(0);
            // println!("{:?}", weed_dead);
            let indx = self.weeds.iter().position(|w| w.rect.x == weed_dead.x && w.rect.y == weed_dead.y);
            
            // println!("{:?}", indx);
            if let Some(indx)  = indx  {
                self.weeds.remove(indx);
                let _indx: usize = self.world.iter().position(|w| w.rect.x == weed_dead.x && w.rect.y == weed_dead.y).unwrap();
                let cell: &mut World = &mut self.world[_indx as usize];
                cell.type_cell = 'b';
                cell.energy = 0.0;
            }
        }

        if !self.dead_bot.is_empty() {
            let indx: i32 = self.dead_bot.remove(0);
            self.agents.remove(indx as usize);
            // if self.agents.is_empty() {
            //     self.agents.push(agents::Agent::make_agent(&mut self.world, None, None));
            //     self.agents.push(agents::Agent::make_agent(&mut self.world, None, None));
            // }
        }


        agents::Agent::move_agent(&mut self.agent.rect, &_ctx.keyboard);
        let indexes_to_remove = agents::Agent::check_collision(&self.agent.rect, &self.weeds);
        // for b in self.agents.iter() {            
        //     let check_remove = agents::Agent::check_collision(&b.rect, &self.weeds);
        //     if !check_remove.is_empty() {
        //         indexes_to_remove.extend(check_remove)
        //     }
        // }
        if !indexes_to_remove.is_empty() {
            for index in indexes_to_remove.iter().rev() {
                self.weeds.remove(*index);
                // self.weeds.push(weeds::Weed::make_weed(&mut self.world, None, None));
            }
        }
        // self.paused = true;
        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // let mut menu_canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(128, 128, 128));

        // Создать объект Text с количеством циклов игры
        let cycles: String = format!("Циклы: {}", self.cycles);
        let text: graphics::Text = graphics::Text::new(cycles); 
        Drawable::draw(&text, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 0.0, 1.0, 1.0)));

        let bot_dies: String = format!("боты ожидающие смерти: {}", self.dead_bot.len());
        let text_dead: graphics::Text = graphics::Text::new(bot_dies);
        Drawable::draw(&text_dead, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 15.0, 1.0, 1.0))); 

        let bot_life: String = format!("живые боты: {}", self.agents.len());
        let text_life: graphics::Text = graphics::Text::new(bot_life);
        Drawable::draw(&text_life, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 30.0, 1.0, 1.0))); 

        let eat: String = format!("трава на съедение: {}", self.eating_weed.len());
        let text_eat: graphics::Text = graphics::Text::new(eat);
        Drawable::draw(&text_eat, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 45.0, 1.0, 1.0))); 

        let weeds: String = format!("травы на петри: {}", self.weeds.len());
        let text_weeds: graphics::Text = graphics::Text::new(weeds);
        Drawable::draw(&text_weeds, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 60.0, 1.0, 1.0))); 

        let bot1: String = format!("бот координаты: {}:{}.\nenergy: {}", self.agents[0].rect.x, self.agents[0].rect.y, self.agents[0].energy);
        let text_bot1: graphics::Text = graphics::Text::new(bot1);
        Drawable::draw(&text_bot1, &mut canvas, graphics::DrawParam::default().dest_rect(Rect::new(500.0, 75.0, 1.0, 1.0))); 


        for a in &self.agents {
            let agents: Mesh = Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(), a.rect, Color::RED,
            ).unwrap();
            Drawable::draw(&agents, &mut canvas, graphics::DrawParam::default());
        }

        for w in &self.weeds {
            let weed: Mesh = Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(), w.rect, Color::GREEN,
            ).unwrap();
            Drawable::draw(&weed, &mut canvas, graphics::DrawParam::default());
        }

        agents::Agent::draw_agent(ctx, self.agent.rect, &mut canvas);

        canvas.finish(ctx)
        // menu_canvas.finish(ctx)
    }

    
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) => {self.paused = !self.paused;}
            Some(KeyCode::D) => {}
            Some(KeyCode::A) => {}
            Some(KeyCode::S) => {}
            Some(KeyCode::W) => {}
            _ => (),
        }
        Ok(())
    }
}
