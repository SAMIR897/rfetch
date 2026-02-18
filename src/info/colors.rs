use colored::*;

pub fn get_color_bar() -> String {
    let colors = [
        "   ".on_black(),
        "   ".on_red(),
        "   ".on_green(),
        "   ".on_yellow(),
        "   ".on_blue(),
        "   ".on_magenta(),
        "   ".on_cyan(),
        "   ".on_white(),
    ];

    colors.iter().map(|c| format!("{}", c)).collect::<String>()
}
