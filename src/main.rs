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
const BALL_SPEED: f32 = PADDLE_SPEED/2.0;

struct Paddle {
    paddle_texture: Texture,
    position: Vec2<f32>,    
}

struct Ball {
    ball_texture: Texture,
    position: Vec2<f32>,    
    velocity: Vec2<f32>,
}

struct GameState {
    player_paddle: Paddle,
    enemy_paddle: Paddle,
    ball: Ball,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "res/paddle.png")?;
        let ball_texture = Texture::new(ctx, "res/ball.png")?;

        let ball_texture_width = ball_texture.width();
        let ball_texture_height = ball_texture.height();

        // Todo: research lifetimes, cloning, ownership, etc
        Ok(GameState {
            player_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new((SCREEN_WIDTH as f32) - PADDING - (paddle_texture.width() as f32), PADDING),
            },
            enemy_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(PADDING, (SCREEN_HEIGHT as f32)/2.0 - (paddle_texture.height() as f32)/2.0),
            },
            ball: Ball {
                ball_texture,
                position: Vec2::new((SCREEN_WIDTH as f32)/2.0 - (ball_texture_width as f32)/2.0, (SCREEN_HEIGHT as f32)/2.0 - (ball_texture_height as f32)/2.0),
                velocity: Vec2::new(BALL_SPEED, -BALL_SPEED),
            }
        })
    }

    fn draw_paddle(ctx: &mut Context, paddle: &Paddle){
        graphics::draw(ctx, &paddle.paddle_texture, paddle.position)
    }

    fn handle_inputs(&mut self, ctx: &mut Context){
        if input::is_key_down(ctx, Key::S) {
            self.player_paddle.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::W) {
            self.player_paddle.position.y -= PADDLE_SPEED;
        }
    }

    fn update_ball(&mut self, ctx: &mut Context){
        self.ball.position += self.ball.velocity;

        if self.ball.position[1] >= (SCREEN_HEIGHT as f32) - (self.ball.ball_texture.height() as f32) || self.ball.position[1] <= 0.0 {
            self.ball.velocity[1] = -self.ball.velocity[1];
        }

        if self.ball.position[0] >= (SCREEN_WIDTH as f32) - (self.ball.ball_texture.width() as f32) || self.ball.position[0] <= 0.0 {
            self.ball.velocity[0] = -self.ball.velocity[0];
        }
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

        GameState::draw_paddle(ctx, &self.enemy_paddle);
        GameState::draw_paddle(ctx, &self.player_paddle);

        graphics::draw(ctx, &self.ball.ball_texture, self.ball.position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_inputs(ctx);
        self.update_ball(ctx);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}