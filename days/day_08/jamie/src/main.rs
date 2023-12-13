fn main() {
    let mut lines = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty());
    let instructions = lines
        .next()
        .expect("Could not load instructions")
        .chars()
        .collect::<Vec<char>>();
    let mut nodes = std::collections::HashMap::new();
    for line in lines {
        let key = &line[0..3];
        let l_choice = (&line[7..10]).to_owned();
        let l_key = format!("{}_{}", "L", key);
        nodes.insert(l_key, l_choice);
        let r_choice = (&line[12..15]).to_owned();
        let r_key = format!("{}_{}", "R", key);
        nodes.insert(r_key, r_choice);
    }
    let mut index = 0;
    let mut count = 0;
    let start_node = String::from("AAA");
    let mut current_node: &str = start_node.as_str();
    loop {
        if let Some(instruction) = instructions.get(index) {
            let key = format!("{}_{}", instruction, current_node);
            current_node = nodes
                .get(key.as_str())
                .unwrap_or_else(|| panic!("No entry for {}", key))
                .as_str();

            index += 1;
            count += 1;
            if current_node == "ZZZ" || count > 100000 {
                break;
            }
        } else {
            index = 0;
        }
    }
    println!("count: {}", count);
}
