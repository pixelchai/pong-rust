use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::math::Vec2;
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

// visual consts
const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const FONT_SIZE: f32 = 32.0;
const PADDING: f32 = FONT_SIZE;

// gameplay consts
const PADDLE_SPEED: f32 = 14.0;
const BALL_SPEED: f32 = PADDLE_SPEED/2.0;
const PADDLE_SPIN: f32 = 3.0;
const BALL_ACC: f32 = 0.005;

// AI constants
const AI_ENABLED: bool = true;
const AI_MAX_ITERS: u32 = 400; // Experimentation results: around 800 is more than sufficient,
                               // 400 is quite good though is insufficient for a short time after ball leaves enemy paddle
const AI_WAIT_FOR_PLAYER_HIT: bool = true;  // wait for the player to hit the ball first before calculating solution
                                            // (= will not have to guess the player's angle of attack)
                                            // NB: if waiting for player hit, max iters may be set to a lower value
const EPSILON: f32 = 1.0;

#[derive(Clone)]
struct Paddle {
    paddle_texture: Texture,
    position: Vec2<f32>,    
}

#[derive(Clone)]
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
    enemy_hit: bool,  // used when simulating
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // init textures
        let paddle_texture = Texture::new(ctx, "res/paddle.png")?;
        let ball_texture = Texture::new(ctx, "res/ball.png")?;
        
        // init ball
        let mut ball = Ball {
            ball_texture,
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(1.0, 1.0),
        };
        ball.reset();  // initialise ball's position
        ball.velocity = ball.velocity.normalized() * BALL_SPEED; // init ball speed

        // calculate paddle initial y
        let paddle_initial_y = (SCREEN_HEIGHT as f32)/2.0 - (paddle_texture.height() as f32)/2.0;

        Ok(GameState {
            ball,
            player_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(
                    (SCREEN_WIDTH as f32) - PADDING - (paddle_texture.width() as f32), 
                    paddle_initial_y,
                ),
            },
            player_score: 0,
            enemy_paddle: Paddle {
                paddle_texture: paddle_texture.clone(),
                position: Vec2::new(
                    PADDING, 
                    paddle_initial_y,
                ),
            },
            enemy_score: 0,
            simulated: false,
            enemy_hit: false,
        })
    }

    fn draw_paddle(ctx: &mut Context, paddle: &Paddle){
        graphics::draw(ctx, &paddle.paddle_texture, paddle.position)
    }

    fn handle_inputs(&mut self, ctx: &mut Context){
        if input::is_key_down(ctx, Key::W) {
            self.player_paddle.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player_paddle.position.y += PADDLE_SPEED;
        }

        // if !AI_ENABLED {
        if !false {
            if input::is_key_down(ctx, Key::O) {
                self.enemy_paddle.position.y -= PADDLE_SPEED;
            }
            if input::is_key_down(ctx, Key::L) {
                self.enemy_paddle.position.y += PADDLE_SPEED;
            }
        }
    }

    /// Check for ball-paddle collision with the given paddle
    fn check_intersects(ball: &Ball, paddle: &Paddle) -> bool{   
        // check if ball's centre point is inside paddle rectangle:
        // method adapted from: https://stackoverflow.com/a/2763387/5013267
        let ab = Vec2::new(paddle.paddle_texture.width() as f32, 0.0); // vector a->b
        let bc = Vec2::new(0.0, paddle.paddle_texture.height() as f32); // vector b->c

        let m = ball.position + Vec2::new(ball.ball_texture.width() as f32, ball.ball_texture.height() as f32)/2.0;

        let ab_dot_am = ab.dot(m - paddle.position);
        let bc_dot_bm = bc.dot(m - (paddle.position + (paddle.paddle_texture.width() as f32, 0.0)));

        // return value:
        0.0 <= ab_dot_am && ab_dot_am <= ab.dot(ab)
        && 0.0 <= bc_dot_bm && bc_dot_bm <= bc.dot(bc)
    }

    fn apply_collision_response(ball: &mut Ball, paddle: &Paddle){
        ball.velocity.x = -(ball.velocity.x + (BALL_ACC * ball.velocity.x.signum()));

        let offset = (paddle.position.y - ball.position.y) / paddle.paddle_texture.height() as f32;
        ball.velocity.y += PADDLE_SPIN * -offset;
    }

    fn update_collision(ball: &mut Ball, paddle: &Paddle){
        if GameState::check_intersects(ball, &paddle){
            GameState::apply_collision_response(ball, paddle);
        }
    }

    fn update_ball(&mut self, _ctx: &mut Context){
        self.update_ai(_ctx);
        self.ball.position += self.ball.velocity;

        if !self.simulated {
            GameState::update_collision(&mut self.ball, &self.player_paddle);
            GameState::update_collision(&mut self.ball, &self.enemy_paddle);
        }else {
            // if simulated, use simplified calculations
            // (always assume ball hits player paddle, otherwise AI would win anyway)
            // only need to check player paddle
            if self.ball.position.x + ((self.ball.ball_texture.width() as f32)/2.0) >= self.player_paddle.position.x {
                GameState::apply_collision_response(&mut self.ball, &mut self.player_paddle);
            }
            
            // check reaches enemy's side (so that iteration can be terminated)
            if self.ball.position.x <= self.enemy_paddle.position.x + self.enemy_paddle.paddle_texture.width() as f32 {
                self.enemy_hit = true;
                return;  // no need to do rest of update calculations
            }
        }

        // walls
        // if bouncing off top or bottom walls...
        if (self.ball.position[1] + (self.ball.ball_texture.height() as f32) >= (SCREEN_HEIGHT as f32)) || self.ball.position[1] <= 0.0 {
            self.ball.velocity[1] = -self.ball.velocity[1];
        }

        // if bouncing off either of the side walls...
        if self.ball.position[0] + (self.ball.ball_texture.width() as f32) >= (SCREEN_WIDTH as f32) || self.ball.position[0] <= 0.0 {
            if self.ball.position[0] <= 0.0 {
                // bounced off left wall
                self.player_score += 1;
            } else {
                // bounced off right wall
                self.enemy_score += 1;
                self.ball.velocity = Vec2::new(1.0, 1.0); // setting direction
            }

            // reset ball to centre
            self.ball.reset();

            // reset ball speed (but not direction)
            self.ball.velocity = self.ball.velocity.normalized() * BALL_SPEED;
        }
    }

    fn update_ai(&mut self, ctx: &mut Context){
        if self.simulated || !AI_ENABLED {
            return;
        }

        if AI_WAIT_FOR_PLAYER_HIT && self.ball.velocity.x >= 0.0 {
            // ball vel.x >= 0.0 implies ball moving towards player still, and has not been returned yet
            return;
        }

        // create a simulation GameState, cloned from real GameState
        let mut sim = GameState {
            ball: self.ball.clone(),
            player_paddle: self.player_paddle.clone(),
            player_score: self.player_score,
            enemy_paddle: self.enemy_paddle.clone(),
            enemy_score: self.enemy_score,
            simulated: true,
            enemy_hit: false,
        };

        for i in 0..AI_MAX_ITERS {
            if !sim.enemy_hit {
                sim.update(ctx).expect("bruh moment when updating sim");
                // sim.draw(ctx).expect("bruh moment when drawing sim"); // NB: only for debug -- rendering here slows down program signficantly
            } else {
                // if enemy_hit, stop iterating: found solution
                // TODO: maybe implement solution caching 
                //       (but low prio because solution prediction is variable anyway [depends on other player's angle of attack])

                let target_y = sim.ball.position.y + (sim.ball.ball_texture.height() as f32)/2.0 
                               - (self.enemy_paddle.paddle_texture.height() as f32)/2.0;
                
                let delta = target_y - self.enemy_paddle.position.y;
                
                if delta.abs() > EPSILON {
                    self.enemy_paddle.position.y += (delta.abs()).min(PADDLE_SPEED).copysign(delta);
                } else {
                    self.enemy_paddle.position.y = target_y;
                }
                break;
            }
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

        if !self.simulated {
            graphics::draw(ctx, &self.ball.ball_texture, self.ball.position);
        } else {
            // for debugging, render simulated run in different shade 
            // (visualisation may not be used in final version)
            graphics::draw(ctx, &self.ball.ball_texture, DrawParams::new()
            .position(self.ball.position)
            .color(Color::rgb(1.0, 0.0, 0.0)));
        }
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