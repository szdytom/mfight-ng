use tetra::graphics::{self, Color, Rectangle, Texture, text::{Text, Font}};
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;

struct Entity {
	texture: Texture,
	position: Vec2<f32>,
	velocity: Vec2<f32>,
}

impl Entity {
	fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
		Entity { texture, position, velocity }
	}

	fn draw(&self, ctx: &mut Context) {
		self.texture.draw(ctx, self.position);
	}

	fn fix_position(&mut self) {
		let max_y = WINDOW_HEIGHT - self.height();
		if self.position.y > max_y {
			self.position.y = max_y;
		} else if self.position.y < 0.0 {
			self.position.y = 0.0;
		}
	}

	fn width(&self) -> f32 {
		self.texture.width() as f32
	}

	fn height(&self) -> f32 {
		self.texture.height() as f32
	}

	fn bounds(&self) -> Rectangle {
		Rectangle::new(
			self.position.x,
			self.position.y,
			self.width(),
			self.height(),
		)
	}

	fn centre(&self) -> Vec2<f32> {
		Vec2::new(
			self.position.x + self.width()  / 2.0,
			self.position.y + self.height() / 2.0,
		)
	}
}

struct GameState {
	player1: Entity,
	player2: Entity,
	ball: Entity,
	font: Font,
	end_text: Option<Text>,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let font = Font::vector(ctx, "./resources/Ubuntu-MI.ttf", 44.0)?;

		let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
		let player1_position = Vec2::new(
			16.0,
			(WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
		);

		let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
		let player2_position = Vec2::new(
			WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
			(WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
		);

		let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
		let ball_position = Vec2::new(
			(WINDOW_WIDTH -  ball_texture.width()  as f32) / 2.0,
			(WINDOW_HEIGHT - ball_texture.height() as f32) / 2.0,
		);

		Ok(GameState {
			player1: Entity::new(player1_texture, player1_position, Vec2::zero()),
			player2: Entity::new(player2_texture, player2_position, Vec2::zero()),
			ball:	Entity::new(ball_texture,	ball_position   , Vec2::new(-BALL_SPEED, 0.0)),
			font,
			end_text: None,
		})
	}
}

impl State for GameState {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		if self.end_text.is_some() {
			return Ok(());
		}

		if input::is_key_down(ctx, Key::W) {
			self.player1.position.y -= PADDLE_SPEED;
		}
		if input::is_key_down(ctx, Key::S) {
			self.player1.position.y += PADDLE_SPEED;
		}
		self.player1.fix_position();

		if input::is_key_down(ctx, Key::Up) {
			self.player2.position.y -= PADDLE_SPEED;
		}
		if input::is_key_down(ctx, Key::Down) {
			self.player2.position.y += PADDLE_SPEED;
		}
		self.player2.fix_position();
		
		self.ball.position += self.ball.velocity;
		
		let ball_bounds = self.ball.bounds();
		let paddle_hit = if ball_bounds.intersects(&self.player1.bounds()) {
			Some(&self.player1)
		} else if ball_bounds.intersects(&self.player2.bounds()) {
			Some(&self.player2)
		} else {
			None
		};

		if let Some(paddle) = paddle_hit {
			// Increase the ball's velocity, then flip it.
			self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

			// Calculate the offset between the paddle and the ball, as a number between
			// -1.0 and 1.0.
			let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
	
			// Apply the spin to the ball.
			self.ball.velocity.y += PADDLE_SPIN * -offset;
		}

		if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
			self.ball.velocity.y = -self.ball.velocity.y;
		}

		if self.ball.position.x < 0.0 {
			self.end_text = Some(Text::new("Player 2 win!", self.font.clone()));
		} else if self.ball.position.x > WINDOW_WIDTH {
			self.end_text = Some(Text::new("Player 1 win!", self.font.clone()));
		}
	
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
		
		if let Some(text) = &mut self.end_text {
			text.draw(ctx, Vec2::new(
				WINDOW_WIDTH / 2.0 - 140.0,
				WINDOW_HEIGHT / 2.0 - 22.0,
			));
			return Ok(());
		}

		self.player1.draw(ctx);
		self.player2.draw(ctx);
		self.ball.draw(ctx);

		Ok(())
	}
}

const WINDOW_WIDTH:  f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED:  f32 = 8.0;
const BALL_SPEED:	f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
	ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
		.build()?
		.run(GameState::new)
}

