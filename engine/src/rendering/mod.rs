use raylib::prelude::{
    RaylibDraw,
    RaylibDrawHandle 
};


use crate::{
    GameEngine,
    graphics::colors::Color, 
};


pub fn start_new_scene(engine: &mut GameEngine) -> RaylibDrawHandle<'_> {

    let thread = &engine.rl_thread;
    let mut draw_thread = engine.rl_handle.begin_drawing(thread);
    draw_thread.clear_background(Color::BLACK.to_rgb().to_raylib_color());
    draw_thread

}


