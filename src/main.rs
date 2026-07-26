use crate::{interaction_medium::console_window::starter::ConsoleWindow, services::ui_renderer::{UIRender, Constructor}};

mod interaction_medium;
mod services;
mod helpers;
mod handlers;
mod models;
mod managers;
mod common;
mod constants;
fn main(){
    let ui_renderer = UIRender::<ConsoleWindow>::new();
    ui_renderer.start();
}