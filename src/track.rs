use maths_toolbox;

pub struct Track {
    // Public
    pub name: String,
    // Private with getters
    is_closed: bool,
    length: f64,
    points: Vec<(f64, f64, f64)>, // (x, y, width)
    // Private without getters
    n_segments: usize,
}

// TODO: Implement segment trait and different types of segments (straight, curve, etc.)
// Could look something like this:
/*
trait Segment {
    fn length(&self) -> f64;
    fn eval(&self, s: f64) -> (f64, f64, f64); // Evaluate at parameter s in [0, 1]
    fn eval_der(&self, s: f64) -> (f64, f64, f64); // Evaluate derivative at s
}

struct CubicBezierSegment {
    p0: (f64, f64, f64),
    p1: (f64, f64, f64),
    p2: (f64, f64, f64),
    p3: (f64, f64, f64),
}
*/

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self.is_closed {
            true => write!(
                f,
                "Closed Track: {} of Length {:.2} m with {} Segments",
                self.name, self.length, self.n_segments
            ),
            false => write!(
                f,
                "Open Track: {} of Length {:.2} m with {} Segments",
                self.name, self.length, self.n_segments
            ),
        };
    }
}

impl Track {
    pub fn new(
        name: String,
        is_closed: bool,
        length: f64,
        n_segments: usize,
        points: Vec<(f64, f64, f64)>,
    ) -> Self {
        Self {
            name,
            is_closed,
            length,
            n_segments,
            points,
        }
    }

    #[allow(dead_code)]
    pub fn straight(length: f64, width: f64) -> Self {
        let points: Vec<(f64, f64, f64)> = vec![
            (0.0, 0.0, width),
            (length / 3.0, 0.0, width),
            (2.0 * length / 3.0, 0.0, width),
            (length, 0.0, width),
        ];

        let name: String = format!("{length:.0} m Straight");
        return Self::new(name, false, length, 1, points);
    }

    #[allow(dead_code)]
    pub fn double_lane_change() -> Self {
        let n_segments: usize = 5;

        let points: Vec<(f64, f64, f64)> = vec![
            (0.0, 0.0, 3.0),
            (2.0, 0.0, 3.0),
            (4.0, 0.0, 3.0),
            (6.0, 0.0, 3.0),
            // Lane change 4.5 meters left over 13.5 meters
            (9.0, 0.0, 3.0),
            (16.5, 4.5, 3.0),
            (19.5, 4.5, 3.0),
            // Straight for 11 meters
            (21.5, 4.5, 3.0),
            (26.5, 4.5, 3.0),
            (30.5, 4.5, 3.0),
            // Lane change 4.5 meters right over 13.5 meters
            (33.5, 4.5, 3.0),
            (41.0, 0.0, 3.0),
            (44.0, 0.0, 3.0),
            // Straight for 6 meters
            (46.0, 0.0, 3.0),
            (48.0, 0.0, 3.0),
            (50.0, 0.0, 3.0),
        ];

        return Self::new(
            "Double Lane Change".to_string(),
            false,
            0.0,
            n_segments,
            points,
        );
    }

    pub fn read_from_file(file_path: &str) -> Self {
        let data: Vec<u8> = match std::fs::read(file_path) {
            Ok(b) => b,
            Err(e) => panic!("Failed to read track file {}: {}", file_path, e),
        };
        let data_len: usize = data.len();
        if data_len < 133 {
            panic!(
                "Invalid track file format: file too short ({} bytes)",
                data_len
            );
        }

        // First four bytes should be the letters "TRKF"
        if &data[0..4] != b"TRKF" {
            panic!("Invalid track file format: missing TRKF header");
        }
        // The next two bytes should be two u8s for major and minor version
        // This is expected to be 0.1 for now
        let major_version: u8 = data[4];
        let minor_version: u8 = data[5];
        if major_version != 0 || minor_version != 1 {
            panic!(
                "Unsupported track file version: {}.{}",
                major_version, minor_version
            );
        }

        // Next 64 bytes are the name of the track as a null-terminated string
        let name_bytes: &[u8] = &data[6..70];
        let mut name_end: usize = 0;
        for (i, &b) in name_bytes.iter().enumerate() {
            if b == 0 {
                name_end = i;
                break;
            }
        }
        if name_end == 0 {
            panic!("Track name is missing");
        }
        let name: String = String::from_utf8(name_bytes[0..name_end].to_vec()).unwrap();

        // The rest of the header is empty for now, so we can skip to the points
        // The next byte is a u8 indicating if the track is closed (0) or open (1)
        let is_closed: bool = match data[128] {
            0 => true,
            1 => false,
            _ => panic!("Invalid value for is_closed in track file"),
        };
        // Next four bytes are a uint32 indicating the number of polynomial segments
        let n_segments: usize =
            u32::from_le_bytes([data[129], data[130], data[131], data[132]]) as usize;
        let n_points: usize = match is_closed {
            true => n_segments * 3,
            false => n_segments * 3 + 1,
        };

        let mut points: Vec<(f64, f64, f64)> = Vec::with_capacity(n_points);

        let mut offset = 133;
        let increment: usize = 24; // Each point is 3 f64s = 24 bytes
        for _ in 0..n_points {
            if offset + increment > data_len {
                panic!("Unexpected end of file while reading track points");
            }

            let x: f64 = f64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);
            let y: f64 = f64::from_le_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]);
            let width: f64 = f64::from_le_bytes([
                data[offset + 16],
                data[offset + 17],
                data[offset + 18],
                data[offset + 19],
                data[offset + 20],
                data[offset + 21],
                data[offset + 22],
                data[offset + 23],
            ]);
            points.push((x, y, width));
            offset += increment;
        }

        let length = calculate_length(&points, n_segments, is_closed);

        return Self::new(name, is_closed, length, n_segments, points);
    }

    // Getters
    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        return self.length;
    }

    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        return self.is_closed;
    }

    #[allow(dead_code)]
    pub fn points(&self) -> Vec<(f64, f64, f64)> {
        return self.points.clone();
    }
}

// Support functions
fn interp_segment(points: &Vec<(f64, f64, f64)>, sq: &Vec<f64>) -> Vec<(f64, f64, f64)> {
    // Validate inputs
    for s in sq {
        assert!(*s >= 0.0 && *s <= 1.0, "s must be in [0, 1]");
    }
    assert!(
        points.len() == 4,
        "points must have exactly 4 control points"
    );

    let mut result: Vec<(f64, f64, f64)> = Vec::with_capacity(sq.len());
    for s in sq {
        let x = points[0].0 * (1.0 - s).powi(3)
            + 3.0 * points[1].0 * s * (1.0 - s).powi(2)
            + 3.0 * points[2].0 * s.powi(2) * (1.0 - s)
            + points[3].0 * s.powi(3);
        let y = points[0].1 * (1.0 - s).powi(3)
            + 3.0 * points[1].1 * s * (1.0 - s).powi(2)
            + 3.0 * points[2].1 * s.powi(2) * (1.0 - s)
            + points[3].1 * s.powi(3);
        let width = points[0].2 * (1.0 - s).powi(3)
            + 3.0 * points[1].2 * s * (1.0 - s).powi(2)
            + 3.0 * points[2].2 * s.powi(2) * (1.0 - s)
            + points[3].2 * s.powi(3);

        result.push((x, y, width));
    }

    return result;
}

fn calculate_length(points: &Vec<(f64, f64, f64)>, n_segments: usize, is_closed: bool) -> f64 {
    // Each cubic segment is defined on the interval [0, 1] in parameter s
    // The length integral kernel is sqrt((dx/ds)^2 + (dy/ds)^2) which is of degree
    // 2 in s, so we can use 2-point Gauss-Legendre quadrature to compute the length exactly
    let lgq_points: Vec<(f64, f64)> = maths_toolbox::glq_interval(0.0, 1.0, 2);
    let mut total_length: f64 = 0.0;

    for k in 0..n_segments {
        let segment_points: Vec<(f64, f64, f64)> = if k == n_segments - 1 && is_closed {
            // Last segment a closed track wraps around to the first point
            vec![
                points[k * 3],
                points[k * 3 + 1],
                points[k * 3 + 2],
                points[0],
            ]
        } else {
            points[k * 3..k * 3 + 4].to_vec()
        };

        let mut segment_length: f64 = 0.0;
        for (s_i, w_i) in &lgq_points {
            // Compute derivatives dx/ds and dy/ds at s_i
            let dx_ds: f64 = -3.0 * segment_points[0].0 * (1.0 - s_i).powi(2)
                + 3.0 * segment_points[1].0 * ((1.0 - s_i).powi(2) - 2.0 * s_i * (1.0 - s_i))
                + 3.0 * segment_points[2].0 * (2.0 * s_i * (1.0 - s_i) - s_i.powi(2))
                + 3.0 * segment_points[3].0 * s_i.powi(2);
            let dy_ds: f64 = -3.0 * segment_points[0].1 * (1.0 - s_i).powi(2)
                + 3.0 * segment_points[1].1 * ((1.0 - s_i).powi(2) - 2.0 * s_i * (1.0 - s_i))
                + 3.0 * segment_points[2].1 * (2.0 * s_i * (1.0 - s_i) - s_i.powi(2))
                + 3.0 * segment_points[3].1 * s_i.powi(2);

            segment_length += w_i * f64::sqrt(dx_ds.powi(2) + dy_ds.powi(2));
        }
        total_length += segment_length;
    }

    return total_length;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_track() {
        let track: Track = Track::straight(120.0, 5.0);
        assert_eq!(track.name, "120 m Straight");
        assert_eq!(track.is_closed(), false);
        assert_eq!(track.length(), 120.0);
        assert_eq!(track.points.len(), 4);
        assert_eq!(track.points[0], (0.0, 0.0, 5.0));
        assert_eq!(track.points[1], (40.0, 0.0, 5.0));
        assert_eq!(track.points[2], (80.0, 0.0, 5.0));
        assert_eq!(track.points[3], (120.0, 0.0, 5.0));
    }

    #[test]
    fn test_interp_straight() {
        let track: Track = Track::straight(120.0, 5.0);
        let points: Vec<(f64, f64, f64)> = track.points();

        let results: Vec<(f64, f64, f64)> = interp_segment(&points[0..4].to_vec(), &vec![0.5]);
        assert!((results[0].0 - 60.0).abs() < 1e-6);
        assert!((results[0].1 - 0.0).abs() < 1e-6);
        assert!((results[0].2 - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_length_calculation() {
        let track: Track = Track::straight(50.0, 5.0);
        let length: f64 = calculate_length(&track.points, track.n_segments, track.is_closed);
        assert!((length - 50.0).abs() < 1e-6);
    }
}
