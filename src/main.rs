fn main() {
    let mut i = 0;
    let mut password: String = "".to_owned();
    const ALPHA: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    while i < 25 {
        password.push_str(ALPHA[i]);
        println!("{}", password);
        i = i + 1;
    }
}