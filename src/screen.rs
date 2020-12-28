pub struct Screen {
    framebuffer: Vec<Vec<u8>>,
    // Maybe add SDL/IMGUI connectors here?
}

impl Screen {
    /// Returns a Vec of 32 empty vectors
    pub fn new() -> Screen {
        Screen {
            framebuffer: vec![Vec::new(); 32],
        }
    }

    /// Calls the graphics/UI library and draws the frame buffer to the display
    pub fn refresh(&self) {
        // Decide on UI library before I write this
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &Vec<u8>) {}
}
