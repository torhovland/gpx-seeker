use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use geo::{point, prelude::*, Point};
use gpx::Waypoint;
use hhmmss::Hhmmss;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    /// The distance of the segment in meters
    #[clap(short, long)]
    distance: f64,

    /// The target speed in km/h
    #[clap(short, long)]
    speed: f64,

    /// The path to the GPX file
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct TrackPoint {
    time: DateTime<Utc>,
    accumulated_distance: f64,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let target_distance = args.distance;
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
    let mut accumulated_distance = 0.0;

    let points: Vec<_> = segment
        .points
        .windows(2)
        .map(|pair| {
            let p1 = &pair[0];
            let p2 = &pair[1];
            let distance = p1.as_geopoint().geodesic_distance(&p2.as_geopoint());
            accumulated_distance += distance;
            let time = p1.time.expect("GPX point is missing time.");
            TrackPoint {
                time,
                accumulated_distance,
            }
        })
        .collect();

    for start_point in points.iter() {
        let start_time = start_point.time;

        if let Some(end_point) = points
            .iter()
            .filter(|p| {
                p.accumulated_distance - start_point.accumulated_distance >= target_distance
            })
            .next()
        {
            let distance = end_point.accumulated_distance - start_point.accumulated_distance;
            let timespan = end_point.time - start_point.time;
            let speed = distance / 1000.0 / (timespan.num_seconds() as f64 / 3600.0);

            if speed >= args.speed && speed < best_speed {
                let end_time = end_point.time;
                best_speed = speed;

                println!(
                    "Start time: {:?} ({:?} - {:?}) - Distance: {:?} - Speed: {:?} km/h, {:?} mph",
                    start_time,
                    start_time.signed_duration_since(activity_start).hhmmss(),
                    end_time.signed_duration_since(activity_start).hhmmss(),
                    distance,
                    speed,
                    speed / 1.60934
                );
            }
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
