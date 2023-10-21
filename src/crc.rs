pub struct Crc([u32; 256]);

impl Crc {
    pub fn new(&mut self) {
        for n in 0..256 {
            let mut c = n as u32;
            for _k in 0..8 {
                if c & 1 == 1 {
                    c = 0xedb88320 ^ (c >> 1);
                } else {
                    c = c >> 1;
                }
            }
            self.0[n] = c;
        }
    }

    pub fn update(&mut self, buffer: &[u8]) -> u32 {
        let mut c = 0xffffffff;
        buffer.iter().for_each(|b| {
            c = self.0[((c ^ *b as u32) & 0xff) as usize] ^ (c >> 8)
        });
        c ^ 0xffffffff
    }
}


