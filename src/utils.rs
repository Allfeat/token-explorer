pub const TREASURY_ACCOUNT: &str = "5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z";

pub const SECS_PER_BLOCK: u64 = 12;

pub const DAY: u64 = 86_400;
pub const WEEK: u64 = 7 * DAY;
pub const MONTH: u64 = 30 * DAY;
pub const YEAR: u64 = 12 * MONTH;

pub fn ss58_identicon_svg(address: &str, size: u32) -> String {
    let mut h: u64 = 5381;
    for b in address.as_bytes() {
        h = ((h << 5).wrapping_add(h)).wrapping_add(*b as u64);
    }

    let palette = [
        "#b7ffe1", "#5be0b5", "#1ea87a", // verts
        "#ff9db1", "#ff6b82", "#e0465f", // rosés
        "#9aa4ad", "#d9e1e5", "#4b5563", // gris
    ];

    let cell = 10;
    let pad = 4;
    let g = 5;
    let w = (g * cell + pad * 2) as u32;
    let hsvg = w;

    let c1 = palette[(h as usize) % palette.len()];
    let c2 = palette[((h >> 8) as usize) % palette.len()];
    let bg = "transparent";

    let mut bits = [false; 25];
    let mut x = h;
    for i in 0..(g * ((g + 1) / 2)) {
        x = x.wrapping_mul(1103515245).wrapping_add(12345);
        bits[i] = (x & 1) == 1;
    }
    for row in 0..g {
        for col in 0..g {
            if col >= g - col - 1 {
                continue;
            }
            let left_idx = row * g + col;
            let right_idx = row * g + (g - col - 1);
            bits[right_idx] = bits[left_idx];
        }
    }

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 {w} {hsvg}" role="img" aria-label="identicon">
<rect width="{w}" height="{hsvg}" fill="{bg}"/>"#
    );

    for row in 0..g {
        for col in 0..g {
            let idx = row * g + col;
            if bits[idx] {
                let x = pad + col * cell;
                let y = pad + row * cell;
                let fill = if ((row + col) % 2) == 0 { c1 } else { c2 };
                svg.push_str(&format!(
                    r#"<rect x="{x}" y="{y}" width="{cell}" height="{cell}" rx="2" ry="2" fill="{fill}" opacity="0.9"/>"#
                ));
            }
        }
    }

    svg.push_str(&format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{c1}" opacity="0.2"/>"#,
        cx = w / 2,
        cy = hsvg / 2,
        r = (g * cell / 2) as u32 - 4
    ));

    svg.push_str("</svg>");
    svg
}

pub fn format_balance(amount: u128, with_symbol: bool) -> String {
    let divisor = 10u128.pow(12);
    let mut value = amount as f64 / divisor as f64;

    let suffix = ["", "K", "M", "B", "T", "P"];
    let mut idx = 0;
    while value >= 1000.0 && idx < suffix.len() - 1 {
        value /= 1000.0;
        idx += 1;
    }

    let mut formatted = format!("{:.2}", value);
    if formatted.find('.').is_some() {
        while formatted.ends_with('0') {
            formatted.pop();
        }
        if formatted.ends_with('.') {
            formatted.pop();
        }
    }

    formatted.push_str(suffix[idx]);

    if with_symbol {
        formatted.push(' ');
        formatted.push_str("$AFT");
    }

    formatted
}

pub fn trim_trailing_zero(mut s: String) -> String {
    if s.ends_with(".0") {
        s.truncate(s.len() - 2);
    }
    s
}

pub fn blocks_to_human_duration(blocks: u32) -> String {
    if blocks == 0 {
        return "None".to_string();
    }

    let secs = blocks as u64 * SECS_PER_BLOCK;

    if secs >= YEAR {
        let v = secs as f64 / YEAR as f64;
        return trim_trailing_zero(format!("{:.1} years", v));
    }
    if secs >= MONTH {
        let v = secs as f64 / MONTH as f64;
        return trim_trailing_zero(format!("{:.1} months", v));
    }
    if secs >= WEEK {
        let v = secs as f64 / WEEK as f64;
        return trim_trailing_zero(format!("{:.1} weeks", v));
    }
    if secs >= DAY {
        let v = secs as f64 / DAY as f64;
        return trim_trailing_zero(format!("{:.1} days", v));
    }

    "Less than 1 day".to_string()
}

pub fn blocks_to_str(b: u32) -> String {
    let s = b.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}
