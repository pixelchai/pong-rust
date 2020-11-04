use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let text = Text::new(
            "Hello, world!\n\nThis is some text being rendered from a TTF font.",
            Font::vector(ctx, "res/vcr_osd_mono.ttf", 16.0)?,
        );
        graphics::draw(ctx, &text, Vec2::new(16.0, 16.0));
        Ok(())
    }

    // fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    //     println!("heyeyyeyey");
    //     Ok(())
    // }
    
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState))
}