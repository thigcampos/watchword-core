use rand::Rng;

fn main() {
    let mut i = 0;
    const ALPHA: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    const NUMBER: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    pub const VALUE: ([&str; 26], [i8; 10]) = (ALPHA, NUMBER);
    println!("tuple of tuples: {:?}", VALUE);
    let mut password: String = "".to_owned();
    while i < 26 {
        let num = rand::thread_rng().gen_range(0..2);

        println!("{}", VALUE.num);
        password.push_str(ALPHA[i]);
        println!("{}", password);
        i = i + 1;
    }
}