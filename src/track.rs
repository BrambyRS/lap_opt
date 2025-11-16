use maths_toolbox;

pub struct Track {
    // Public
    pub name: String,
    // Private with getters
    is_closed: bool,
    length: f64,
    // Private without getters
    n_segments: usize,
    segment_lengths: Vec<f64>,
    segments: Vec<Box<CubicBezierSegment>>, // Currently only works with CubicBezierSegment
}

pub struct TrackFrame {
    // Currently only 2D tracks are supported so normal is always (0, 0, 1) and thus omitted
    // Private with getters
    position: (f64, f64),
    tangent: (f64, f64), // Unit vector in "forward" direction
    lateral: (f64, f64), // Unit vector to the left of tangent
    width: f64,
}

pub trait Segment {
    fn calc_length(&self) -> f64;
    fn eval(&self, s: f64) -> (f64, f64, f64); // Evaluate at parameter s in [0, 1]
    fn eval_ds(&self, s: f64) -> (f64, f64, f64); // Evaluate derivative wrt s at s
}

struct CubicBezierSegment {
    // Each of these is a control point (x, y, width)
    p0: (f64, f64, f64),
    p1: (f64, f64, f64),
    p2: (f64, f64, f64),
    p3: (f64, f64, f64),
}

// TRACK IMPLEMENTATION ++++++++++++++++++++++++++++++++
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
        n_segments: usize,
        points: Vec<(f64, f64, f64)>,
    ) -> Self {
        // Divide points into segments
        let mut segments: Vec<Box<CubicBezierSegment>> = Vec::with_capacity(n_segments);
        let mut segment_lengths: Vec<f64> = Vec::with_capacity(n_segments);
        let mut length: f64 = 0.0;
        for i in 0..n_segments {
            let idx_offset: usize = i * 3;

            let p0: (f64, f64, f64) = points[idx_offset];
            let p1: (f64, f64, f64) = points[idx_offset + 1];
            let p2: (f64, f64, f64) = points[idx_offset + 2];
            let p3: (f64, f64, f64) = points[idx_offset + 3];

            let segment: Box<CubicBezierSegment> =
                Box::new(CubicBezierSegment::new(p0, p1, p2, p3));
            segments.push(segment);

            let seg_length: f64 = segments[i].calc_length();
            segment_lengths.push(seg_length);
            length += seg_length;
        }

        Self {
            name,
            is_closed,
            n_segments,
            length,
            segment_lengths,
            segments,
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
        return Self::new(name, false, 1, points);
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

        return Self::new("Double Lane Change".to_string(), false, n_segments, points);
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
        let n_points: usize = n_segments * 3 + 1;

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

        return Self::new(name, is_closed, n_segments, points);
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
}

// TRACKFRAME IMPLEMENTATION +++++++++++++++++++++++++++
impl std::fmt::Display for TrackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: ({:.2}, {:.2})\nTangent: ({:.2}, {:.2})\nLateral: ({:.2}, {:.2})\nWidth: {:.2}",
            self.position.0,
            self.position.1,
            self.tangent.0,
            self.tangent.1,
            self.lateral.0,
            self.lateral.1,
            self.width
        )
    }
}

impl TrackFrame {
    pub fn new(position: (f64, f64), tangent_raw: (f64, f64), width: f64) -> Self {
        // Calculate lateral as a unit vector to the left of tangent under the assumption
        // that the track is in the XY plane

        let tangent_norm: f64 = f64::sqrt(tangent_raw.0.powi(2) + tangent_raw.1.powi(2));
        let tangent: (f64, f64) = (tangent_raw.0 / tangent_norm, tangent_raw.1 / tangent_norm);

        // Cross product [0;0;1]x[tangent.0;tangent.1;0] = [-tangent.1; tangent.0; 0]
        let lateral: (f64, f64) = (-tangent.1, tangent.0);

        return Self {
            position,
            tangent,
            lateral,
            width,
        };
    }

    #[allow(dead_code)]
    pub fn position(&self) -> (f64, f64) {
        return self.position;
    }

    #[allow(dead_code)]
    pub fn tangent(&self) -> (f64, f64) {
        return self.tangent;
    }

    #[allow(dead_code)]
    pub fn lateral(&self) -> (f64, f64) {
        return self.lateral;
    }

    #[allow(dead_code)]
    pub fn width(&self) -> f64 {
        return self.width;
    }
}

// SEGMENT IMPLEMENTATION for CubicBezierSegment +++++++
impl Segment for CubicBezierSegment {
    fn calc_length(&self) -> f64 {
        let lgq_points: Vec<(f64, f64)> = maths_toolbox::glq_interval(0.0, 1.0, 2);
        let mut length: f64 = 0.0;
        for (s_i, w_i) in &lgq_points {
            let (dx_ds, dy_ds, _dwidth_ds) = self.eval_ds(*s_i);
            length += w_i * f64::sqrt(dx_ds.powi(2) + dy_ds.powi(2));
        }
        return length;
    }

    fn eval(&self, s: f64) -> (f64, f64, f64) {
        // Validate s
        assert!(
            s >= 0.0 && s <= 1.0,
            "Parameter s must be in the range [0, 1], got {}",
            s
        );

        let x = self.p0.0 * (1.0 - s).powi(3)
            + 3.0 * self.p1.0 * s * (1.0 - s).powi(2)
            + 3.0 * self.p2.0 * s.powi(2) * (1.0 - s)
            + self.p3.0 * s.powi(3);
        let y = self.p0.1 * (1.0 - s).powi(3)
            + 3.0 * self.p1.1 * s * (1.0 - s).powi(2)
            + 3.0 * self.p2.1 * s.powi(2) * (1.0 - s)
            + self.p3.1 * s.powi(3);
        let width = self.p0.2 * (1.0 - s).powi(3)
            + 3.0 * self.p1.2 * s * (1.0 - s).powi(2)
            + 3.0 * self.p2.2 * s.powi(2) * (1.0 - s)
            + self.p3.2 * s.powi(3);

        return (x, y, width);
    }

    fn eval_ds(&self, s: f64) -> (f64, f64, f64) {
        // Validate s
        assert!(
            s >= 0.0 && s <= 1.0,
            "Parameter s must be in the range [0, 1], got {}",
            s
        );

        let dx_ds = -3.0 * self.p0.0 * (1.0 - s).powi(2)
            + 3.0 * self.p1.0 * ((1.0 - s).powi(2) - 2.0 * s * (1.0 - s))
            + 3.0 * self.p2.0 * (2.0 * s * (1.0 - s) - s.powi(2))
            + 3.0 * self.p3.0 * s.powi(2);
        let dy_ds = -3.0 * self.p0.1 * (1.0 - s).powi(2)
            + 3.0 * self.p1.1 * ((1.0 - s).powi(2) - 2.0 * s * (1.0 - s))
            + 3.0 * self.p2.1 * (2.0 * s * (1.0 - s) - s.powi(2))
            + 3.0 * self.p3.1 * s.powi(2);
        let dwidth_ds = -3.0 * self.p0.2 * (1.0 - s).powi(2)
            + 3.0 * self.p1.2 * ((1.0 - s).powi(2) - 2.0 * s * (1.0 - s))
            + 3.0 * self.p2.2 * (2.0 * s * (1.0 - s) - s.powi(2))
            + 3.0 * self.p3.2 * s.powi(2);

        return (dx_ds, dy_ds, dwidth_ds);
    }
}

// CUBICBEZIERSEGMENT IMPLEMENTATION ++++++++++++++++
impl CubicBezierSegment {
    pub fn new(
        p0: (f64, f64, f64),
        p1: (f64, f64, f64),
        p2: (f64, f64, f64),
        p3: (f64, f64, f64),
    ) -> Self {
        return Self { p0, p1, p2, p3 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TRACK TESTS +++++++++++++++++++++++++++++++++++++
    #[test]
    fn test_straight_track() {
        let track: Track = Track::straight(120.0, 5.0);
        assert_eq!(track.name, "120 m Straight");
        assert_eq!(track.is_closed(), false);
        assert_eq!(track.length(), 120.0);
    }

    // TRACKFRAME TESTS ++++++++++++++++++++++++++++++++
    #[test]
    fn test_trackframe_xdir() {
        // Test with tangent along x-axis
        let position: (f64, f64) = (10.0, 5.0);
        let tangent_raw: (f64, f64) = (3.0, 0.0);
        let width: f64 = 4.0;

        let frame: TrackFrame = TrackFrame::new(position, tangent_raw, width);
        assert!((frame.position.0 - 10.0).abs() < 1e-6);
        assert!((frame.position.1 - 5.0).abs() < 1e-6);
        assert!((frame.tangent.0 - 1.0).abs() < 1e-6);
        assert!((frame.tangent.1 - 0.0).abs() < 1e-6);
        assert!((frame.lateral.0 - 0.0).abs() < 1e-6);
        assert!((frame.lateral.1 - 1.0).abs() < 1e-6);
        assert!((frame.width - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_trackframe_ydir() {
        // Test with tangent along y-axis
        let position: (f64, f64) = (0.0, 0.0);
        let tangent_raw: (f64, f64) = (0.0, -2.0);
        let width: f64 = 2.5;

        let frame: TrackFrame = TrackFrame::new(position, tangent_raw, width);
        assert!((frame.position.0 - 0.0).abs() < 1e-6);
        assert!((frame.position.1 - 0.0).abs() < 1e-6);
        assert!((frame.tangent.0 - 0.0).abs() < 1e-6);
        assert!((frame.tangent.1 + 1.0).abs() < 1e-6);
        assert!((frame.lateral.0 - 1.0).abs() < 1e-6);
        assert!((frame.lateral.1 - 0.0).abs() < 1e-6);
        assert!((frame.width - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_trackframe_diag() {
        // Test with tangent at 45 degrees
        let position: (f64, f64) = (1.0, 1.0);
        let tangent_raw: (f64, f64) = (1.0, 1.0);
        let width: f64 = 3.0;

        let frame: TrackFrame = TrackFrame::new(position, tangent_raw, width);
        let inv_sqrt2: f64 = 1.0 / f64::sqrt(2.0);
        assert!((frame.position.0 - 1.0).abs() < 1e-6);
        assert!((frame.position.1 - 1.0).abs() < 1e-6);
        assert!((frame.tangent.0 - inv_sqrt2).abs() < 1e-6);
        assert!((frame.tangent.1 - inv_sqrt2).abs() < 1e-6);
        assert!((frame.lateral.0 + inv_sqrt2).abs() < 1e-6);
        assert!((frame.lateral.1 - inv_sqrt2).abs() < 1e-6);
        assert!((frame.width - 3.0).abs() < 1e-6);
    }

    // CUBICBEZIERSEGMENT TESTS ++++++++++++++++++++++++
    #[test]
    fn test_cubic_bezier_eval() {
        let segment: CubicBezierSegment = CubicBezierSegment::new(
            (0.0, 0.0, 2.0),
            (1.0, 2.0, 2.5),
            (2.0, 2.0, 3.0),
            (3.0, 0.0, 3.5),
        );

        let (x0, y0, w0) = segment.eval(0.0);
        assert!((x0 - 0.0).abs() < 1e-6);
        assert!((y0 - 0.0).abs() < 1e-6);
        assert!((w0 - 2.0).abs() < 1e-6);

        let (x05, y05, w05) = segment.eval(0.5);
        assert!((x05 - 1.5).abs() < 1e-6);
        assert!((y05 - 1.5).abs() < 1e-6);
        assert!((w05 - 2.75).abs() < 1e-6);

        let (x1, y1, w1) = segment.eval(1.0);
        assert!((x1 - 3.0).abs() < 1e-6);
        assert!((y1 - 0.0).abs() < 1e-6);
        assert!((w1 - 3.5).abs() < 1e-6);
    }

    #[test]
    fn test_cubic_bezier_eval_ds() {
        let segment: CubicBezierSegment = CubicBezierSegment::new(
            (0.0, 0.0, 2.0),
            (1.0, 2.0, 2.5),
            (2.0, 2.0, 3.0),
            (3.0, 0.0, 3.5),
        );

        let (dx0, dy0, dw0) = segment.eval_ds(0.0);
        assert!((dx0 - 3.0).abs() < 1e-6);
        assert!((dy0 - 6.0).abs() < 1e-6);
        assert!((dw0 - 1.5).abs() < 1e-6);

        let (dx05, dy05, dw05) = segment.eval_ds(0.5);
        assert!((dx05 - 3.0).abs() < 1e-6);
        assert!((dy05 - 0.0).abs() < 1e-6);
        assert!((dw05 - 1.5).abs() < 1e-6);

        let (dx1, dy1, dw1) = segment.eval_ds(1.0);
        assert!((dx1 - 3.0).abs() < 1e-6);
        assert!((dy1 + 6.0).abs() < 1e-6);
        assert!((dw1 - 1.5).abs() < 1e-6);
    }
}
