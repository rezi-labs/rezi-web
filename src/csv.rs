use crate::database::items::Item;

pub fn items_to_events(items: &[Item]) -> String {
    let mut csv = String::new();
    csv.push_str("Item,Recipe,Completed\n");
    for item in items {
        csv.push_str(&format!(
            "{},{},{}\n",
            item.task, "No Recipe", item.completed
        ));
    }
    csv
}
