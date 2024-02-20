use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Drawable};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
// use ggez::mint;
// use nalgebra::Scale;
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
        .window_mode(ggez::conf::WindowMode::default().dimensions(constants::WIDTH, constants::HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    cycles: u32,
    agent: Agent,
    agents: Vec<Agent>,
    weeds: Vec<Weed>,
    world: Vec<World>,
    dead_bot: Vec<i32>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut world: Vec<World> = world::World::make_cells();
        let agent: Agent = agents::Agent::make_agent(&mut world, None, None);
        // let agent = agents::Agent::make_agent(); 
        let mut dead_bot: Vec<i32> = Vec::new();
        let cycles: u32 = 0;
        let mut agents: Vec<Agent> = Vec::new();
        while 6 > agents.len() {
            agents.push(agents::Agent::make_agent(&mut world, None, None));
        }

        let mut weeds: Vec<Weed> = Vec::new();
        while 500 > weeds.len() {
            weeds.push(weeds::Weed::make_weed(&mut world, None, None));
        }

        MyGame {
            cycles,
            world,
            agent,
            agents,
            weeds,
            dead_bot,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cycles += 1;
        // let mut dead_bot: Vec<i32> = Vec::new();
        let mut eating_weed: Vec<Weed> = Vec::new();
        let mut new_life: Vec<(f32, f32)> = Vec::new();

        if self.cycles % 30 == 0 {
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

        if eating_weed.is_empty() && self.dead_bot.is_empty() {
            for (i, a) in self.agents.iter_mut().enumerate() {
                let result = agents::Agent::do_agent(a, i.try_into().unwrap(), &mut self.weeds, &mut self.world);

                match result {
                    Return::Int(i) => self.dead_bot.push(i.try_into().unwrap()),
                    Return::Weed(w) => eating_weed.push(w),
                    Return::NewBot(b) => new_life.push(b),
                    Return::Move(mut m) => m.energy -= 1.0,
                    Return::Sleep(mut s) => s.energy += 0.009,
                }
            }
        }

        if !new_life.is_empty() {
            for b in new_life {
                let (x, y) = b;
                self.agents.push(agents::Agent::make_agent(&mut self.world, Some(x), Some(y)));
            }
        }

        if !eating_weed.is_empty() {
            let weed_dead: Weed = eating_weed.remove(0);
            let indx = self.weeds.iter().position(|p| p.pos == weed_dead.pos).unwrap();
            self.weeds.remove(indx);
            if self.weeds.is_empty() {
                self.weeds.push(weeds::Weed::make_weed(&mut self.world, None, None));
            }
        }

        if !self.dead_bot.is_empty() {
            let indx: i32 = self.dead_bot.remove(0);
            self.agents.remove(indx as usize);
            if self.agents.is_empty() {
                self.agents.push(agents::Agent::make_agent(&mut self.world, None, None));
                self.agents.push(agents::Agent::make_agent(&mut self.world, None, None));
            }
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
        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Создать объект Text с количеством циклов игры
        let cycles: String = format!("Циклы: {}", self.cycles);
        let text: graphics::Text = graphics::Text::new(cycles); 
        Drawable::draw(&text, &mut canvas, graphics::DrawParam::default());

        let bot_dies: String = format!("боты ожидающие смерти: {}", self.dead_bot.len());
        let text_dead: graphics::Text = graphics::Text::new(bot_dies);
        Drawable::draw(&text_dead, &mut canvas, graphics::DrawParam::default()); 

        let bot_life: String = format!("живые боты: {}", self.agents.len());
        let text_life: graphics::Text = graphics::Text::new(bot_life);
        Drawable::draw(&text_life, &mut canvas, graphics::DrawParam::default()); 

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
    }

    
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::D) => {}
            Some(KeyCode::A) => {}
            Some(KeyCode::S) => {}
            Some(KeyCode::W) => {}
            _ => (),
        }
        Ok(())
    }
}
