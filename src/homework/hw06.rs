pub fn draw_tree(levels: usize) {
    let mut tree = String::new();

    let max_width = levels * 2 + 1 + (levels - 1) * 2;

    for level in 0..levels {
        let triangle_height = level + 1;

        for i in 0..triangle_height {
            let stars = 1 + 2 * i;
            let padding = (max_width - stars) / 2;
            let line = " ".repeat(padding) + &"*".repeat(stars) + "\n";
            tree.push_str(&line);
        }
    }

    // Додати верхівку дерева (одинарну зірочку посередині)
    let trunk_padding = (max_width - 1) / 2;
    let trunk_line = format!("{}*\n", " ".repeat(trunk_padding));
    tree = trunk_line.repeat(levels) + &tree;

    print!("{}", tree);
}
