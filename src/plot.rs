use std::error::Error;

use plotters::prelude::*;

use crate::reporter::Row;

pub fn generate_plot(
  report_path: &std::path::Path,
  rows: &Vec<Row>,
) -> Result<(), Box<dyn Error>> {
  let output_path = report_path.join("report.png");

  let data_x_axis = rows
    .iter()
    .map(|r| r.time.as_millis())
    .collect::<Vec<u128>>();
  let data_x_max = data_x_axis.iter().max().unwrap().clone();

  let data_cpu = rows
    .iter()
    .map(|r| r.cpu.unwrap() as usize)
    .collect::<Vec<usize>>();
  let data_y_axis_cpu_max = data_cpu.iter().max().unwrap().clone() as u64;
  let data_y_axis_cpu = rows
    .iter()
    .map(|r| (r.time.as_millis(), r.cpu.unwrap()))
    .collect::<Vec<(u128, u64)>>();

  let data_mem = rows
    .iter()
    .map(|r| (r.memory.unwrap() / 1024) as usize)
    .collect::<Vec<usize>>();
  let data_y_axis_mem_max = data_mem.iter().max().unwrap().clone() as u64;
  let data_y_axis_mem = rows
    .iter()
    .map(|r| (r.time.as_millis(), (r.memory.unwrap() / 1024)))
    .collect::<Vec<(u128, u64)>>();
  let root = BitMapBackend::new(&output_path, (1920, 1080)).into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .margin(10)
    .set_label_area_size(LabelAreaPosition::Left, 60)
    .set_label_area_size(LabelAreaPosition::Right, 60)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(0..data_x_max, 0..data_y_axis_cpu_max)?
    .set_secondary_coord(0..data_x_max, 0..data_y_axis_mem_max);

  chart
    .configure_secondary_axes()
    .y_desc("Ram Usage (mb)")
    .y_label_formatter(&|v| format!("{}", v / 1024))
    .draw()?;

  chart
    .configure_mesh()
    .x_desc(format!("Time (ms)"))
    .disable_x_mesh()
    .disable_y_mesh()
    .x_labels(round_up(data_x_max as usize) / 1000)
    .x_label_formatter(&|v| format!("{}", v / 1000))
    .y_label_formatter(&|v| format!("{} %", v))
    .y_desc("CPU Usage")
    .draw()?;

  chart.draw_series(LineSeries::new(data_y_axis_cpu, &BLUE))?;

  if data_x_max > 2000 {
    chart
      .draw_secondary_series(LineSeries::new(data_y_axis_mem, &RED))?
      .label(format!("Time: {:.2} s", data_x_max as f64 / 1000.0));
  } else {
    chart
      .draw_secondary_series(LineSeries::new(data_y_axis_mem, &RED))?
      .label(format!("Time: {} ms", data_x_max));
  }

  chart
    .configure_series_labels()
    .background_style(RGBColor(128, 128, 128))
    .draw()?;

  // To avoid the IO failure being ignored silently, we manually call the present function
  root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
  Ok(())
}

fn round_up(i: usize) -> usize {
  let mut c = i;
  loop {
    if c % 1000 == 0 {
      return c;
    }
    c += 1;
  }
}
