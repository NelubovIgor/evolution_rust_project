use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Drawable, Rect};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;

mod agents;
use agents::Agent;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct Weed {
    rect: Rect,
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
            let x = rand::thread_rng().gen_range(0..500) as f32;
            let y = rand::thread_rng().gen_range(0..500) as f32;
            let weed = Weed {
                rect: Rect::new(x, y, 3.0, 3.0),
            };
            weeds.push(weed);
            
        }


        MyGame {
            agent,
            weeds,
        }
    }

    // fn check_collision(&mut self) {
    //     let player_rect = self.player_rect();
    //     let mut indexes_to_remove = Vec::new();
    //     for (i, gr) in &self.grass.iter().enumerate() {
    //         let rectangle_rect = gr.dimensions();
    //         if player_rect.overlaps(&rectangle_rect) {
    //             indexes_to_remove.push(i);
    //         }
    //     }

    //     for index in indexes_to_remove.iter().rev() {
    //         self.grass.remove(*index);
    //         self.agent_score += 1;
    //     }
    // }
}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // println!("Вывод: {:?}", &mut self.agent);

        // println!("{}", self);
        agents::Agent::move_agent(&mut self.agent.rect, &_ctx.keyboard);
        // let k_ctx = &_ctx.keyboard;
        // if k_ctx.is_key_pressed(KeyCode::D) {
        //     self.agent.rect.x += 1.0;
        // }
        // else if k_ctx.is_key_pressed(KeyCode::A) {
        //     self.agent.rect.x -= 1.0;
        // }
        // else if k_ctx.is_key_pressed(KeyCode::S) {
        //     self.agent.rect.y += 1.0;
        // }
        // else if k_ctx.is_key_pressed(KeyCode::W) {
        //     self.agent.rect.y -= 1.0;
        // }
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
