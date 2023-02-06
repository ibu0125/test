fn get_input() ->String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok();
    return name.trim().to_string();
}