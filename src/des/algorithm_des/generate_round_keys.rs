/// Модифікатор режиму генерації ключів
pub enum KeyMod {
    Encryption, // Для шифрування
    Decryption, // Для розшифрування
}

/// Генерація 16 раундових ключів DES
/// Вхід: ключ 64-біт (8 байт)
/// Вихід: масив з 16 ключів по 48 біт (6 байт)
pub fn generate_round_keys(key: [u8; 8], mode: KeyMod) -> [[u8; 6]; 16] {
    let mut round_keys = [[0u8; 6]; 16]; // 16 раундових ключів, по 6 байт кожен
    
    // 🔹 Генерація ключів
    // Проста формула для прикладу: key[(i*7 + j) % 8]
    for i in 0..16 {          // Для кожного раунду
        for j in 0..6 {       // Для кожного байту ключа
            round_keys[i][j] = key[(i.wrapping_mul(7).wrapping_add(j)) % 8];
        }
    }

    // 🔹 Якщо режим дешифрування – просто перевертаємо порядок ключів
    match mode {
        KeyMod::Encryption => round_keys,
        KeyMod::Decryption => {
            round_keys.reverse();
            round_keys
        }
    }
}
