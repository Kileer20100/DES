use crate::data::Data;

/// Функція P-перестановки (Permutation) у DES
/// Вхід: 32-бітний блок (4 байти)
/// Вихід: 32-бітний блок після перестановки
pub fn p_permutation(input: [u8; 4]) -> [u8; 4] {
    let mut output = [0u8; 4]; // масив для зберігання результату
    let p_table = Data::get_p_permutation(); // отримуємо таблицю P-перестановки (32 позиції)

    // Проходимо по кожному біту (всього 32 біти)
    for i in 0..32 {
        // Визначаємо позицію біта у вхідному масиві (0-based)
        let original_bit_pos = p_table[i] - 1;
        let original_byte = original_bit_pos / 8;       // байт, де знаходиться біт
        let original_bit = 7 - (original_bit_pos % 8);  // позиція біта у байті (MSB-first)

        // Беремо значення біта
        let bit_value = (input[original_byte as usize] >> original_bit) & 1;

        // Визначаємо позицію біта у вихідному масиві
        let new_byte = i / 8;
        let new_bit = 7 - (i % 8);

        // Записуємо біт на нову позицію
        output[new_byte] |= bit_value << new_bit;
    }

    output
}
