pub struct Timers {
    delay: u8,
    sound: u8,
}

impl Timers {
    /// initializes a new timer
    pub fn new(d: u8, s: u8) -> Timers {
        Timers { delay: d, sound: s }
    }
    /// Updates the value of both timers
    /// Need to add a beep function for the sound timer
    pub fn update(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            // Beep function goes here
            self.sound -= 1;
        }
    }

    /// Sets the delay timer to the given value
    pub fn set_delay(&mut self, value: u8) {
        self.delay = value;
    }

    /// Sets the sound timer to the given value
    pub fn set_sound(&mut self, value: u8) {
        self.sound = value;
    }

    /// Gets the value of the delay timer.
    /// There is no opcode to get the value of the sound timer
    pub fn get(&self) -> u8 {
        self.delay
    }

    /// Should not be used by the rest of the program, only needed for testing
    /// Returns the value of the sound timer
    pub fn __get_sound(&self) -> u8 {
        self.sound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_set() {
        let mut t = Timers::new(10, 10);

        // Test that new works properly
        assert_eq!(10, t.get());
        assert_eq!(10, t.__get_sound());

        // Set the timers to new values
        t.set_delay(11);
        t.set_sound(12);

        // Test that those new values were applied sucessfully
        assert_eq!(11, t.get());
        assert_eq!(12, t.__get_sound());
    }

    #[test]
    fn test_update() {
        // Initialize a timer to cover all cases
        // case 0: timers are the same
        // case 1: sound runs out first
        // case 2: delay runs out first
        //
        // The case of both 0 happens halfway-3/4 of the way through these
        // tests, so that doesn't require a separate case
        //
        let mut t_same = Timers::new(127, 127);
        let mut t_sound_first = Timers::new(127, 63);
        let mut t_delay_first = Timers::new(63, 127);

        // Update the timers 255 times. Timers should be at 0 halfway through
        for i in (0..255).rev() {
            t_same.update();
            t_sound_first.update();
            t_delay_first.update();

            // For the first 128 iterations, make sure that the timers that are
            // set to 127 are counting down properly
            if i > 127 {
                assert_eq!(i - 128 as u8, t_same.get());
                assert_eq!(i - 128 as u8, t_same.__get_sound());
                assert_eq!(i - 128 as u8, t_sound_first.get());
                assert_eq!(i - 128 as u8, t_delay_first.__get_sound());
            } else {
                // Make sure that they remain at 0 for the next 128 iterations
                assert_eq!(0, t_same.get());
                assert_eq!(0, t_same.__get_sound());
                assert_eq!(0, t_delay_first.__get_sound());
                assert_eq!(0, t_sound_first.get());
            }

            // For the first 64 iterations, make sure that the timers that are
            // set to 63 are counting down properly
            if i > 191 {
                assert_eq!(i - 192 as u8, t_delay_first.get());
                assert_eq!(i - 192 as u8, t_sound_first.__get_sound());
            } else {
                // Make sure that they remain at 0 for the next 192 iterations
                assert_eq!(0, t_sound_first.__get_sound());
                assert_eq!(0, t_delay_first.get());
            }
        }
    }
}
