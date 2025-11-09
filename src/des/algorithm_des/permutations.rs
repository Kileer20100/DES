use crate::data::Data;

/// Початкова перестановка (Initial Permutation, IP) у DES
/// Вхід: 64-бітний блок (8 байт)
/// Вихід: 64-бітний блок після перестановки
pub fn initial_permutation(input: [u8; 8]) -> [u8; 8] {
    let mut output = [0u8; 8]; // Результат перестановки

    for (i, &pos) in Data::START_IP.iter().enumerate() {
        // pos – позиція біта в оригінальному блоці (1-based)
        let src_bit_index = (pos - 1) as usize; // переводимо у 0-based
        let src_byte = src_bit_index / 8;       // номер байта у вхідному масиві
        let src_bit = 7 - (src_bit_index % 8);  // біти нумеруємо від MSB

        let dst_byte = i / 8;        // байт у вихідному масиві
        let dst_bit = 7 - (i % 8);   // позиція біта у байті

        let bit = (input[src_byte] >> src_bit) & 1; // отримуємо потрібний біт
        output[dst_byte] |= bit << dst_bit;         // записуємо його на нову позицію
    }

    output
}

/// Кінцева перестановка (Final Permutation, FP) у DES
/// Це інверсія початкової перестановки
pub fn final_permutation(input: [u8; 8]) -> [u8; 8] {
    let mut output = [0u8; 8];

    for (i, &pos) in Data::FINAL_FP.iter().enumerate() {
        let src_bit_index = (pos - 1) as usize;
        let src_byte = src_bit_index / 8;
        let src_bit = 7 - (src_bit_index % 8);

        let dst_byte = i / 8;
        let dst_bit = 7 - (i % 8);

        let bit = (input[src_byte] >> src_bit) & 1;
        output[dst_byte] |= bit << dst_bit;
    }

    output
}
