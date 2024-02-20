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
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut world = world::World::make_cells();
        let agent = agents::Agent::make_agent(&mut world, None, None);
        // let agent = agents::Agent::make_agent();
        let cycles = 0;
        let mut agents = Vec::new();
        while 5 > agents.len() {
            agents.push(agents::Agent::make_agent(&mut world, None, None));
        }

        let mut weeds = Vec::new();
        while 100 > weeds.len() {
            weeds.push(weeds::Weed::make_weed(&mut world, None, None));
        }

        MyGame {
            cycles,
            world,
            agent,
            agents,
            weeds,
        }
    }

}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cycles += 1;
        let mut dead_bot = Vec::new();
        let mut eating_weed = Vec::new();
        let mut new_life = Vec::new();

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

        if eating_weed.is_empty() && dead_bot.is_empty() {
            for (i, a) in self.agents.iter_mut().enumerate() {
                let result = agents::Agent::do_agent(a, i.try_into().unwrap(), &mut self.weeds, &mut self.world);

                match result {
                    Return::Int(i) => dead_bot.push(i.try_into().unwrap()),
                    Return::Weed(w) => eating_weed.push(w),
                    Return::NewBot(b) => new_life.push(b),
                    Return::Move(mut m) => m.energy -= 3.0,
                    Return::Sleep(mut s) => s.energy += 0.05,
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
            let _indx = eating_weed.remove(0).pos;
            self.weeds.remove(_indx.try_into().unwrap());
            // self.weeds.remove(self.weeds.iter().position(|w| w.id == _indx.id).unwrap());
            if self.weeds.is_empty() {
                self.weeds.push(weeds::Weed::make_weed(&mut self.world, None, None));
            }
        }

        if !dead_bot.is_empty() {
            let indx: i32 = dead_bot.remove(0);
            self.agents.remove(indx as usize);
            if self.agents.is_empty() {
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
        let cycles = format!("Циклы: {}", self.cycles);
        // let font_data = include_bytes!("/DejaVuSerif.ttf"); // Загрузить данные шрифта из файла
        // let font = graphics::Font::from_file_data(ctx, font_data)?; // Создать шрифт из данных
        let text = graphics::Text::new(cycles); // Создать текст без указания размера шрифта
        // text.set_font(font, Scale::uniform(32.0)); // Установить шрифт и размер шрифта
        // text.set_color(Color::WHITE)?; // Установить цвет текста
        
        // Отрисовать текст в левом верхнем углу
        // let dest_point = mint::Point2 { x: 10.0, y: 10.0 }; // Координаты точки назначения
        
        Drawable::draw(&text, &mut canvas, graphics::DrawParam::default()); 

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
