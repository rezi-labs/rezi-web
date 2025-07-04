use chrono::{DateTime, Utc};

use crate::routes::Item;

pub fn items_to_events(items: &[Item]) -> String {
    let start_date = now().to_string();
    let end_date: DateTime<Utc> = DateTime::<Utc>::MAX_UTC;
    let end_date = end_date.to_string();

    let mut csv = String::new();
    csv.push_str("Subject,Start date,Start time\n");
    for item in items {
        csv.push_str(&format!("{},{},{}\n", item.task, &start_date, &end_date));
    }
    csv
}

fn now() -> DateTime<Utc> {
    Utc::now()
}
