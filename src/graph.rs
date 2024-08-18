use anyhow::Ok;
use chrono::Utc;
use log::info;
use std::collections::HashMap;
use std::fs;
use strum::IntoEnumIterator;

use plotly::{
    box_plot::BoxPoints,
    color::Rgb,
    common::{Font, Line, Marker, Title},
    layout::{Axis, Layout, Margin},
    BoxPlot, ImageFormat, Plot,
};

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

        let mut values: Vec<(String, Vec<f64>)> = Vec::new();

        for (network, durations) in measurements {
            if let Some(durations) = durations.get(&action) {
                // Transform the vector of Durations into a vector of f64 seconds
                info!("Values: {:?}", durations);
                let secs_f64: Vec<f64> = durations.iter().map(|d| d.as_secs_f64()).collect();
                values.push((network.name().to_string(), secs_f64));
            }
        }

        info!("Values {:?}", values);

        let _ = draw_box_plot(&folder_name, action, &values);
    }

    Ok(())
}

fn draw_box_plot(folder_name: &str, action: Action, values: &Vec<(String, Vec<f64>)>) {
    let plot_name = format!("{}/{}_boxplot.png", folder_name, action.name()).replace(" ", "_");
    info!("{}", plot_name);

    let plot_title = format!("{} (in seconds)", action.name());
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(Title::with_text(plot_title).font(Font::new().size(20)))
        .y_axis(
            Axis::new()
                .auto_range(true)
                .auto_margin(true)
                .show_grid(true)
                .show_line(true)
                .zero_line(false)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2)
                .tick_font(Font::new().size(15)),
        )
        .x_axis(
            Axis::new()
                .auto_range(true)
                .auto_margin(true)
                .show_grid(false)
                .show_line(true)
                .zero_line(false)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2)
                .tick_font(Font::new().size(15)),
        )
        .margin(Margin::new().left(40).right(30).bottom(80).top(100))
        .paper_background_color(Rgb::new(243, 243, 243))
        .plot_background_color(Rgb::new(243, 243, 243))
        .show_legend(false);
    plot.set_layout(layout);

    for (name, durations) in values {
        let trace = BoxPlot::new(durations.clone())
            .name(name)
            .box_points(BoxPoints::All)
            .jitter(0.5)
            .whisker_width(0.2)
            .marker(Marker::new().size(6))
            .line(Line::new().width(2.0));
        plot.add_trace(trace);
    }

    plot.write_image(plot_name, ImageFormat::PNG, 800, 600, 1.0);
}
