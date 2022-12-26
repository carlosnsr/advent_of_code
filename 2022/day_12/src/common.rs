pub type Height = char;

pub fn height(value: Height) -> usize {
    match value {
        'S' => 0, // should match the height of 'a'
        'E' => 'z' as usize - 'a' as usize, // should match the height of 'z'
        _ => value as usize - 'a' as usize,
    }
}
