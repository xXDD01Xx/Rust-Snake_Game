use rand::*;
use piston_window::*;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coordinate_u32;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main()
{
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new
        (
            "Snake",        //create game window
            [to_coordinate_u32(width), to_coordinate_u32(height)],
        ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new_game(width, height);
    while let Some(event) = window.next()
    {
            if let Some(Button::Keyboard(key)) = event.press_args()
            {
                game.key_pressed(key);
            }
            window.draw_2d(&event, |context, graphics_buffer, _|
                {
                    clear(BACKGROUND_COLOR, graphics_buffer);
                    game.draw(&context, graphics_buffer);
                });

                event.update(|update_args|
                    {
                       game.update(update_args.dt);
                    });
        }
    }

