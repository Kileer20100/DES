use crate::data::Data;
/// Модифікатор режиму генерації ключів
pub enum KeyMod {
    Encryption, // Для шифрування
    Decryption, // Для розшифрування
}


/// Генерація 16 раундових ключів DES згідно зі стандартом
/// Вхід: ключ 64-біт (8 байт)
/// Вихід: масив з 16 ключів по 48 біт (6 байт)
pub fn generate_round_keys(key: [u8; 8], mode: KeyMod) -> [[u8; 6]; 16] {
    let mut round_keys = [[0u8; 6]; 16];
    
    // 1. Конвертуємо ключ у бітовий вектор (64 біти)
    let key_bits = bytes_to_bits_64(key);
    
    // 2. Застосовуємо PC1 перестановку (64 -> 56 біт)
    let pc1_bits = permute(&key_bits, &Data::PC1, 64);
    
    // 3. Розділяємо на C0 і D0 (по 28 біт)
    let (mut c, mut d) = split_56_to_28_28(&pc1_bits);
    
    // 4. Генеруємо 16 раундових ключів
    for i in 0..16 {
        // Циклічний зсув вліво
        left_rotate_28(&mut c, Data::SHIFTS[i]);
        left_rotate_28(&mut d, Data::SHIFTS[i]);
        
        // Об'єднуємо C і D (56 біт)
        let cd = combine_28_28_to_56(&c, &d);
        
        // Застосовуємо PC2 перестановку (56 -> 48 біт)
        let key_48_bits = permute(&cd, &Data::PC2, 56);
        
        // Конвертуємо біти назад у байти
        round_keys[i] = bits_to_bytes_48(&key_48_bits);
    }

    // Для дешифрування зворотній порядок ключів
    match mode {
        KeyMod::Encryption => round_keys,
        KeyMod::Decryption => {
            round_keys.reverse();
            round_keys
        }
    }
}

// Допоміжні функції

/// Конвертація 8 байт у 64 біти (MSB-first)
fn bytes_to_bits_64(bytes: [u8; 8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(64);
    for &byte in &bytes {
        for i in 0..8 {
            bits.push((byte >> (7 - i)) & 1);
        }
    }
    bits
}

/// Перестановка бітів згідно з таблицею
fn permute(input: &[u8], table: &[u8], input_size: usize) -> Vec<u8> {
    table.iter()
        .map(|&pos| {
            let bit_pos = (pos - 1) as usize;
            input[bit_pos]
        })
        .collect()
}

/// Розділення 56 біт на два блоки по 28 біт
fn split_56_to_28_28(bits: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let c = bits[0..28].to_vec();
    let d = bits[28..56].to_vec();
    (c, d)
}

/// Об'єднання двох блоків по 28 біт у один 56-бітний
fn combine_28_28_to_56(c: &[u8], d: &[u8]) -> Vec<u8> {
    let mut combined = Vec::with_capacity(56);
    combined.extend_from_slice(c);
    combined.extend_from_slice(d);
    combined
}

/// Циклічний зсув вліво для 28-бітного блоку
fn left_rotate_28(bits: &mut Vec<u8>, shift: u8) {
    let shift_usize = shift as usize;
    let mut rotated = Vec::with_capacity(28);
    
    // Циклічний зсув: беремо елементи з кінця та початку
    rotated.extend_from_slice(&bits[shift_usize..]);
    rotated.extend_from_slice(&bits[0..shift_usize]);
    
    bits.copy_from_slice(&rotated);
}

/// Конвертація 48 біт у 6 байт
fn bits_to_bytes_48(bits: &[u8]) -> [u8; 6] {
    let mut bytes = [0u8; 6];
    for (i, chunk) in bits.chunks(8).enumerate() {
        for (j, &bit) in chunk.iter().enumerate() {
            bytes[i] |= bit << (7 - j);
        }
    }
    bytes
}