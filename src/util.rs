pub fn group_strs_to_fit_width<'a>(
    words: &'a [&'a str],
    width: usize,
    delimiter: &'a str,
) -> Vec<Vec<&'a str>> {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut current_length: usize = 0;
    let mut current_group: Vec<&str> = Vec::new();
    let delimiter_len = delimiter.len();
    for word in words {
        if !current_group.is_empty() && current_length + word.len() > width {
            groups.push(current_group);
            current_group = Vec::new();
            current_length = 0;
        }
        current_length += word.len();
        current_length += delimiter_len;
        current_group.push(word);
    }
    groups.push(current_group);
    groups
}
