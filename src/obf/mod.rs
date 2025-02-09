const KEY: u8 = 3;

pub fn obfuscate(input: &str) -> String {
    input
        .chars()
        .map(|c| (c as u8 ^ KEY) as char) // XOR each character with the key
        .collect()
}

pub fn deobfuscate(obfuscated: &str) -> String {
    obfuscated
        .chars()
        .map(|c| (c as u8 ^ KEY) as char) // XOR each character with the key to reverse the obfuscation
        .collect()
}
