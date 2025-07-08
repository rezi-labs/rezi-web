use chrono::{DateTime, Utc};

use crate::routes::Item;

pub fn items_to_events(items: &[Item]) -> String {
    let start_date = now().to_string();
    let end_date: DateTime<Utc> = DateTime::<Utc>::MAX_UTC;
    let end_date = end_date.to_string();

    let beginning = "BEGIN:VCALENDAR";
    let end = "END:VCALENDAR";

    let collected = items
        .iter()
        .map(|item| generate_ical_event(&item.task, &start_date, &end_date))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"{beginning}
{collected}
{end}"#
    )
}

fn generate_ical_event(title: &str, start: &str, end: &str) -> String {
    format!("BEGIN:VEVENT\nSUMMARY:{title}\nDTSTART:{start}\nDTEND:{end}\nEND:VEVENT")
}

fn now() -> DateTime<Utc> {
    Utc::now()
}
