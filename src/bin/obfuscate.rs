use ::extreme_secret_santa::obf::obfuscate;

fn main() {
    let target = std::env::args().nth(1).expect("no target given");
    let obfuscated = obfuscate(&target);
    println!("{} {}", target, obfuscated);
}
