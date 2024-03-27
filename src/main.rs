use ggez::audio::SoundSource;
use ggez::conf::WindowMode;
use ggez::mint::Point2;
use ggez::{audio, event, graphics, Context, ContextBuilder, GameResult};

const SCARE_IMAGE: &'static [u8] = include_bytes!("../resources/scream.png");
const SCREAM_SOUND: &'static [u8] = include_bytes!("../resources/scream.ogg");

struct GameState {
    mouse_position: Point2<f32>,
    initial_position: Point2<f32>,
    is_moving: bool,
    walls: Vec<graphics::Rect>,
    scare_image: graphics::Image,
    scream_sound: audio::Source,
    show_scare: bool,
    end_position: Point2<f32>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let initial_position = Point2 { x: 10.0, y: 60.0 };
        let scare_image = graphics::Image::from_bytes(ctx, SCARE_IMAGE)?;
        let scream_sound_data = SCREAM_SOUND.to_vec().into();
        let scream_sound = audio::Source::from_data(ctx, scream_sound_data)?;

        let game_state = GameState {
            mouse_position: initial_position,
            initial_position,
            is_moving: false,
            walls: vec![
                // start maze left - top - width - height
                graphics::Rect::new(0.0, 00.0, 900.0, 30.0),
                graphics::Rect::new(0.0, 80.0, 800.0, 30.0),
                // first down maze
                graphics::Rect::new(800.0, 80.0, 30.0, 230.0),
                graphics::Rect::new(880.0, 0.0, 30.0, 370.0),
                // turn left maze
                graphics::Rect::new(0.0, 280.0, 800.0, 30.0),
                graphics::Rect::new(70.0, 340.0, 825.0, 30.0),
                // second down maze
                graphics::Rect::new(0.0, 300.0, 30.0, 500.0),
                graphics::Rect::new(70.0, 350.0, 30.0, 400.0),
                // turn right maze
                graphics::Rect::new(70.0, 740.0, 830.0, 30.0),
                graphics::Rect::new(0.0, 800.0, 900.0, 30.0),
            ],
            scare_image,
            scream_sound,
            show_scare: false,
            end_position: Point2 { x: 870.0, y: 790.0 },
        };

        Ok(game_state)
    }

    fn play_scream_sound(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.scream_sound.play(ctx)?;
        Ok(())
    }

    fn check_player_reached_end(&self) -> bool {
        let proximity = 20.0;
        (self.mouse_position.x - self.end_position.x).abs() < proximity
            && (self.mouse_position.y - self.end_position.y).abs() < proximity
    }

    fn check_collision(&self) -> bool {
        let quad_size = 20.0;
        let quad_rect = graphics::Rect::new(
            self.mouse_position.x - quad_size / 2.0,
            self.mouse_position.y - quad_size / 2.0,
            quad_size,
            quad_size,
        );

        for wall in &self.walls {
            if quad_rect.overlaps(wall) {
                return true;
            }
        }

        false
    }

    fn restart(&mut self) {
        self.mouse_position = self.initial_position;
        self.is_moving = false;
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.check_player_reached_end() {
            self.play_scream_sound(ctx)?;
            self.show_scare = true;
        }

        if self.check_collision() {
            self.restart();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        for wall in &self.walls {
            let wall_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                *wall,
                graphics::Color::WHITE,
            )?;

            let draw_params = graphics::DrawParam::default().offset([0.5, 0.5]);

            graphics::draw(ctx, &wall_mesh, draw_params)?;
        }

        if !self.show_scare {
            let mouse_quad = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    self.mouse_position.x - 10.0,
                    self.mouse_position.y - 10.0,
                    20.0,
                    20.0,
                ),
                graphics::Color::WHITE,
            )?;

            graphics::draw(ctx, &mouse_quad, (Point2 { x: 0.0, y: 0.0 },))?;
        }

        if self.show_scare {
            let (win_w, win_h) = graphics::drawable_size(ctx);
            let scale = graphics::DrawParam::default().scale([
                win_w / self.scare_image.width() as f32,
                win_h / self.scare_image.height() as f32,
            ]);
            graphics::draw(ctx, &self.scare_image, scale)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if !self.is_moving {
            let distance = ((self.initial_position.x - x).powi(2)
                + (self.initial_position.y - y).powi(2))
            .sqrt();
            if distance < 10.0 {
                self.is_moving = true;
            }
        }

        if self.is_moving {
            self.mouse_position = Point2 { x, y };
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("terror_maze", "Carol")
        .window_mode(WindowMode::default().dimensions(900.0, 900.0))
        .build()
        .expect("Failed to build context");

    let game_state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game_state)
}
