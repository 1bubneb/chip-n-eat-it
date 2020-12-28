/// A sprite is 8px wide by 1-16px tall with 1-bit color. Therefore, the sprite
/// should be stored as a Vec<u8>.
///
/// The 0th indice of the vector will be the top of the sprite; the nth indice
/// of the vector will be the bottom, like so:
/// | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |   |
/// |---+---+---+---+---+---+---+---+---|
/// |   |   |   |   |   |   |   |   | 0 |
/// |   |   | X | X | X | X |   |   | 1 |
/// |   | X |   |   |   |   | X |   | 2 |
/// |   | X | X |   |   | X | X |   | 3 |
/// |   | X |   |   |   |   | X |   | 4 |
/// |   | X |   | X | X |   | X |   | 5 |
/// |   |   | X | X | X | X |   |   | 6 |
/// |   |   |   |   |   |   |   |   | 7 |
///   1   2   4   8   16  32  64 128
///   0000 0000 0x00
///   0011 1100 0x3C
///   0100 0010 0x82
///   0110 0110 0x66
///   0100 0010 0x42
///   0101 1010 0x5A
///   0011 1100 0x3C
///   0000 0000 0x00

pub struct Sprite {
    lines: Vec<u8>,
    height: u8,
}

impl Sprite {
    /// Create a new sprite from a Vec<u8> The sprite struct is just a wrapper
    /// around the Vec<T> built-in.
    ///
    /// This is also done so that a sprite can be easily dumped in and out of
    /// memory as that module communicates with Vector slices
    ///
    pub fn new(sprite: Vec<u8>, h: u8) -> Sprite {
        if h > 16 {
            panic!("Sprite too tall");
        }
        Sprite {
            lines: sprite,
            height: h,
        }
    }

    /// Returns a Vec<u8> so that the sprite can be dumped into the framebuffer
    pub fn dump(&self) -> Vec<u8> {
        let mut dumped_sprite: Vec<u8> = Vec::new();

        // Loop needs to be done this way so as not to anger the borrow checker
        for i in 0..self.height {
            dumped_sprite.push(self.lines[i as usize]);
        }

        dumped_sprite
    }

    /// Loads a sprite into the Sprite struct
    pub fn read(&mut self, sprite: Vec<u8>) {
        self.lines = sprite;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite() {
        // Create a Vec with the bits to draw a 8x8 smiley face as its contents
        let mut v: Vec<u8> = vec![0x00, 0x3C, 0x82, 0x66, 0x42, 0x5A, 0x3C, 0x00];
        let mut s: Sprite = Sprite::new(v, 8);

        // Assert that the sprite loaded is correct
        // Also makes sure that Sprite::sprite.dump() is working
        assert_eq!(
            vec![0x00, 0x3C, 0x82, 0x66, 0x42, 0x5A, 0x3C, 0x00],
            s.dump()
        );

        s.read(vec![0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x30]);

        // Load a new sprite, make sure that it's correct
        assert_eq!(
            vec![0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x30],
            s.dump()
        );
    }
}
