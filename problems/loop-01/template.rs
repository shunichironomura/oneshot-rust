pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    'search: for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == target {
                break 'search Some((r, c));
            }
        }
    }
}

pub fn parse_setting(line: &str) -> Option<(&str, u32)> {
    let Some((key, value)) = line.split_once('=') else {
        None
    };
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    let value = value.trim().parse::<u32>().ok()?;
    Some((key, value))
}

pub fn classify(xs: &[i32]) -> &'static str {
    'check: {
        if xs.is_empty() {
            break "empty";
        }
        for &x in xs {
            if x < 0 {
                break 'check "negative";
            }
        }
        if xs.contains(&0) {
            break 'check "zero";
        }
        "positive"
    }
}
