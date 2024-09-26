use anyhow::Ok;
use chrono::{format::format, Utc};
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

pub fn get_and_create_folder() -> anyhow::Result<String> {
    let folder_name = format!(
        "test/{}",
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    );
    fs::create_dir_all(folder_name.clone())?;
    Ok(folder_name)
}

pub fn draw_all_measurements(
    measurements: &HashMap<IotaTangleNetwork, Measurement>,
) -> anyhow::Result<()> {
    let folder_name = get_and_create_folder().unwrap();

    for (network, durations) in measurements {
        draw_action_measurements(network.name(), durations, folder_name.clone());
    }

    Ok(())
}

pub fn draw_action_measurements(title: &str, measurements: &Measurement, folder_name: String) {
    let mut values: Vec<(String, Vec<f64>)> = Vec::new();

    for action in Action::iter() {
        if let Some(durations) = measurements.get(&action) {
            // Transform the vector of Durations into a vector of f64 seconds
            let secs_f64: Vec<f64> = durations.iter().map(|d| d.as_secs_f64()).collect();
            values.push((action.name().to_string(), secs_f64));
            // info!("Values for {}: {:?}", action.name(), values);
        }
    }

    let _ = draw_box_plot(&folder_name, title, &values);
}

fn draw_box_plot(folder_name: &str, title: &str, values: &Vec<(String, Vec<f64>)>) {
    let plot_title = format!("{}", title);
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(Title::with_text(plot_title).font(Font::new().size(22)))
        .y_axis(
            Axis::new()
                .title(Title::with_text("Time (seconds)").font(Font::new().size(20)))
                .auto_range(true)
                .auto_margin(true)
                .show_grid(true)
                .show_line(true)
                .zero_line(false)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2)
                .tick_font(Font::new().size(17).color("#898989")),
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
                .tick_font(Font::new().size(17).color("#898989")),
        )
        .margin(Margin::new().left(10).right(10).bottom(20).top(50))
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

    let plot_name = format!("{}/{}_boxplot", folder_name, title).replace(" ", "_");
    let plot_name_png = format!("{}.png", plot_name);
    let plot_name_svg = format!("{}.svg", plot_name);
    info!("{}", plot_name);

    plot.write_image(plot_name_png, ImageFormat::PNG, 1100, 400, 1.0);
    plot.write_image(plot_name_svg, ImageFormat::SVG, 1100, 400, 1.0);
}

// fn draw_box_plot2(folder_name: &str, title: &str, values: &Vec<(String, Vec<f64>)>) {
//     let plot_name = format!("{}/{}_boxplot.png", folder_name, title).replace(" ", "_");
//     info!("{}", plot_name);

//     let mut plot = Plot::new();
//     let layout = Layout::new()
//         .y_axis(
//             Axis::new()
//                 .auto_range(true)
//                 .auto_margin(true)
//                 .show_grid(true)
//                 .show_line(true)
//                 .zero_line(false)
//                 .grid_color(Rgb::new(150, 150, 150))
//                 .grid_width(1)
//                 .line_color(Rgb::new(0, 0, 0))
//                 .line_width(2)
//                 .tick_font(Font::new().size(18)),
//         )
//         .x_axis(
//             Axis::new()
//                 .auto_range(true)
//                 .auto_margin(true)
//                 .show_grid(false)
//                 .show_line(true)
//                 .zero_line(false)
//                 .grid_color(Rgb::new(150, 150, 150))
//                 .grid_width(1)
//                 .line_color(Rgb::new(0, 0, 0))
//                 .line_width(2)
//                 .tick_font(Font::new().size(18)),
//         )
//         .margin(Margin::new().left(40).right(30).bottom(80).top(20))
//         .paper_background_color(Rgb::new(243, 243, 243))
//         .plot_background_color(Rgb::new(243, 243, 243))
//         .show_legend(false);
//     plot.set_layout(layout);

//     for (name, durations) in values {
//         let trace = BoxPlot::new(durations.clone())
//             .name(name)
//             .box_points(BoxPoints::All)
//             .jitter(0.5)
//             .whisker_width(0.2)
//             .marker(Marker::new().size(6))
//             .line(Line::new().width(2.0));
//         plot.add_trace(trace);
//     }

//     plot.write_image(plot_name, ImageFormat::PNG, 1100, 600, 1.0);
// }