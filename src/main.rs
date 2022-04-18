use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use geo::{point, prelude::*, Point};
use gpx::Waypoint;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    /// The path to the GPX file
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let gpx = gpx::read(reader)?;
    assert_eq!(gpx.tracks.len(), 1);
    let track = &gpx.tracks[0];
    assert_eq!(track.segments.len(), 1);
    let segment = &track.segments[0];

    let mut previous_point: Option<&Waypoint> = None;
    let mut distance = 0.0;
    let mut timespan = Duration::zero();

    for point in &segment.points {
        if let Some(previous_point) = previous_point {
            let p1: Point<f64> = previous_point.as_geopoint();
            let p2: Point<f64> = point.as_geopoint();
            distance += p1.geodesic_distance(&p2);

            let t1 = previous_point.time.expect("GPX point is missing time.");
            let t2 = point.time.expect("GPX point is missing time.");
            timespan = timespan.checked_add(&(t2 - t1)).unwrap();
        }

        println!("{:?}", point);
        println!("Distance: {:?}", distance);
        println!("Time span: {:?}", timespan);

        previous_point = Some(point);
    }

    Ok(())
}

trait IntoGeoPoint {
    fn as_geopoint(&self) -> Point<f64>;
}

impl IntoGeoPoint for &Waypoint {
    fn as_geopoint(&self) -> Point<f64> {
        point!(x: self.point().x(), y: self.point().y())
    }
}
