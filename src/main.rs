use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Drawable};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
// use rand::Rng;

mod agents;
use agents::Agent;
mod weeds;
use weeds::Weed;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(500.0, 500.0))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    agent: Agent,
    weeds: Vec<Weed>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let agent = agents::Agent::make_agent();

        let mut weeds = Vec::new();
        while 20 > weeds.len() {
            weeds.push(weeds::Weed::make_weed());
        }

        MyGame {
            agent,
            weeds,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {

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
