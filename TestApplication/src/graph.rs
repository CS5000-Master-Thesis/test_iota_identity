use anyhow::Ok;
use chrono::{format::format, Utc};
use log::info;
use rayon::vec;
use std::collections::HashMap;
use std::fs;
use strum::IntoEnumIterator;

use plotly::{
    box_plot::BoxPoints,
    color::Rgb,
    common::{Anchor, Font, Line, Marker, Mode, Orientation, Title},
    layout::{
        Annotation, Axis, GridPattern, Layout, LayoutGrid, Legend, Margin, RowOrder, TraceOrder,
    },
    BoxPlot, Configuration, ImageFormat, Plot, Scatter,
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
    folder_name: &str,
    measurements: &HashMap<IotaTangleNetwork, Measurement>,
) -> anyhow::Result<()> {
    for (network, durations) in measurements {
        draw_action_measurements(network.name(), durations, folder_name);
    }

    Ok(())
}

pub fn draw_action_measurements(title: &str, measurements: &Measurement, folder_name: &str) {
    let mut values: Vec<(String, Vec<f64>)> = Vec::new();

    for action in Action::iter() {
        if let Some(durations) = measurements.get(&action) {
            values.push((action.name().to_string(), durations.clone()));
        }
    }

    let _ = draw_box_plot(&folder_name, title, &values);
}

fn draw_box_plot(folder_name: &str, title: &str, values: &Vec<(String, Vec<f64>)>) {
    let plot_title = format!("{}", title);
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(Title::with_text(plot_title).font(Font::new().size(18)))
        .y_axis(
            Axis::new()
                .title(Title::with_text("Time (seconds)").font(Font::new().size(16)))
                .auto_range(true)
                .auto_margin(true)
                .show_grid(true)
                .show_line(true)
                .zero_line(false)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2)
                .tick_font(Font::new().size(15).color("#898989")),
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
                .tick_font(Font::new().size(15).color("#898989")),
        )
        .margin(Margin::new().left(10).right(10).bottom(20).top(50))
        .paper_background_color(Rgb::new(250, 250, 250))
        .plot_background_color(Rgb::new(250, 250, 250))
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

    let plot_name = format!("{}/{}_boxplot", folder_name, title)
        .replace(" ", "_")
        .replace("(", "_")
        .replace(")", "_")
        .replace(":", "_");
    let plot_name_png = format!("{}.png", plot_name);
    let plot_name_svg = format!("{}.svg", plot_name);
    info!("{}", plot_name);

    plot.write_image(plot_name_png, ImageFormat::PNG, 1100, 380, 1.0);
    plot.write_image(plot_name_svg, ImageFormat::SVG, 1100, 380, 1.0);
}

struct RemoteLocalValues {
    pub title: String,
    pub remote: Vec<f64>,
    pub local: Vec<f64>,
}

struct PlotData {
    pub x_values: Vec<f64>,
    pub x_axis_title: String,
    pub y_values: Vec<RemoteLocalValues>,
    pub y_axis_title: String,
}

pub fn line_plot_decline_bps_vs_node_count() {
    let data = PlotData {
        x_axis_title: "Node Count".to_string(),
        x_values: vec![2.0, 3.0, 4.0],
        y_axis_title: "BPS".to_string(),
        y_values: vec![
            RemoteLocalValues {
                title: "Min PoW Score 0".to_string(),
                remote: vec![920.332, 871.325, 832.758],
                local: vec![484.565, 472.579, 449.282],
            },
            RemoteLocalValues {
                title: "Min PoW Score 750".to_string(),
                remote: vec![74.266, 73.859, 72.693],
                local: vec![44.945, 44.222, 43.557],
            },
            RemoteLocalValues {
                title: "Min PoW Score 1500".to_string(),
                remote: vec![26.366, 25.315, 24.762],
                local: vec![15.814, 15.980, 15.884],
            },
        ],
    };

    create_plot(data, "bps_decline_for_different_node_count");
}

pub fn line_plot_decline_bps_vs_min_pow_score() {
    let data = PlotData {
        x_axis_title: "MinPoWScore".to_string(),
        x_values: vec![0.0, 750.0, 1500.0],
        y_axis_title: "BPS".to_string(),
        y_values: vec![
            RemoteLocalValues {
                title: "Nodes = 2".to_string(),
                remote: vec![920.332, 74.266, 26.366],
                local: vec![484.565, 44.945, 15.814],
            },
            RemoteLocalValues {
                title: "Nodes = 3".to_string(),
                remote: vec![871.325, 73.859, 25.315],
                local: vec![472.579, 44.222, 15.980],
            },
            RemoteLocalValues {
                title: "Nodes = 4".to_string(),
                remote: vec![832.758, 72.693, 24.762],
                local: vec![449.282, 43.557, 15.884],
            },
        ],
    };

    create_plot(data, "bps_decline_for_different_pow_scores");
}

fn create_plot(data: PlotData, file_name: &str) {
    let mut plot = Plot::new();

    for (index, trace_data) in data.y_values.iter().enumerate() {
        let show_legend = index < 1;
        let trace_remote = Scatter::new(data.x_values.clone(), trace_data.remote.clone())
            .name("Remote")
            .mode(Mode::LinesMarkers)
            .marker(Marker::new().color("blue"))
            .x_axis(format!("x{}", (index + 1)))
            .y_axis(format!("y{}", (index + 1)))
            .show_legend(show_legend);

        let trace_local = Scatter::new(data.x_values.clone(), trace_data.local.clone())
            .name("Local")
            .mode(Mode::LinesMarkers)
            .marker(Marker::new().color("green"))
            .x_axis(format!("x{}", (index + 1)))
            .y_axis(format!("y{}", (index + 1)))
            .show_legend(show_legend);

        // Add traces to the plot
        plot.add_trace(trace_remote);
        plot.add_trace(trace_local);
    }

    // Create common axis
    let y_axis_template = Axis::new()
        .auto_range(true)
        .auto_margin(true)
        .show_grid(true)
        .show_line(true)
        .zero_line(false)
        .grid_color(Rgb::new(150, 150, 150))
        .grid_width(1)
        .line_color(Rgb::new(0, 0, 0))
        .line_width(2)
        .tick_font(Font::new().size(15).color("#898989"));

    let x_axis_template = Axis::new()
        .tick_values(data.x_values.clone())
        .auto_range(true)
        .auto_margin(true)
        .show_grid(false)
        .show_line(true)
        .zero_line(false)
        .grid_color(Rgb::new(150, 150, 150))
        .grid_width(1)
        .line_color(Rgb::new(0, 0, 0))
        .line_width(2)
        .tick_font(Font::new().size(15).color("#898989"));

    let mut annotaions: Vec<Annotation> = Vec::new();
    for (index, rem_loc_values) in data.y_values.iter().enumerate() {
        let annotaion = plotly::layout::Annotation::new()
            .text(rem_loc_values.title.clone())
            .font(Font::new().size(15))
            .x(0.5)
            .y(1.15)
            .y_ref(format!("y{} domain", (index + 1)))
            .x_ref(format!("x{} domain", (index + 1)))
            .show_arrow(false);
        annotaions.push(annotaion);
    }

    // Define layout with grid (3 plots in a row) and titles for each plot
    let layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(1)
                .columns(3)
                .pattern(plotly::layout::GridPattern::Independent),
        )
        .x_axis(x_axis_template.clone().title(data.x_axis_title.clone()))
        .x_axis2(x_axis_template.clone().title(data.x_axis_title.clone()))
        .x_axis3(x_axis_template.clone().title(data.x_axis_title.clone()))
        .y_axis(y_axis_template.clone().title(data.y_axis_title.clone()))
        .y_axis2(y_axis_template.clone())
        .y_axis3(y_axis_template.clone())
        .annotations(annotaions)
        .show_legend(true)
        .legend(
            plotly::layout::Legend::new()
                .x(1.05) // Position the legend to the right
                .y(1.0)
                .orientation(Orientation::Vertical)
                .font(Font::new().size(15)),
        )
        .width(1200)
        .height(320)
        .margin(Margin::new().left(10).right(10).bottom(35).top(35))
        .paper_background_color(Rgb::new(250, 250, 250))
        .plot_background_color(Rgb::new(250, 250, 250));
    // .title("BPS Decline for Different PoW Scores");

    // Set layout and show plot
    plot.set_layout(layout);
    // plot.show();

    info!("{}", file_name);
    let plot_name_png = format!("{}.png", file_name);
    // let plot_name_svg = format!("{}.svg", file_name);

    plot.write_image(plot_name_png, ImageFormat::PNG, 1200, 250, 1.0);
    // plot.write_image(plot_name_svg, ImageFormat::SVG, 1200, 250, 1.0);
}
