use time::Time;
use time::macros::format_description;

pub struct Meeting {
    pub start: Time,
    pub end: Time,
}

impl Meeting {
    pub fn get_meetings() -> Vec<Meeting> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/meetings.csv");
        let contents = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
        let format = format_description!("[hour]:[minute]");

        contents
            .lines()
            .skip(1)
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mut parts = line.split(',');
                let start = parts.next().expect("missing start column").trim();
                let end = parts.next().expect("missing end column").trim();
                Meeting {
                    start: Time::parse(start, &format).expect("invalid start time"),
                    end: Time::parse(end, &format).expect("invalid end time"),
                }
            })
            .collect()
    }
}