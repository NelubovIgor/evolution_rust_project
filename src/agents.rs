use ggez::graphics::{self, Rect, Drawable, Mesh, Color, Canvas};
use ggez::input::keyboard::{KeyCode, KeyboardContext};
use rand::Rng;
use ggez::{Context};

#[derive(Debug)]
pub struct Agent {
    pub rect: Rect,
}

impl Agent {
    pub fn make_agent() -> Agent {
        let x = rand::thread_rng().gen_range(0..500) as f32;
        let y = rand::thread_rng().gen_range(0..500) as f32;
        Agent {
            rect: Rect::new(x, y, 3.0, 3.0),
        }
    }

    pub fn draw_agent(ctx: &Context, a: Rect, canvas: &mut Canvas) {
        let agent = Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(), a, Color::BLUE,
        ).unwrap();
        Drawable::draw(&agent, canvas, graphics::DrawParam::default())
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

    // fn key_down_event_agent(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
    //     match input.keycode {
    //         Some(KeyCode::D) => {}
    //         Some(KeyCode::A) => {}
    //         Some(KeyCode::S) => {}
    //         Some(KeyCode::W) => {}
    //         _ => (),
    //     }
    //     Ok(())
    // }

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
    //         // self.agent_score += 1;
    //     }
    // }
}
