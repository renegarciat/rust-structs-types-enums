use std::io::Bytes;

enum FileSize {
    Bytes(u64),
    Kibibytes(u64),
    Mebibytes(u64),
    Gibibytes(u64),
}

impl FileSize {
    // Lets use Binary magnitudes instead!
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kibibytes(kb) => format!("{:.2} KiB", *kb as f64 / 1024.0),
            FileSize::Mebibytes(mb) => format!("{:.2} MiB", *mb as f64 / (1024^2) as f64),
            FileSize::Gibibytes(gb) => format!("{:.2} GiB", *gb as f64 / (1024^3) as f64),
        }
    }
}


fn main() {
    let size = 10000000000;
    let filezise = match size {
        0..1024 => FileSize::Bytes(size),
        1024..1048576 => FileSize::Kibibytes(size),
        1048576..1073741824 => FileSize::Mebibytes(size),
        _ => FileSize::Gibibytes(size)
    };
    println!("File size: {}", filezise.format_size());
}
