use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use winit::event::{WindowEvent, ElementState};
use winit::keyboard::{Key, NamedKey};
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;

// Wir binden unsere neue Renderer-Datei ein!
use crate::renderer::Renderer;

pub struct MinecraftApp {
    renderer: Option<Renderer>,
}

impl MinecraftApp {
    pub fn new() -> Self {
        Self {
            renderer: None,
        }
    }
}

impl ApplicationHandler for MinecraftApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            // Initialisiere den Renderer asynchron
        let renderer = pollster::block_on(Renderer::new(&event_loop));
        self.renderer = Some(renderer);

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Spiel wird geschlossen...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {

                if let Some(renderer) = &mut self.renderer {
                     renderer.render()
                }
                if let Some(renderer) = &self.renderer {
                    renderer.window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event.key_without_modifiers();
                if event.state == ElementState::Pressed {
                   match key {
                       Key::Named(named_key) => {
                           println!("Key: {:?}", named_key);
                       }
                       Key::Character(ch) => {
                           println!("Key: {:?}", ch);
                       }
                       _ => ()
                   }
                }
            }
            _ => (),
        }
    }
}