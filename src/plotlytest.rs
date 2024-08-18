use plotly::{
    box_plot::BoxPoints,
    color::{Rgb, Rgba},
    common::{Line, Marker},
    layout::{Axis, Layout, Margin},
    BoxPlot, Plot,
};
use rand_distr::{Distribution, Uniform};

pub fn fully_styled_box_plot() {
    let rnd_sample = |num, mul| -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(num);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, mul);
        for _ in 0..num {
            v.push(uniform.sample(&mut rng));
        }
        v
    };

    let x_data = [
        "Carmelo<br>Anthony",
        "Dwyane<br>Wade",
        "Deron<br>Williams",
        "Brook<br>Lopez",
        "Damian<br>Lillard",
        "David<br>West",
        "Blake<br>Griffin",
        "David<br>Lee",
        "Demar<br>Derozan",
    ];
    let y_data = vec![
        rnd_sample(30, 10.0),
        rnd_sample(30, 20.0),
        rnd_sample(30, 25.0),
        rnd_sample(30, 40.0),
        rnd_sample(30, 45.0),
        rnd_sample(30, 30.0),
        rnd_sample(30, 20.0),
        rnd_sample(30, 15.0),
        rnd_sample(30, 43.0),
    ];

    let mut plot = Plot::new();
    let layout = Layout::new()
        .title("Points Scored by the Top 9 Scoring NBA Players in 2012")
        .y_axis(
            Axis::new()
                .auto_range(true)
                .show_grid(true)
                .zero_line(true)
                .dtick(5.0)
                .grid_color(Rgb::new(255, 255, 255))
                .grid_width(1)
                .zero_line_color(Rgb::new(255, 255, 255))
                .zero_line_width(2),
        )
        .margin(Margin::new().left(40).right(30).bottom(80).top(100))
        .paper_background_color(Rgb::new(243, 243, 243))
        .plot_background_color(Rgb::new(243, 243, 243))
        .show_legend(false);
    plot.set_layout(layout);

    for index in 0..x_data.len() {
        let trace = BoxPlot::new(y_data[index].clone())
            .name(x_data[index])
            .box_points(BoxPoints::All)
            .jitter(0.5)
            .whisker_width(0.2)
            .marker(Marker::new().size(6))
            .line(Line::new().width(2.0));
        plot.add_trace(trace);
    }

    plot.show();
}

pub fn box_plot_styling_outliers() {
    let y = vec![
        0.75, 5.25, 5.5, 6.0, 6.2, 6.6, 6.80, 7.0, 7.2, 7.5, 7.5, 7.75, 8.15, 8.15, 8.65, 8.93,
        9.2, 9.5, 10.0, 10.25, 11.5, 12.0, 16.0, 20.90, 22.3, 23.25,
    ];
    let trace1 = BoxPlot::new(y.clone())
        .name("All Points")
        .jitter(0.3)
        .point_pos(-1.8)
        .marker(Marker::new().color(Rgb::new(7, 40, 89)))
        .box_points(BoxPoints::All);
    let trace2 = BoxPlot::new(y.clone())
        .name("Only Whiskers")
        .marker(Marker::new().color(Rgb::new(9, 56, 125)))
        .box_points(BoxPoints::False);
    let trace3 = BoxPlot::new(y.clone())
        .name("Suspected Outlier")
        .marker(
            Marker::new()
                .color(Rgb::new(8, 81, 156))
                .outlier_color(Rgba::new(219, 64, 82, 0.6))
                .line(
                    Line::new()
                        .outlier_color(Rgba::new(219, 64, 82, 1.0))
                        .outlier_width(2),
                ),
        )
        .box_points(BoxPoints::SuspectedOutliers);
    let trace4 = BoxPlot::new(y)
        .name("Whiskers and Outliers")
        .marker(Marker::new().color(Rgb::new(107, 174, 214)))
        .box_points(BoxPoints::Outliers);

    let layout = Layout::new()
        .title("Box Plot Styling Outliers")
        .y_axis(
            Axis::new()
                .auto_range(true)
                .auto_margin(true)
                .show_grid(true)
                .show_line(true)
                .zero_line(false)
                .show_dividers(true)
                // .dtick(5.0)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2),
        )
        .x_axis(
            Axis::new()
                .auto_range(true)
                .auto_margin(true)
                .show_grid(false)
                .show_line(true)
                .zero_line(false)
                .show_dividers(true)
                // .dtick(5.0)
                .grid_color(Rgb::new(150, 150, 150))
                .grid_width(1)
                .line_color(Rgb::new(0, 0, 0))
                .line_width(2),
        );

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    plot.show();
}
