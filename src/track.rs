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

        return Self::new(name, is_closed, 0.0, n_segments, points);
    }

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

// Support function

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
}
