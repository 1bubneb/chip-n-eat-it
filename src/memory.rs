pub struct Ram {
    r: Vec<u8>,
}

impl Ram {
    /// Returns a 4096 byte Vec with every item initialized to 0
    pub fn new() -> Ram {
        Ram { r: vec![0; 4096] }
    }

    /// Writes 1 byte to memory at the specified location
    pub fn write(&mut self, loc: usize, x: u8) {
        // Check that loc is within the bounds
        if loc > 4095 {
            panic!("mem out of bounds");
        } // loc is unsigned, no need to check for negative values

        self.r[loc as usize] = x;
    }

    /// Returns the byte contained in a specified memory location
    pub fn read(&self, loc: usize) -> u8 {
        // check that we're reading within bounds
        if loc > 4095 {
            panic!("read out of bounds!");
        }

        self.r[loc]
    }

    /// Used for opcodes FX65 and DXYN
    /// Reads a number of entries from memory and returns a Vec<u8> of their
    /// contents. A function under the 'register' mod should handle loading that
    /// Vec<u8> into the correct registers
    pub fn read_seq(&self, start_addr: usize, entries: usize) -> Vec<u8> {
        if start_addr + 16 > 4096 {
            panic!("sequential read out of bounds!");
        }

        let mut ret: Vec<u8> = Vec::new();

        for i in 0..entries {
            ret.push(self.read(start_addr + i));
        }

        // return the vector
        ret
    }

    /// Implements opcode FX55, dump V0..VX into memory at I. Does not clear
    /// the V registers, that should be handled in the 'register' module
    pub fn write_seq(&mut self, start_addr: usize, regs: &mut Vec<u8>) {
        let mut i: usize = 0;
        while let Some(value) = regs.pop() {
            self.write(start_addr + i, value);
            i += 1;
        }
    }

    /// Writes the BCD (Binary Coded Decimal) representation of the character
    /// stored in VX to memory at the specified location
    pub fn write_bcd(&mut self, start_addr: usize, value: u8) {
        let hundreds: u8 = value / 100;
        let tens: u8 = (value % 100) / 10;
        let ones: u8 = value % 10;

        self.write(start_addr, hundreds);
        self.write(start_addr + 1, tens);
        self.write(start_addr + 2, ones);
    }

    /// Swap two locations, not actually in the opcode table, but trivial to
    /// implement and *might* help with optimizing something
    pub fn swap(&mut self, l1: usize, l2: usize) {
        // Bounds check
        if l1 > 4095 || l2 > 4095 {
            panic!("read/write out of bounds!");
        }

        let t1: u8 = self.read(l1);
        let t2: u8 = self.read(l2);

        self.write(l1, t2);
        self.write(l2, t1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rws() {
        let mut r = Ram::new();

        // Make sure _all_ locations are writable
        for i in 0..4096 {
            r.write(i, (i % 255) as u8);
            assert_eq!((i % 255) as u8, r.read(i));
        }

        r.write(0, 69);
        r.write(1, 42);
        r.swap(0, 1);

        assert_eq!(42, r.read(0));
        assert_eq!(69, r.read(1));
    }

    #[test]
    fn test_read_seq() {
        let mut r = Ram::new();
        let mut v: Vec<u8> = Vec::new();

        // Copied from test_rws() above, used to initialize a the memory with
        // some known values to test that sequential reads work
        for i in 0..4096 {
            r.write(i, (i % 255) as u8);
            assert_eq!((i % 255) as u8, r.read(i));
        }

        // Read the first 0xF bytes of r into v
        v = r.read_seq(0, 16);

        // make sure that everything read into v is correct
        // v is popped after each read so that we can pop again after the loop
        // to make sure that we read the correct amount
        for i in (0..16).rev() {
            assert_eq!(i, v.pop().unwrap());
        }

        assert_eq!(None, v.pop());
    }

    #[test]
    fn test_write_seq() {
        let mut r = Ram::new();
        let mut v1: Vec<u8> = Vec::new();
        let mut v2: Vec<u8> = Vec::new();

        // Load a mock register array to dump later
        for i in (0..16).rev() {
            v1.push(i as u8);
        }

        // Dump the mock register array
        r.write_seq(16, &mut v1);

        // Load the dumped data into a different register array
        // Note: The real program will dump back into the same array, but
        // dumping into a different one is suffcient for testing
        v2 = r.read_seq(16, 16);

        let mut j: usize = 0;
        for i in &v1 {
            assert_eq!(*i, v2[j]);
            j += 1;
        }

        // Test Dumping a slice of the registers
    }

    #[test]
    fn test_bcd() {
        let mut r = Ram::new();

        // Write 123 to 0x69 as BCD
        r.write_bcd(69, 123);

        assert_eq!(1, r.read(69));
        assert_eq!(2, r.read(70));
        assert_eq!(3, r.read(71));
    }
}
