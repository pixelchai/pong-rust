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

impl Ball {
    fn reset(&mut self){
        self.position = Vec2::new(
            (SCREEN_WIDTH as f32)/2.0 - (self.ball_texture.width() as f32)/2.0,
            (SCREEN_HEIGHT as f32)/2.0 - (self.ball_texture.height() as f32)/2.0
        );
    }
}

struct GameState {
    ball: Ball,

    player_paddle: Paddle,
    player_score: i32,

    enemy_paddle: Paddle,
    enemy_score: i32,

    simulated: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "res/paddle.png")?;
        let ball_texture = Texture::new(ctx, "res/ball.png")?;
        
        // init ball
        let mut ball = Ball {
            ball_texture,
            position: Vec2::new(0.0, 0.0),
            velocity:  Vec2::new(BALL_SPEED, BALL_SPEED),
        };
        ball.reset();  // initialise ball's position

        Ok(GameState {
            ball,
            player_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(
                    (SCREEN_WIDTH as f32) - PADDING - (paddle_texture.width() as f32), 
                    (SCREEN_HEIGHT as f32)/2.0 - (paddle_texture.height() as f32)/2.0 - 25.0
                ),
            },
            player_score: 0,
            enemy_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(
                    PADDING, 
                    (SCREEN_HEIGHT as f32)/2.0 - (paddle_texture.height() as f32)/2.0 + 30.0
                ),
            },
            enemy_score: 0,
            simulated: false,
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

    /// Check for ball-paddle collision with the given paddle
    fn check_intersects(ball: &Ball, paddle: &Paddle) -> bool{
        // // bouncing off paddle horizontally
        // if ball.position[1] + (ball.ball_texture.height() as f32)  >= paddle.position[1] && ball.position[1] <= paddle.position[1] + (paddle.paddle_texture.height() as f32){
        //     if ball.position[0] + (ball.ball_texture.width() as f32) >= paddle.position[0] && (ball.position[0] <= paddle.position[0] + (paddle.paddle_texture.width() as f32)){
        //         ball.velocity[0] = -ball.velocity[0];
        //         println!("Bounced off paddle horizontally!");
        //     }
        // }

        // // bouncing off paddle vertically
        // if (ball.position[0] + (ball.ball_texture.width() as f32) >= paddle.position[0]) && (ball.position[0] <= paddle.position[0] + (paddle.paddle_texture.width() as f32)){
        //     if (ball.position[1] <= paddle.position[1] + (paddle.paddle_texture.height() as f32)) && (ball.position[1] + (ball.ball_texture.height() as f32) >= paddle.position[1]) {
        //         // ball.velocity[1] = -ball.velocity[1];
        //         ball.velocity[0] = -ball.velocity[0];
        //         println!("Bounced off paddle vertically!");


        //     }
        // }
        
        // check if ball's centre point is inside paddle rectangle:
        // method adapted from: https://stackoverflow.com/a/2763387/5013267
        // let ab = Vec2::new(paddle.paddle_texture.width() as f32, 0.0); // vector a->b
        let ab = (paddle.position + (paddle.paddle_texture.width() as f32, 0.0)) - paddle.position;
        let bc = Vec2::new(0.0, paddle.paddle_texture.height() as f32); // vector b->c

        let m = (ball.position + Vec2::new(ball.ball_texture.width() as f32, ball.ball_texture.height() as f32))/2.0;
        // println!("{}", m);

        // let mut intersects = true;
        let ab_dot_am = ab.dot(m - paddle.position);
        // println!("{}", ab_dot_am);
        // println!("{}", m - paddle.position);
        println!("{} - {} = {} ", m, paddle.position, m - paddle.position);
        // intersects &= ab_dot_am >= 0.0 && ab_dot_am <= ab.dot(ab);

        let bc_dot_bm = bc.dot(m - (paddle.position + (paddle.paddle_texture.width() as f32, 0.0)));
        // intersects &= bc_dot_bm >= 0.0 && bc_dot_bm <= bc.dot(bc);
        // println!("{}, {}", ab_dot_am >= 0.0 && ab_dot_am <= ab.dot(ab), bc_dot_bm >= 0.0 && bc_dot_bm <= bc.dot(bc));

        0.0 <= ab_dot_am && ab_dot_am <= ab.dot(ab)
        && 0.0 <= bc_dot_bm && bc_dot_bm <= bc.dot(bc)  // return value
    }

    fn update_ball(&mut self, ctx: &mut Context){
        self.ball.position += self.ball.velocity;

        // GameState::update_collision(&mut self.ball, &self.enemy_paddle);
        // GameState::update_collision(&mut self.ball, &self.player_paddle);
        // if GameState::check_intersects(&self.ball, &self.enemy_paddle) {
        //     println!("WOOOO enemy paddle!!");
        // }

        if GameState::check_intersects(&self.ball, &self.player_paddle) {
            println!("WOOOO player paddle!!");
        }

        // walls
        // bouncing off top and bottom walls
        if (self.ball.position[1] + (self.ball.ball_texture.height() as f32) >= (SCREEN_HEIGHT as f32)) || self.ball.position[1] <= 0.0 {
            self.ball.velocity[1] = -self.ball.velocity[1];
        }

        // if bounce off either of the side walls...
        if self.ball.position[0] + (self.ball.ball_texture.width() as f32) >= (SCREEN_WIDTH as f32) || self.ball.position[0] <= 0.0 {
            if self.ball.position[0] <= 0.0 {
                // bounced off left wall
                self.player_score += 1;
            } else {
                // bounced off right wall
                self.enemy_score += 1;
            }

            // reset ball to centre
            self.ball.reset();
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let text = Text::new(
            format!("{}-{}", self.enemy_score, self.player_score),
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
    let mut gs = GameState::new;
    ContextBuilder::new("Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(gs)
}