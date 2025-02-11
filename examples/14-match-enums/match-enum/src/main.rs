use std::io::Bytes;

enum FileSize {
    Bytes(u64),
    Kibibytes(u64),
    Mebibytes(u64),
    Gibibytes(u64),
    Tebibytes(u64),
}

const KiB: u64 = 2^10;
const MiB: u64 = 2^20;
const GiB: u64 = 2^30;
const TiB: u64 = 2^40;

impl FileSize {
    // Lets use Binary magnitudes instead!
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kibibytes(kb) => format!("{:.2} KiB", *kb as f64 / KiB as f64),
            FileSize::Mebibytes(mb) => format!("{:.2} MiB", *mb as f64 / MiB as f64),
            FileSize::Gibibytes(gb) => format!("{:.2} GiB", *gb as f64 / GiB as f64),
            FileSize::Tebibytes(tb) => format!("{:.2} TiB", *tb as f64 / TiB as f64),
        }
    }
}


fn main() {
    let size = 100000000;
    let filezise = match size {
        0..KiB => FileSize::Bytes(size),
        KiB..MiB => FileSize::Kibibytes(size),
        MiB..GiB => FileSize::Mebibytes(size),
        GiB..TiB => FileSize::Gibibytes(size),
        _ => FileSize::Tebibytes(size)
    };
    println!("File size: {}", filezise.format_size());
}
