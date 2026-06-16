mod t3t;
use t3t::TrinitarianEngine;
use rand::Rng;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = TrinitarianEngine::new();
    let mut rng = rand::thread_rng();
    
    // Generar datos
    let mut data = Vec::new();
    for _ in 0..50 {
        let bit = rng.gen_range(0..2);
        let output = engine.process_stream(&[bit])[0];
        data.push((bit, output));
    }

    // Configurar el gráfico
    let root = BitMapBackend::new("t3t_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Dinámica del Motor T3T", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..50, 0..8)?; // Eje Y hasta 7 (binario 111)

    chart.configure_mesh().draw()?;

    // Dibujar la salida del motor
    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(i, &(_, out))| (i as i32, out as i32)), // Convertimos a i32 explícitamente
        &BLUE,
    ))?;

    println!("Gráfica generada: t3t_plot.png");
    Ok(())
}