use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const TREASURY_ACCOUNT: &str = "qSwoJVKfgchSRjD6CZ739j9G7zR1khXqkvbeMVCN1NPKJgeup";

pub const SS58_PREFIX: u16 = 440;

pub const SECS_PER_BLOCK: u64 = 6;

pub const DAY: u64 = 86_400;
pub const WEEK: u64 = 7 * DAY;
pub const MONTH: u64 = 30 * DAY;
pub const YEAR: u64 = 12 * MONTH;

/// Known addresses with human-readable names
const KNOWN_ADDRESSES: &[(&str, &str)] = &[
    (TREASURY_ACCOUNT, "Allfeat Foundation"),
];

/// Returns the known name for an address, or None if not known
pub fn get_known_address_name(address: &str) -> Option<&'static str> {
    KNOWN_ADDRESSES
        .iter()
        .find(|(addr, _)| *addr == address)
        .map(|(_, name)| *name)
}

/// Returns either the known name or the address itself
pub fn display_address(address: &str) -> String {
    get_known_address_name(address)
        .map(|name| name.to_string())
        .unwrap_or_else(|| address.to_string())
}

/// Returns the known name with the address as a shortened suffix, or just the address if not known
/// Example: "Allfeat Foundation (qSwo...Jgeup)" or "qSwoJVKf...N1NPKJgeup"
pub fn display_address_with_hint(address: &str) -> String {
    if let Some(name) = get_known_address_name(address) {
        let short = shorten_address(address);
        format!("{} ({})", name, short)
    } else {
        address.to_string()
    }
}

/// Shortens an address to show first 4 and last 5 characters
pub fn shorten_address(address: &str) -> String {
    if address.len() > 12 {
        format!("{}...{}", &address[..4], &address[address.len() - 5..])
    } else {
        address.to_string()
    }
}

/// Generates a unique, stylized SS58 Identicon SVG for a given address.
/// The pattern and colors are heavily seeded by the address content.
pub fn ss58_identicon_svg(address: &str, size: u32) -> String {
    // --- 1. Robust Hashing ---
    let mut hasher = DefaultHasher::new();
    address.hash(&mut hasher);
    let hash_value = hasher.finish();
    let h: u64 = hash_value;

    // --- 2. Palette (Expanded & Organized) ---
    let palette = [
        // Greens
        "#b7ffe1", "#5be0b5", "#1ea87a", "#008f5d", "#33cc99", "#20e6a8",
        // Reds/Pinks
        "#ff9db1", "#ff6b82", "#e0465f", "#d62c4a", "#ff4f70", "#f27991",
        // Grays/Blues
        "#9aa4ad", "#d9e1e5", "#4b5563", "#3a4750", "#7f90a2", "#5e7789",
    ];

    // --- 3. Configuration & ViewBox ---
    let cell = 10;
    let pad = 4;
    let g: usize = 5; // Grid size 5x5
    let w = (g * cell + pad * 2) as u32;
    let hsvg = w;

    let c1_index = (h & 0xFF) as usize % palette.len();
    let c2_index = ((h >> 8) & 0xFF) as usize % palette.len();

    let c1 = palette[c1_index];
    let c2 = if c1_index == c2_index {
        palette[(c2_index + 1) % palette.len()]
    } else {
        palette[c2_index]
    };

    let bg = "transparent";

    // --- 4. Pattern Generation (Clippy Fixes Applied Here) ---

    // Fix: manual_div_ceil: Use `g.div_ceil(2)`
    // G is usize (5), (g + 1) / 2 is manual div_ceil for integer types
    let num_half_cells = g * (g.div_ceil(2_usize));

    let mut bits = [false; 25];
    let mut x = h;

    // Fix: needless_range_loop: Use iterators instead of index `i`
    for bit in bits.iter_mut().take(num_half_cells) {
        // LCG with large prime multiplier/increment
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *bit = (x & 1) == 1;
    }

    // Mirror the pattern for symmetry
    for row in 0..g {
        for col in 0..g {
            if col < g.div_ceil(2_usize) {
                // Fix: manual_div_ceil again
                let left_idx = row * g + col;
                let right_idx = row * g + (g - col - 1);
                bits[right_idx] = bits[left_idx];
            }
        }
    }

    // --- 5. SVG Construction ---
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

                let fill = if idx.is_multiple_of(2) { c1 } else { c2 };

                svg.push_str(&format!(
                    r#"<rect x="{x}" y="{y}" width="{cell}" height="{cell}" rx="1" ry="1" fill="{fill}" opacity="0.9"/>"#
                ));
            }
        }
    }

    // --- 6. The Ring (Substrate-style outer circle) ---
    let cx = w / 2;
    let cy = hsvg / 2;
    let r_inner = (g * cell / 2) as u32 - 2;
    let r_outer = (g * cell / 2) as u32 + 2;

    svg.push_str(&format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{r_inner}" fill="{c1}" opacity="0.15"/>"#
    ));

    svg.push_str(&format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{r_outer}" stroke="{c1}" stroke-width="1.5" fill="none" opacity="0.6"/>"#
    ));

    svg.push_str("</svg>");
    svg
}

/// Formats a large u128 balance amount into a human-readable string (e.g., 1.23M).
///
/// The function assumes the blockchain's base unit has 12 decimal places (10^12).
/// It handles scientific notation for very large numbers using suffixes (K, M, B, etc.)
/// and cleans up trailing zeros for precision.
pub fn format_balance(amount: u128, with_symbol: bool) -> String {
    // Defines the scaling factor: 10^12 (1 trillion)
    const UNIT_DECIMALS: u32 = 12;
    let divisor: f64 = 10.0f64.powi(UNIT_DECIMALS as i32);

    let mut value = amount as f64 / divisor;

    // --- 1. Scientific Suffix Handling (K, M, B, T, P) ---
    let suffixes = ["", "K", "M", "B", "T", "P"];
    let mut suffix_index = 0;

    // Stop at 1000.0 or the end of the suffix array
    while value >= 1000.0 && suffix_index < suffixes.len() - 1 {
        value /= 1000.0;
        suffix_index += 1;
    }

    let suffix = suffixes[suffix_index];

    // --- 2. Decimal Precision and Cleanup ---

    let precision = if value >= 100.0 {
        // High values: 123.4K -> 1 decimal
        1
    } else if value >= 10.0 {
        // Medium values: 12.34K -> 2 decimals
        2
    } else {
        // Low values: 1.234K or small numbers < 1000
        3
    };

    // Format with the determined precision
    let mut formatted = format!("{:.p$}", value, p = precision);

    // Clean trailing zeros and dot: e.g., 1.230 -> 1.23; 1.0 -> 1
    if let Some(dot_pos) = formatted.find('.') {
        // Find the last non-zero digit or the decimal point
        if let Some(last_digit_pos) = formatted.trim_end_matches('0').rfind(|c: char| c != '.') {
            formatted.truncate(last_digit_pos + 1);
        } else {
            // Should only happen if the number is exactly X.000...
            formatted.truncate(dot_pos);
        }
    }

    // --- 3. Final String Assembly ---
    formatted.push_str(suffix);

    if with_symbol {
        formatted.push(' ');
        formatted.push_str("$AFT"); // Use your desired symbol
    }

    formatted
}

pub fn trim_trailing_zero(mut s: String) -> String {
    if s.ends_with(".0") {
        s.truncate(s.len() - 2);
    }
    s
}

/// Converts block count into a human-readable duration string showing the two largest units.
pub fn blocks_to_human_duration(blocks: u32) -> String {
    if blocks == 0 {
        return "None".to_string();
    }

    let secs = blocks as u64 * SECS_PER_BLOCK;

    if secs < DAY {
        // Less than a day: use the smaller units (hours, minutes, or blocks)
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;

        // This part needs specific logic if we want to show exact short durations
        if hours > 0 {
            return format_duration_pair(secs);
        }
        if mins > 0 {
            return format_duration_pair(secs);
        }

        // Default to blocks if less than a minute
        if blocks < 60 / SECS_PER_BLOCK as u32 {
            return format!("{} blocks", blocks);
        }

        format_duration_pair(secs)
    } else {
        // One day or more: use the general two-unit formatting
        format_duration_pair(secs)
    }
}

/// Formats a block number (u32) by adding space separators (e.g., 1000000 -> 1 000 000).
pub fn blocks_to_str(b: u32) -> String {
    let s = b.to_string();
    let mut out = String::with_capacity(s.len() + (s.len() / 3));

    // Iterate characters in reverse
    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(ch);
    }

    // Reverse the result back
    out.chars().rev().collect()
}

// Helper function to format one unit, pluralizing the name
fn format_unit(value: u64, name: &str) -> Option<String> {
    if value > 0 {
        let plural = if value > 1 { "s" } else { "" };
        Some(format!("{} {}{}", value, name, plural))
    } else {
        None
    }
}

/// Formats a duration in seconds into a human-readable string using the two largest units (e.g., "1 year 2 months").
fn format_duration_pair(mut secs: u64) -> String {
    let mut result = Vec::new();

    let units = [
        (YEAR, "year"),
        (MONTH, "month"),
        (WEEK, "week"), // Use weeks instead of days if time is large
        (DAY, "day"),
        (3600, "hour"),
        (60, "minute"),
        // SECS_PER_BLOCK is implicitly handled below
    ];

    for (divisor, name) in units.iter() {
        if secs >= *divisor {
            let count = secs / *divisor;
            secs %= *divisor; // Calculate remainder

            if let Some(s) = format_unit(count, name) {
                result.push(s);
                if result.len() == 2 {
                    break; // Only show two major units for brevity
                }
            }
        }
    }

    // If no major unit was hit (less than a day) and we have remaining seconds
    if result.is_empty() {
        if secs > 0 {
            let blocks = secs / SECS_PER_BLOCK; // Remaining seconds are converted back to blocks
            return format!("{} blocks", blocks);
        } else {
            // Should only happen if input was 0 blocks
            return "None".to_string();
        }
    }

    result.join(" ")
}
