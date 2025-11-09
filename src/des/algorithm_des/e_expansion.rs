use crate::data::Data;

/// Функція розширення E для правого блоку DES
/// Вхід:
/// - `r` – правий 32-бітний блок (4 байти)
/// Вихід:
/// - 48-бітний блок (6 байт) після розширення
///
/// Розширення E повторює деякі біти, щоб отримати 48-бітний блок для XOR з ключем раунду
pub fn e_expansion(r: [u8; 4]) -> [u8; 6] {
    let mut expanded = [0u8; 6];
    
    for i in 0..48 {
        // Отримуємо позицію біта з таблиці розширення (1-based → 0-based)
        let original_bit_pos = Data::E_EXPANSION[i] - 1; 
        let original_byte = (original_bit_pos / 8) as usize; // який байт
        let original_bit = 7 - (original_bit_pos % 8);       // який біт у байті (MSB = 0)

        // Беремо значення біта з правого блоку
        let bit_value = (r[original_byte] >> original_bit) & 1;

        // Визначаємо, куди записати біт у новому розширеному масиві
        let new_byte = i / 8;
        let new_bit = 7 - (i % 8);
        expanded[new_byte] |= bit_value << new_bit; // записуємо біт
    }
    
    expanded 
}
