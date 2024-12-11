#[cfg(test)]
mod tests {
    use image::{GenericImageView, ImageBuffer, ImageReader};
    use plotters::{
        chart::ChartBuilder,
        prelude::{
            BitMapBackend, IntoDrawingArea, IntoLinspace, IntoSegmentedCoord, PathElement,
            Rectangle, SVGBackend,
        },
        series::{Histogram, LineSeries, SurfaceSeries},
        style::{Color, BLACK, BLUE, GREEN, RED, WHITE},
    };
    use std::collections::HashMap;

    #[test]
    fn copy_image_pixel_one_by_one() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let img = ImageReader::open("./data/scan_raw.jpeg")?.decode()?;
        let (width, height) = img.dimensions();
        let mut imgbuf = ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let rgba = img.get_pixel(width - x - 1, height - y - 1);
            *pixel = rgba;
        }
        imgbuf.save("./data/scan_output.png")?;
        Ok(())
    }

    #[test]
    fn plotters_works() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let area = SVGBackend::new("./data/3d-plot.svg", (1024, 768)).into_drawing_area();

        area.fill(&WHITE)?;

        let x_axis = (-3.0..3.0).step(0.1);
        let y_axis = (-30.0..30.0).step(0.1);
        let z_axis = (-3.0..3.0).step(0.1);

        let mut chart = ChartBuilder::on(&area)
            .caption("3D Plot Test", ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), y_axis, z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-30..30).map(|f| f as f64 / 10.0),
                    (-30..30).map(|f| f as f64 / 10.0),
                    |x, z| (x * x + z * z).cos(),
                )
                .style(BLUE.mix(0.2).filled()),
            )?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .draw_series(LineSeries::new(
                (-100..100)
                    .map(|y| y as f64 / 40.0)
                    .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                &BLACK,
            ))?
            .label("Line")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

        chart.configure_series_labels().border_style(BLACK).draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        log::info!("Result has been saved");
        Ok(())
    }

    #[test]
    fn image_rgb_3d_plot() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let img = ImageReader::open("./data/scan_raw.jpeg")?.decode()?;
        let mut x_y_r = HashMap::new();
        let mut x_y_g = HashMap::new();
        let mut x_y_b = HashMap::new();
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let rgba = img.get_pixel(x, y);
                x_y_r.insert(format!("{}-{}", x, y), rgba.0[0]);
                x_y_g.insert(format!("{}-{}", x, y), rgba.0[1]);
                x_y_b.insert(format!("{}-{}", x, y), rgba.0[2]);
            }
        }

        let area = SVGBackend::new("./data/3d-plot.svg", (1024, 768)).into_drawing_area();

        area.fill(&WHITE)?;

        let x_axis = (0.0..255.0).step(0.1);
        let y_axis = (0.0..255.0).step(0.1);
        let z_axis = (0.0..255.0).step(0.1);

        let mut chart = ChartBuilder::on(&area)
            .caption("3D Plot Test", ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), y_axis, z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (0..255).map(|f| f as f64),
                    (0..255).map(|f| f as f64),
                    |a, b| {
                        let z = x_y_r.get(&format!("{}-{}", a, b)).unwrap_or(&0);
                        *z as f64
                    },
                )
                .style(RED.mix(0.2).filled()),
            )?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });
        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (0..255).map(|f| f as f64),
                    (0..255).map(|f| f as f64),
                    |a, b| {
                        let z = x_y_g.get(&format!("{}-{}", a, b)).unwrap_or(&0);
                        *z as f64
                    },
                )
                .style(GREEN.mix(0.2).filled()),
            )?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (0..255).map(|f| f as f64),
                    (0..255).map(|f| f as f64),
                    |a, b| {
                        let z = x_y_b.get(&format!("{}-{}", a, b)).unwrap_or(&0);
                        *z as f64
                    },
                )
                .style(BLUE.mix(0.2).filled()),
            )?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        log::info!("Result has been saved");
        Ok(())
    }

    #[test]
    fn image_rgb_histogram() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let img = ImageReader::open("./data/scan_raw.jpeg")?.decode()?;
        let mut r_vec = Vec::new();
        let mut g_vec = Vec::new();
        let mut b_vec = Vec::new();
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let rgba = img.get_pixel(x, y);
                r_vec.push(rgba.0[0] as u32);
                g_vec.push(rgba.0[1] as u32);
                b_vec.push(rgba.0[2] as u32);
            }
        }

        let root = BitMapBackend::new("./data/histogram.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Histogram Test", ("sans-serif", 50.0))
            .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..100_000u32)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("RGB")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.1).filled())
                .data(r_vec.into_iter().map(|x: u32| (x, 1))),
        )?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(GREEN.mix(0.1).filled())
                .data(g_vec.into_iter().map(|x: u32| (x, 1))),
        )?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.1).filled())
                .data(b_vec.into_iter().map(|x: u32| (x, 1))),
        )?;

        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        log::info!("Result has been saved");
        Ok(())
    }

    #[test]
    fn fix_image_pixel_one_by_one() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let img = ImageReader::open("./data/scan_raw.jpeg")?.decode()?;
        let (width, height) = img.dimensions();
        let mut imgbuf = ImageBuffer::new(width, height);

        let (start, end) = (125, 255);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut rgba = img.get_pixel(width - x - 1, height - y - 1);
            if (rgba.0[0] >= start && rgba.0[0] <= end)
                || (rgba.0[1] >= start && rgba.0[1] <= end)
                || (rgba.0[2] >= start && rgba.0[2] <= end)
            {
                rgba.0[0] = 255;
                rgba.0[1] = 255;
                rgba.0[2] = 255;
            }
            *pixel = rgba;
        }
        imgbuf.save("./data/scan_output.png")?;
        Ok(())
    }
}
