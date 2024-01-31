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
// use world::World;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Evolution", "Igor")
        .window_mode(ggez::conf::WindowMode::default().dimensions(constants::WIDTH, constants::HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    agent: Agent,
    agents: Vec<Agent>,
    weeds: Vec<Weed>,
    // world: Vec<world::Objects>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // let world = world::World::make_cells();

        let agent = agents::Agent::make_agent();
        
        let mut agents = Vec::new();
        while 5 > agents.len() {
            agents.push(agents::Agent::make_agent());
        }

        let mut weeds = Vec::new();
        while 20 > weeds.len() {
            weeds.push(weeds::Weed::make_weed());
        }

        MyGame {
            agent,
            agents,
            weeds,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // for (i, a) in self.agents.iter()enumerate() {
        //     let dead = agents::Agent::do_agent(a);
        //     if dead {
        //         self.agents.remove(i);
        //     }
        // }

        for b in self.agents.iter_mut() {
            agents::Agent::move_bot(&mut b.rect, &self.weeds);
        }

        agents::Agent::move_agent(&mut self.agent.rect, &_ctx.keyboard);
        let indexes_to_remove = agents::Agent::check_collision(&self.agent, &mut self.weeds);
        if !indexes_to_remove.is_empty() {
            for index in indexes_to_remove.iter().rev() {
                self.weeds.remove(*index);
                self.weeds.push(weeds::Weed::make_weed());
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
