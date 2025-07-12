use plotters::prelude::*;
use std::time::Instant;

// I have no idea what is going on here. This was made at 10pm with the (albeit useless) help of RRs
// built-in AI. At this point I am going to leave this alone and just suffer with the fact it is
// unoptimised, unintelligent, piece of shit.

pub fn plot_trajectories(all_positions: &Vec<Vec<(f32, f32)>>) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let frame_count = if let Some(first) = all_positions.first() {
        println!("Total frames to process: {}", first.len());
        first.len()
    } else {
        println!("No positions to plot");
        return Ok(());
    };

    let (global_min_x, global_max_x, global_min_y, global_max_y) = all_positions.iter()
        .flat_map(|positions| positions.iter())
        .fold((f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY),
              |acc, &(x, y)| {
                  (acc.0.min(x), acc.1.max(x), acc.2.min(y), acc.3.max(y))
              });

    let global_width = global_max_x - global_min_x;
    let global_height = global_max_y - global_min_y;

    let padding = 0.1;
    let view_min_x = global_min_x - global_width * padding;
    let view_max_x = global_max_x + global_width * padding;
    let view_min_y = global_min_y - global_height * padding;
    let view_max_y = global_max_y + global_height * padding;

    let colors = [&RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &YELLOW, &BLACK, &WHITE];

    // Create a GIF file
    let root = BitMapBackend::gif("bodies_trajectory.gif", (800, 600), 100)?
        .into_drawing_area();

    println!("Starting to generate frames...");

    for frame in (0..frame_count).step_by(5) {
        if frame % 10 == 0 {
            println!("Processing frame {}/{}", frame, frame_count);
        }

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Bodies Trajectories", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                view_min_x..view_max_x,
                view_min_y..view_max_y
            )?;

        chart.configure_mesh().draw()?;

        for (i, positions) in all_positions.iter().enumerate() {
            // To draw full list of points faintly
            // chart.draw_series(LineSeries::new(
            //     positions.iter().map(|&(x, y)| (x, y)),
            //     colors[i % colors.len()].mix(0.1)
            // ))?;

            if frame > 0 {
                chart.draw_series(LineSeries::new(
                    positions[0..=frame].iter().map(|&(x, y)| (x, y)),
                    colors[i % colors.len()].mix(0.5)
                ))?;
            }

            chart.draw_series(PointSeries::of_element(
                vec![positions[frame]],
                8,
                colors[i % colors.len()],
                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
            ))?.label(format!("Body {}", i))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], colors[i % colors.len()]));
        }

        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }

    let duration = start_time.elapsed();
    println!("Animation generated in {:?}", duration);
    println!("GIF saved as 'bodies_trajectory.gif'");

    Ok(())
}
