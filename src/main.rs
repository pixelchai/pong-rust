use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const FONT_SIZE: f32 = 32.0;
const PADDING: f32 = FONT_SIZE;

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
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "res/paddle.png")?;
        let paddle_texture_width = paddle_texture.width(); // see notes.txt -- todo research error msg
        Ok(GameState {
            player_paddle: Paddle {
                paddle_texture: paddle_texture,
                position: Vec2::new((SCREEN_WIDTH as f32) - PADDING - (paddle_texture_width as f32), PADDING),
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
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}