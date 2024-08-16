use anyhow::Ok;
use chrono::Utc;
use itertools::Itertools;
use log::info;
use plotters::data::fitting_range;
use plotters::prelude::*;
use std::collections::HashMap;
use std::fs;
use strum::IntoEnumIterator;

use crate::utils::{Action, IotaTangleNetwork, Measurement};

pub fn draw_all_measurements(
    measurements: &HashMap<IotaTangleNetwork, Measurement>,
) -> anyhow::Result<()> {
    let folder_name = format!(
        "test/{}",
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    );
    fs::create_dir(folder_name.clone())?;

    for action in Action::iter() {
        info!("Enum {}", action.name());

        let mut values: Vec<(String, Quartiles)> = Vec::new();

        for (network, durations) in measurements {
            if let Some(durations) = durations.get(&action) {
                // Transform the vector of Durations into a vector of f64 seconds
                info!("Values: {:?}", durations);
                let secs_f64: Vec<f32> = durations.iter().map(|d| d.as_secs_f32()).collect();
                values.push((network.name().to_string(), Quartiles::new(&secs_f64)));
            }
        }

        info!("Values {:?}", values);

        let _ = draw_box_plot(&folder_name, action, values);
    }

    Ok(())
}

fn draw_box_plot(
    folder_name: &str,
    action: Action,
    values: Vec<(String, Quartiles)>,
) -> anyhow::Result<()> {
    let plot_name = format!("{}/{}_boxplot.png", folder_name, action.name()).replace(" ", "_");
    info!("{}", plot_name);
    let root = BitMapBackend::new(&plot_name, (640, 480)).into_drawing_area();
    // let root = SVGBackend::new(&plot_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let host_list: Vec<_> = values
        .iter()
        .unique_by(|x| x.0.clone())
        .map(|x| x.0.clone())
        .collect();

    let mut colors = (0..).map(Palette99::pick);

    let flat_values: Vec<f32> = values.iter().flat_map(|x| x.1.values().to_vec()).collect();
    let values_range = fitting_range(flat_values.iter());
    let value_padding = (values_range.end - values_range.start) * 0.1;
    let chart_caption = format!("{} Boxplot", action.name());

    let mut chart = ChartBuilder::on(&root)
        .caption(chart_caption, ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            host_list[..].into_segmented(),
            values_range.start - value_padding..values_range.end + value_padding,
        )?;

    chart
        .configure_mesh()
        .y_desc("Duration, seconds")
        .x_labels(host_list.len())
        .y_label_style(("sans-serif", 15))
        .x_label_style(("sans-serif", 15))
        .axis_desc_style(("sans-serif", 20))
        .light_line_style(WHITE)
        .draw()?;

    chart.draw_series(values.iter().map(|x| {
        Boxplot::new_vertical(SegmentValue::CenterOf(&x.0), &x.1)
            .width(20)
            .whisker_width(0.5)
            .style(colors.next().unwrap())
    }))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", plot_name);

    Ok(())
}

pub fn draw_custom() -> anyhow::Result<()> {
    let data = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0, 5.5, 6.0, 7.0, 8.0];

    // Calculate the statistics manually
    let min = *data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let q1 = percentile(&data, 25.0);
    let median = percentile(&data, 50.0);
    let q3 = percentile(&data, 75.0);

    let root = BitMapBackend::new("boxplot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Boxplot with Min/Max Whiskers",
            ("sans-serif", 40).into_font(),
        )
        .x_label_area_size(30)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(0_f64..1_f64, 0.0_f64..10.0_f64)?;

    chart.configure_mesh().disable_x_mesh().draw()?;

    // Draw the boxplot
    let box_x = 0.5;

    chart.draw_series(vec![
        // Draw whiskers
        PathElement::new(vec![(box_x, min), (box_x, q1)], &BLACK), // Lower whisker
        PathElement::new(vec![(box_x, q3), (box_x, max)], &BLACK), // Upper whisker
        // Draw the median
        PathElement::new(vec![(box_x - 0.1, median), (box_x + 0.1, median)], &BLACK),
    ])?;

    chart.draw_series(vec![
        // Draw the box
        Rectangle::new(
            [(box_x - 0.1, q1), (box_x + 0.1, q3)],
            BLUE.mix(0.3).filled(),
        ),
    ])?;

    root.present()?;
    println!("Boxplot has been saved to 'boxplot.png'");

    Ok(())
}

// Function to calculate percentiles
fn percentile(data: &Vec<f64>, percentile: f64) -> f64 {
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = (percentile / 100.0) * (sorted_data.len() - 1) as f64;
    let f = k.floor();
    let c = k.ceil();

    if f == c {
        sorted_data[k as usize]
    } else {
        let d0 = sorted_data[f as usize] * (c - k);
        let d1 = sorted_data[c as usize] * (k - f);
        d0 + d1
    }
}
