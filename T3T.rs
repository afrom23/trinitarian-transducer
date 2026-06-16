/// Motor Trinitario (T3T)
/// Basado en el Manifiesto del Traductor Funcional Puro.
/// Licencia: GPLv3

struct MotorTrinitario {
    registro: u8, // Usamos u8 pero solo nos importan los 3 bits inferiores
}

impl MotorTrinitario {
    fn new() -> Self {
        MotorTrinitario { registro: 0b000 }
    }

    /// La función de traducción: el corazón del mecanismo
    /// Recibe un bit de ruido (0 o 1) y ejecuta el ciclo de fase
    fn traducir(&mut self, ruido: u8) {
        let entrada = ruido & 0b001; // Aislamos el bit de ruido

        // 1. COMPUERTA AND (El Filtro)
        // Cruzamos el estado actual con el ruido entrante
        let resultado_and = self.registro & entrada;

        // 2. GIRO DE HILBERT Y FILTRO MOD3
        // Desplazamos para cambiar de fase (giro 90 grados)
        let siguiente_fase = resultado_and << 1;

        // 3. LA LEY DE AUTORREGULACIÓN (/-AND por desborde)
        // Si el bit sale del cubo de 3 bits ( > 7), forzamos el reset a 000
        if siguiente_fase > 0b111 {
            self.registro = 0b000;
        } else {
            self.registro = siguiente_fase;
        }
    }

    fn ver_estado(&self) {
        println!("Estado actual: {:03b}", self.registro & 0b111);
    }
}

fn main() {
    let mut motor = MotorTrinitario::new();
    let flujo_ruido = [1, 1, 1, 0, 1]; // Ejemplo de ráfaga de ruido estocástico

    println!("Iniciando el Motor Trinitario...");
    
    for (i, &bit) in flujo_ruido.iter().enumerate() {
        motor.traducir(bit);
        print!("Ciclo {}: Entrada {}, ", i + 1, bit);
        motor.ver_estado();
    }
}
