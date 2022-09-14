use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlightText(
    sentence_text: String,
    br_word_percentage: f32,
    max_fixation_parts: usize,
    fixation_lower_bound: usize,
) -> Option<String> {
    let mut text: Vec<String> = sentence_text.rsplit('{').map(|x| x.to_string()).collect();
    for word in text.iter_mut() {
        let w: Vec<&str> = word.split('}').collect();
        let stem_width = if w.len() > 3 {
            (w.len() as f32 * br_word_percentage + 0.5) as usize
        } else {
            w.len()
        };
        let mut first_half: String = w[0..stem_width].join("");
        let second_half = if w[stem_width..].is_empty() {
            format!("<br-edge>{:?}</br-edge>", &w[stem_width..])
        } else {
            String::from("")
        };
        let max_fixation_parts = if first_half.len() >= max_fixation_parts {
            max_fixation_parts
        } else {
            first_half.len()
        };
        let fixation_width = (first_half.len() as f32 * (1.0 / max_fixation_parts as f32)).ceil() as usize;
        if fixation_width == fixation_lower_bound {
            first_half = format!(
                "<br-fixation fixation-strength=\"1\">{:?}</br-fixation>",
                first_half
            );
        } else {
            let mut fixation_splits = Vec::with_capacity(max_fixation_parts);
            for part in 0..max_fixation_parts {
                // fixation_splits[part] =
                let start_boundry = part * fixation_width;
                let end_boundry = if start_boundry + fixation_width > first_half.len()
                {
                    first_half.len()
                } else {
                    start_boundry + fixation_width
                };
                fixation_splits.push(format!(
                    "<br-fixation fixation-strength=\"{}\">{:?}</br-fixation>",
                    part + 1,
                    &first_half[start_boundry..end_boundry]
                ));
            }
            first_half = fixation_splits.join("");
        }
        *word = format!("<br-bold>{:?}</br-bold>{:?}}}", first_half, second_half);
    }
    Some(text.join("{"))
}
