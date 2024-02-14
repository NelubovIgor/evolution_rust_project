use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Drawable};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
// use rand::Rng;

mod constants;
mod agents;
use agents::Agent;
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
    cycle_count: u32,
    agent: Agent,
    agents: Vec<Agent>,
    weeds: Vec<Weed>,
    world: Vec<World>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut world = world::World::make_cells();
        let agent = agents::Agent::make_agent(&mut world);
        // let agent = agents::Agent::make_agent();
        let cycle_count = 0;
        let mut agents = Vec::new();
        while 3 > agents.len() {
            agents.push(agents::Agent::make_agent(&mut world));
        }

        let mut weeds = Vec::new();
        while 10 > weeds.len() {
            weeds.push(weeds::Weed::make_weed(&mut world, None, None));
        }

        MyGame {
            cycle_count,
            world,
            agent,
            agents,
            weeds,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cycle_count += 1;
        let mut dead_bot = Vec::new();

        if self.cycle_count % 30 == 0 {
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

        for (i, a) in self.agents.iter_mut().enumerate() {
            agents::Agent::do_agent(a, i.try_into().unwrap(), &self.weeds, &mut dead_bot, &mut self.world);
        }

        if !dead_bot.is_empty() {
            let indx = dead_bot.remove(0);
            self.agents.remove(indx as usize);
            if self.agents.is_empty() {
                self.agents.push(agents::Agent::make_agent(&mut self.world));
            }
        }


        agents::Agent::move_agent(&mut self.agent.rect, &_ctx.keyboard);
        let mut indexes_to_remove = agents::Agent::check_collision(&self.agent.rect, &self.weeds);
        for b in self.agents.iter() {            
            let check_remove = agents::Agent::check_collision(&b.rect, &self.weeds);
            if !check_remove.is_empty() {
                indexes_to_remove.extend(check_remove)
            }
        }
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

        for a in &self.agents {
            let agents = Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(), a.rect, Color::RED,
            ).unwrap();
            Drawable::draw(&agents, &mut canvas, graphics::DrawParam::default());
        }

        for w in &self.weeds {
            let weed = Mesh::new_rectangle(
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
