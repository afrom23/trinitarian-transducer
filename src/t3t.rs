pub struct TrinitarianTransducer {
    register: u8,
}

impl TrinitarianTransducer {
    pub fn new() -> Self {
        Self { register: 0 }
    }

    pub fn translate(&mut self, noise: u8) -> u8 {
        let input = noise & 0b001;
        if self.register == 0 { self.register = 1; }
        let and_result = self.register & input;
        let next_phase = and_result << 1;

        if next_phase > 0b111 {
            self.register = 0b001;
        } else {
            self.register = next_phase;
        }
        self.register
    }
}

pub struct TrinitarianEngine {
    nonios: [TrinitarianTransducer; 3],
}

impl TrinitarianEngine {
    pub fn new() -> Self {
        Self {
            nonios: [
                TrinitarianTransducer::new(),
                TrinitarianTransducer::new(),
                TrinitarianTransducer::new(),
            ],
        }
    }

    pub fn process_stream(&mut self, noise_stream: &[u8]) -> Vec<u8> {
        noise_stream.iter().map(|&bit| {
            let mut combined_output = 0;
            for (i, nonio) in self.nonios.iter_mut().enumerate() {
                combined_output |= nonio.translate(bit) << i;
            }
            combined_output & 0b111
        }).collect()
    }
}