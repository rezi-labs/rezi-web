use crate::database::items::Item;
use printpdf::*;
use std::io::BufWriter;

pub fn items_to_pdf(items: &[Item]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Items Export", Mm(210.0), Mm(297.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let mut current_page;
    let mut current_layer_id;

    // Load font
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Title
    current_layer.use_text("Items", 24.0, Mm(20.0), Mm(270.0), &font_bold);
    current_layer.use_text(
        format!(
            "Generated on: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ),
        12.0,
        Mm(20.0),
        Mm(255.0),
        &font,
    );

    let mut y_position = 240.0;

    // Headers
    current_layer.use_text("Item", 14.0, Mm(20.0), Mm(y_position), &font_bold);
    current_layer.use_text("Recipe", 14.0, Mm(110.0), Mm(y_position), &font_bold);
    current_layer.use_text("Status", 14.0, Mm(150.0), Mm(y_position), &font_bold);

    y_position -= 10.0;

    // Draw header line
    let line = Line {
        points: vec![
            (Point::new(Mm(20.0), Mm(y_position)), false),
            (Point::new(Mm(190.0), Mm(y_position)), false),
        ],
        is_closed: false,
    };
    current_layer.add_line(line);

    y_position -= 15.0;

    // Add items
    for item in items {
        if y_position < 30.0 {
            // Add new page if we're running out of space
            let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
            current_page = page;
            current_layer_id = layer;
            current_layer = doc.get_page(current_page).get_layer(current_layer_id);
            y_position = 270.0;
        }

        // Truncate long tasks
        let task = if item.task.len() > 50 {
            format!("{}...", &item.task[..47])
        } else {
            item.task.clone()
        };

        current_layer.use_text(&task, 12.0, Mm(20.0), Mm(y_position), &font);

        current_layer.use_text("No Recipe", 12.0, Mm(110.0), Mm(y_position), &font);

        let status = if item.completed() {
            "✓ Completed"
        } else {
            "○ Pending"
        };
        current_layer.use_text(status, 12.0, Mm(150.0), Mm(y_position), &font);

        y_position -= 12.0;
    }

    // Convert to bytes
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf)?;
    let bytes = buf.into_inner()?;

    Ok(bytes)
}
