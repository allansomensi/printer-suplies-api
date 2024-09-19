use chrono::{DateTime, FixedOffset, Utc};
use std::fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

struct UtcFormattedTime;

impl FormatTime for UtcFormattedTime {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        let brasilia_offset = FixedOffset::west_opt(3 * 3600).unwrap();
        let now: DateTime<FixedOffset> = Utc::now().with_timezone(&brasilia_offset);
        let formatted_time = now.format("%d/%m/%Y %H:%M:%S").to_string();
        write!(writer, "{}", formatted_time)
    }
}

pub fn init() {
    tracing_subscriber::fmt()
        .pretty()
        .with_timer(UtcFormattedTime)
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .init();
}
