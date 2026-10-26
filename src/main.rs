mod app;
mod renderer;

use app::MinecraftApp;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    // 2. Den Event-Loop vom Betriebssystem erstellen
    let event_loop = EventLoop::new()?;

    // 3. Unsere eigene App-Instanz erzeugen
    let mut minecraft_app = MinecraftApp::new();

    // 4. Den Event-Loop starten und unsere App übergeben
    event_loop.run_app(&mut minecraft_app)?;

    Ok(())
}