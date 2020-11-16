use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const FONT_SIZE: f32 = 32.0;
const PADDING: f32 = FONT_SIZE;
const PADDLE_SPEED: f32 = 16.0;

struct Paddle {
    paddle_texture: Texture,
    position: Vec2<f32>,
}

// impl Paddle {
//     fn new(texture: Texture, position: Vec2<f32>) -> Paddle {
//         Paddle {
//             paddle_texture,
//         }
//     }
// }

struct GameState {
    player_paddle: Paddle,
    enemy_paddle: Paddle,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "res/paddle.png")?;

        // Todo: research lifetimes, cloning, ownership, etc
        Ok(GameState {
            player_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new((SCREEN_WIDTH as f32) - PADDING - (paddle_texture.width() as f32), PADDING),
            },
            enemy_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(PADDING, (SCREEN_HEIGHT as f32)/2.0 - (paddle_texture.height() as f32)/2.0),
            }
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let text = Text::new(
            "1-1",
            Font::vector(ctx, "res/vcr_osd_mono.ttf", FONT_SIZE)?,
        );
        graphics::draw(ctx, &text, Vec2::new((SCREEN_WIDTH/2) as f32, FONT_SIZE));
        graphics::draw(ctx, &self.player_paddle.paddle_texture, self.player_paddle.position);
        graphics::draw(ctx, &self.enemy_paddle.paddle_texture, self.enemy_paddle.position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::S) {
            self.player_paddle.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::W) {
            self.player_paddle.position.y -= PADDLE_SPEED;
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}