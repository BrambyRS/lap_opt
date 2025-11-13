use std::fs::File;

pub struct Track {
    // Public
    pub name: String,
    // Private
    is_closed: bool,
    length: f64,
    points: Vec<(f64, f64)>,
}

impl Track {
    pub fn new(name: String, is_closed: bool, length: f64, points: Vec<(f64, f64)>) -> Self {
        Self {
            name,
            is_closed,
            length,
            points,
        }
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
        let num_segments: u32 = u32::from_le_bytes([data[129], data[130], data[131], data[132]]);
        let num_points: usize = (num_segments as usize) * 3;

        let mut points: Vec<(f64, f64)> = Vec::with_capacity(num_points);

        let mut offset = 133;
        for _ in 0..num_points {
            if offset + 16 > data_len {
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
            points.push((x, y));
            offset += 16;
        }

        return Self::new(name, is_closed, 0.0, points);
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        self.length
    }

    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    #[allow(dead_code)]
    pub fn points(&self) -> &Vec<(f64, f64)> {
        &self.points
    }
}
