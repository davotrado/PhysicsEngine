use plotters::prelude::*;

pub fn plot_trajectories(all_positions: &Vec<Vec<(f32, f32)>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("bodies_trajectory.png", 
                                  (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let min_x = all_positions.iter()
        .flat_map(|positions| positions.iter().map(|(x, _)| x))
        .fold(f32::INFINITY, |a, &b| a.min(b));
    let max_x = all_positions.iter()
        .flat_map(|positions| positions.iter().map(|(x, _)| x))
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let min_y = all_positions.iter()
        .flat_map(|positions| positions.iter().map(|(_, y)| y))
        .fold(f32::INFINITY, |a, &b| a.min(b));
    let max_y = all_positions.iter()
        .flat_map(|positions| positions.iter().map(|(_, y)| y))
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    let padding = ((max_x - min_x) * 0.1).max((max_y - min_y) * 0.1);

    let mut chart = ChartBuilder::on(&root)
        .caption("Bodies Trajectories", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (min_x - padding)..(max_x + padding),
            (min_y - padding)..(max_y + padding)
        )?;

    chart.configure_mesh().draw()?;

    let colors = [
        &RED, &BLUE, &GREEN, &CYAN, &MAGENTA,
        &YELLOW, &BLACK, &WHITE
    ];

    for (i, positions) in all_positions.iter().enumerate() {
        chart
            .draw_series(LineSeries::new(
                positions.iter().map(|&(x, y)| (x, y)),
                colors[i % colors.len()]
            ))?
            .label(format!("Body {}", i))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], colors[i % colors.len()]));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())

}