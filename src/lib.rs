use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlightText(
    sentence_text: String,
    br_word_percentage: f32,
    max_fixation_parts: usize,
    fixation_lower_bound: usize,
) -> Option<String> {
    let mut active = false;
    let mut trailing: usize = 0;
    let mut result: Vec<_> = sentence_text.chars().collect();
    let mut chars = sentence_text.chars().peekable();
    let mut offset = 0;
    for (pos, c) in chars.clone().enumerate() {
        if (!c.is_whitespace() || c != '{' || c != '}') && !active {
            trailing = pos;
            active = true;
        }
        if let Some(v) = chars.peek() {
            if v == &' ' || v == &'{' || v == &'}' && active {
                let w = result[trailing + offset..pos + offset].iter().cloned().collect::<String>();
                let initial_length = result.len();
                parse(
                    w,
                    trailing + offset,
                    br_word_percentage,
                    max_fixation_parts,
                    fixation_lower_bound,
                    &mut result,
                );
                offset += result.len() - initial_length;
                active = false;
            }
        }
        if pos == sentence_text.len() - 1 && (c != ' ' || c == '{' || c == '}') && active {
            let w = result[trailing + offset..pos + offset].iter().cloned().collect::<String>();
            let initial_length = result.len();
            parse(
                w,
                trailing + offset,
                br_word_percentage,
                max_fixation_parts,
                fixation_lower_bound,
                &mut result,
            );
            offset += result.len() - initial_length;
            active = false;
        }
        chars.next();
    }
    Some(result.iter().cloned().collect())
}

#[inline]
fn parse(
    word: String,
    trailing: usize,
    br_word_percentage: f32,
    max_fixation_parts: usize,
    fixation_lower_bound: usize,
    result: &mut Vec<char>,
) {
    let mut ending = 0;
    if !word.is_empty() {
        for (i, cf) in process(
            &word,
            br_word_percentage,
            max_fixation_parts,
            fixation_lower_bound,
        )
        .chars()
        .enumerate()
        {
            if i < word.len() {
                result[trailing + i] = cf;
                ending = i;
            } else {
                ending += 1;
                result.insert(trailing + ending, cf);
            }
        }
    }
}

#[inline]
fn process(
    w: &str,
    br_word_percentage: f32,
    max_fixation_parts: usize,
    fixation_lower_bound: usize,
) -> String {
    let stem_width = if w.len() > 3 {
        (w.len() as f32 * br_word_percentage + 0.5) as usize
    } else {
        w.len()
    };
    let mut first_half: String = w[0..stem_width].to_owned();
    let second_half = if !w[stem_width..].is_empty() {
        format!("<br-edge>{}</br-edge>", &w[stem_width..])
    } else {
        String::from("")
    };
    let max_fixation_parts = if first_half.len() >= max_fixation_parts {
        max_fixation_parts
    } else {
        first_half.len()
    };
    let fixation_width =
        (first_half.len() as f32 * (1.0 / max_fixation_parts as f32)).ceil() as usize;
    if fixation_width == fixation_lower_bound {
        first_half = format!("<br-fixation fixation-strength=\"1\">{first_half}</br-fixation>");
    } else {
        let mut fixation_splits = Vec::with_capacity(max_fixation_parts);
        for part in 0..max_fixation_parts {
            let start_boundry = part * fixation_width;
            let end_boundry = if start_boundry + fixation_width > first_half.len() {
                first_half.len()
            } else {
                start_boundry + fixation_width
            };
            fixation_splits.push(format!(
                "<br-fixation fixation-strength=\"{}\">{}</br-fixation>",
                part + 1,
                &first_half[start_boundry..end_boundry]
            ));
        }
        first_half = fixation_splits.join("");
    }
    format!("<br-bold>{first_half}</br-bold>{second_half}")
}
