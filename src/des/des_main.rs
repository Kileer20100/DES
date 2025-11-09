use crate::des::algorithm_des::generate_round_keys::KeyMod::{Encryption, Decryption};
use crate::des::algorithm_des::generate_round_keys::generate_round_keys;
use crate::des::algorithm_des::feistel_round::feistel_round;
use crate::des::algorithm_des::permutations::{initial_permutation, final_permutation};

// Функція шифрування тексту DES
pub fn encryption_start(text_byte: Vec<Vec<u8>>, key: [u8; 8]) -> Vec<Vec<u8>> {

    // Генеруємо 16 раундових ключів для шифрування
    let round_keys = generate_round_keys(key, Encryption);
    let mut result = Vec::new();

    // Проходимо по кожному блоку тексту (8 байт)
    for original_block in text_byte {
        // Перетворюємо в масив [u8;8], DES працює тільки з 8-байтовими блоками
        let input_block: [u8; 8] = original_block.try_into().unwrap();

        // Початкова перестановка (Initial Permutation)
        let ip_block = initial_permutation(input_block);

        // Розділяємо блок на ліву і праву частини по 4 байти
        let mut current_l = ip_block[0..4].try_into().unwrap();
        let mut current_r = ip_block[4..8].try_into().unwrap();

        // 16 раундів Feistel
        for round in 0..16 {
            let round_key = round_keys[round];
            (current_l, current_r) = feistel_round(current_l, current_r, round_key);
        }

        // Після останнього раунду міняємо місцями L і R
        std::mem::swap(&mut current_l, &mut current_r);

        // Об'єднуємо блок назад у [u8;8]
        let mut combined = [0u8; 8];
        combined[0..4].copy_from_slice(&current_l);
        combined[4..8].copy_from_slice(&current_r);

        // Застосовуємо фінальну перестановку (Final Permutation)
        let encrypted = final_permutation(combined);

        // Додаємо зашифрований блок у результат
        result.push(encrypted.to_vec());
    }

    result
}

// Функція дешифрування тексту DES
pub fn decryption_start(encrypted_blocks: Vec<Vec<u8>>, key: [u8; 8]) -> Vec<Vec<u8>> {
    // Генеруємо 16 раундових ключів для дешифрування
    let round_keys = generate_round_keys(key, Decryption);
    let mut result = Vec::new();

    // Проходимо по кожному зашифрованому блоку
    for original_block in encrypted_blocks {
        let input_block: [u8; 8] = original_block.try_into().unwrap();

        // Початкова перестановка
        let ip_block = initial_permutation(input_block);

        let mut current_l = ip_block[0..4].try_into().unwrap();
        let mut current_r = ip_block[4..8].try_into().unwrap();

        // 16 раундів Feistel з ключами для дешифрування
        for round in 0..16 {
            let round_key = round_keys[round];
            (current_l, current_r) = feistel_round(current_l, current_r, round_key);
        }

        // Міняємо місцями L і R після останнього раунду
        std::mem::swap(&mut current_l, &mut current_r);

        let mut combined = [0u8; 8];
        combined[0..4].copy_from_slice(&current_l);
        combined[4..8].copy_from_slice(&current_r);

        // Фінальна перестановка
        let decrypted = final_permutation(combined);

        // Додаємо розшифрований блок у результат
        result.push(decrypted.to_vec());
    }

    result
}
