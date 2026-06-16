/// first version of t3t us main and t3t contained on src
/// Trinitarian Transducer (T3T)
/// Based on the Pure Functional Transducer Manifesto.
/// License: GPLv3

struct TrinitarianTransducer {
    register: u8, // We use u8, but only the lowest 3 bits are relevant
}

impl TrinitarianTransducer {
    fn new() -> Self {
        TrinitarianTransducer { register: 0b000 }
    }

    /// The translation function: the core of the mechanism
    /// Receives a noise bit (0 or 1) and executes the phase cycle
    fn translate(&mut self, noise: u8) {
        let input = noise & 0b001; // Isolate the noise bit

        // 1. AND GATE (The Filter)
        // Cross-reference the current state with the incoming noise
        let and_result = self.register & input;

        // 2. HILBERT TURN AND MOD3 FILTER
        // Shift to change phase (90-degree turn)
        let next_phase = and_result << 1;

        // 3. LAW OF SELF-REGULATION (/-AND via overflow)
        // If the bit exceeds the 3-bit cube (> 7), force a reset to 000
        if next_phase > 0b111 {
            self.register = 0b000;
        } else {
            self.register = next_phase;
        }
    }

    fn show_state(&self) {
        println!("Current state: {:03b}", self.register & 0b111);
    }
}

fn main() {
    let mut motor = TrinitarianTransducer::new();
    let noise_flow = [1, 1, 1, 0, 1]; // Example of a stochastic noise burst

    println!("Starting the Trinitarian Transducer...");
    
    for (i, &bit) in noise_flow.iter().enumerate() {
        motor.translate(bit);
        print!("Cycle {}: Input {}, ", i + 1, bit);
        motor.show_state();
    }
}
