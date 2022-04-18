use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use geo::{point, prelude::*, Point};
use gpx::Waypoint;
use hhmmss::Hhmmss;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    /// The length of the segment in meters
    #[clap(short, long)]
    length: f64,

    /// The target speed in km/h
    #[clap(short, long)]
    speed: f64,

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
    let mut best_speed = f64::MAX;
    assert!(segment.points.len() > 0);
    let activity_start = segment.points[0].time.expect("GPX point is missing time.");

    for (i, start_point) in segment.points.iter().enumerate() {
        let start_time = start_point.time.expect("GPX point is missing time.");
        let mut previous_point: Option<&Waypoint> = None;
        let mut distance = 0.0;
        let mut timespan = Duration::zero();

        for point in &segment.points[(i + 1)..] {
            if let Some(previous_point) = previous_point {
                let p1: Point<f64> = previous_point.as_geopoint();
                let p2: Point<f64> = point.as_geopoint();
                distance += p1.geodesic_distance(&p2);

                let t1 = previous_point.time.expect("GPX point is missing time.");
                let t2 = point.time.expect("GPX point is missing time.");
                timespan = timespan.checked_add(&(t2 - t1)).unwrap();
            }

            if distance > args.length {
                let speed = distance / 1000.0 / (timespan.num_seconds() as f64 / 3600.0);

                if speed >= args.speed && speed < best_speed {
                    let time = point.time.expect("GPX point is missing time.");
                    best_speed = speed;

                    println!(
                        "Start time: {:?} ({:?} - {:?}) - Distance: {:?} - Speed: {:?} km/h, {:?} mph",
                        start_time,
                        start_time.signed_duration_since(activity_start).hhmmss(),
                        time.signed_duration_since(activity_start).hhmmss(),
                        distance,
                        speed,
                        speed / 1.60934
                    );
                }

                break;
            }

            previous_point = Some(point);
        }
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
