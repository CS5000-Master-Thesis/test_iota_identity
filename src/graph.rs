use std::collections::HashMap;

use anyhow::Ok;
use plotters::prelude::*;

fn read_data<BR: BufRead>(reader: BR) -> HashMap<(String, String), Vec<f64>> {
    let mut ds = HashMap::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let tuple: Vec<&str> = line.split('\t').collect();
        if tuple.len() == 3 {
            let key = (String::from(tuple[0]), String::from(tuple[1]));
            let entry = ds.entry(key).or_insert_with(Vec::new);
            entry.push(tuple[2].parse::<f64>().unwrap());
        }
    }
    ds
}

pub fn draw_box_plot() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("clustered_bar_chart.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let root = root.margin(5, 5, 5, 5);

    let (upper, lower) = root.split_vertically(512);

    let args: Vec<String> = env::args().collect();

    let ds = if args.len() < 2 {
        read_data(io::Cursor::new(get_data()))
    } else {
        let file = fs::File::open(&args[1])?;
        read_data(BufReader::new(file))
    };
    let dataset: Vec<(String, String, Quartiles)> = ds
        .iter()
        .map(|(k, v)| (k.0.clone(), k.1.clone(), Quartiles::new(v)))
        .collect();

    let host_list: Vec<_> = dataset
        .iter()
        .unique_by(|x| x.0.clone())
        .sorted_by(|a, b| b.2.median().partial_cmp(&a.2.median()).unwrap())
        .map(|x| x.0.clone())
        .collect();

    let mut colors = (0..).map(Palette99::pick);
    let mut offsets = (-12..).step_by(24);
    let mut series = BTreeMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1.clone())
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f32> = dataset.iter().flat_map(|x| x.2.values().to_vec()).collect();
    let values_range = fitting_range(values.iter());

    let mut chart = ChartBuilder::on(&upper)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .caption("Ping Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(
            values_range.start - 1.0..values_range.end + 1.0,
            host_list[..].into_segmented(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Ping, ms")
        .y_desc("Host")
        .y_labels(host_list.len())
        .light_line_style(WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(SegmentValue::CenterOf(&x.0), x.1)
                    .width(20)
                    .whisker_width(0.5)
                    .style(style)
                    .offset(*offset)
            }))?
            .label(label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 6), (x + 12, y + 6)], style.filled()));
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(BLACK.mix(0.5))
        .legend_area_size(22)
        .draw()?;

    let drawing_areas = lower.split_evenly((1, 2));
    let (left, right) = (&drawing_areas[0], &drawing_areas[1]);

    let quartiles_a = Quartiles::new(&[
        6.0, 7.0, 15.9, 36.9, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0,
    ]);
    let quartiles_b = Quartiles::new(&[16.0, 17.0, 50.0, 60.0, 40.2, 41.3, 42.7, 43.3, 47.0]);

    let ab_axis = ["a", "b"];

    let values_range = fitting_range(
        quartiles_a
            .values()
            .iter()
            .chain(quartiles_b.values().iter()),
    );
    let mut chart = ChartBuilder::on(left)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Vertical Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(
            ab_axis[..].into_segmented(),
            values_range.start - 10.0..values_range.end + 10.0,
        )?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;
    chart.draw_series(vec![
        Boxplot::new_vertical(SegmentValue::CenterOf(&"a"), &quartiles_a),
        Boxplot::new_vertical(SegmentValue::CenterOf(&"b"), &quartiles_b),
    ])?;

    let mut chart = ChartBuilder::on(right)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Horizontal Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(-30f32..90f32, 0..3)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;
    chart.draw_series(vec![
        Boxplot::new_horizontal(1, &quartiles_a),
        Boxplot::new_horizontal(2, &Quartiles::new(&[30])),
    ])?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

// #[derive(Debug)]
// struct Measurement {
//     iterations: usize,
//     time_taken: f64,
// }

// fn collect_data() -> Vec<Measurement> {
//     let mut data = Vec::new();

//     // Simulate collecting some data
//     data.push(Measurement {
//         iterations: 100,
//         time_taken: 0.005,
//     });
//     data.push(Measurement {
//         iterations: 100,
//         time_taken: 0.003,
//     });
//     // Add more data as needed...

//     data
// }

// pub fn gen_line_chart() -> anyhow::Result<()> {
//     // let data = collect_data();

//     // // Create a root drawing area
//     // let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
//     // // let root = SVGBackend::new("output.svg", (640, 480)).into_drawing_area();
//     // root.fill(&WHITE).unwrap();

//     // // Set up the chart
//     // let mut chart = ChartBuilder::on(&root)
//     //     .caption("Performance Measurements", ("sans-serif", 50))
//     //     .margin(10)
//     //     .x_label_area_size(30)
//     //     .y_label_area_size(30)
//     //     .build_cartesian_2d(0usize..200usize, 0f64..0.01f64)?;

//     // chart
//     //     .configure_mesh()
//     //     .disable_x_mesh()
//     //     .bold_line_style(WHITE.mix(0.3))
//     //     .y_desc("Average time in seconds")
//     //     .x_desc("Iterations")
//     //     .axis_desc_style(("sans-serif", 15))
//     //     .draw()?;

//     // // Draw lines for different concurrency levels
//     // let concurrency_levels = data.iter().map(|m| m.concurrency_level).collect::<Vec<_>>();
//     // let unique_concurrency_levels: Vec<usize> = concurrency_levels.into_iter().collect();

//     // for &concurrency in &unique_concurrency_levels {
//     //     let filtered_data: Vec<(usize, f64)> = data
//     //         .iter()
//     //         .filter(|m| m.concurrency_level == concurrency)
//     //         .map(|m| (m.iterations, m.time_taken))
//     //         .collect();

//     //     chart
//     //         .draw_series(LineSeries::new(
//     //             filtered_data,
//     //             &Palette99::pick(concurrency),
//     //         ))
//     //         .unwrap()
//     //         .label(format!("Concurrency Level {}", concurrency))
//     //         .legend(move |(x, y)| {
//     //             PathElement::new([(x, y), (x + 20, y)], &Palette99::pick(concurrency))
//     //         });
//     // }

//     // chart
//     //     .configure_series_labels()
//     //     .background_style(&WHITE.mix(0.8))
//     //     .border_style(&BLACK)
//     //     .draw()?;

//     Ok(())
// }

// pub fn test_clustered_bar_chart() -> anyhow::Result<()> {
//     let root = BitMapBackend::new("clustered_bar_chart.png", (640, 480)).into_drawing_area();
//     root.fill(&WHITE)?;

//     let x_labels = ["1", "10", "25", "50"];
//     let laptop_data = [4.5, 6.31, 9.27, 9.82];
//     let desktop_data = [4.83, 5.0, 6.92, 9.31];

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Average runtime in seconds", ("sans-serif", 30))
//         .margin(20.0)
//         .x_label_area_size(40.0)
//         .y_label_area_size(50.0)
//         .build_cartesian_2d(0f64..4f64, 0.0f64..10.0f64)?;

//     chart
//         .configure_mesh()
//         .x_labels(4)
//         // .x_label_formatter(&|x: &f64| x_labels[*x as usize].to_string()) // Pass the closure as a reference
//         .y_desc("Average runtime in seconds")
//         .x_desc("Number of assets")
//         .axis_desc_style(("sans-serif", 15))
//         .draw()?;

//     let bar_width = 0.3;
//     let bar_gap = 0.1;

//     for (i, &_label) in x_labels.iter().enumerate() {
//         chart.draw_series(vec![
//             Rectangle::new(
//                 [
//                     ((i as f64) - bar_width - bar_gap, 0.0),
//                     ((i as f64) - bar_gap, laptop_data[i]),
//                 ],
//                 BLUE.filled(),
//             ),
//             Rectangle::new(
//                 [
//                     ((i as f64) + bar_gap, 0.0),
//                     ((i as f64) + bar_width + bar_gap, desktop_data[i]),
//                 ],
//                 RED.filled(),
//             ),
//         ])?;

//         chart.draw_series(vec![
//             Text::new(
//                 format!("{:.2}", laptop_data[i]),
//                 ((i as f64) - bar_width / 2.0 - bar_gap, laptop_data[i] + 0.2),
//                 ("sans-serif", 15).into_font().color(&BLUE),
//             ),
//             Text::new(
//                 format!("{:.2}", desktop_data[i]),
//                 (
//                     (i as f64) + bar_width / 2.0 + bar_gap,
//                     desktop_data[i] + 0.2,
//                 ),
//                 ("sans-serif", 15).into_font().color(&RED),
//             ),
//         ])?;
//     }

//     chart
//         .configure_series_labels()
//         .border_style(&BLACK)
//         .draw()?;

//     Ok(())
// }
