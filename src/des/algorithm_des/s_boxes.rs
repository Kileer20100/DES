use crate::data::Data;

// Функція проходить через всі 8 S-блоків DES
// Вхід: 48-бітний блок (6 байт)
// Вихід: 32-бітний блок (4 байти)
pub fn s_boxes(input: [u8; 6]) -> [u8; 4] {
    let mut output = [0u8; 4]; // Результат S-блоків
    let s_boxes_data = Data::get_s_boxes(); // Беремо дані S-блоків (8 блоків по 4x16)

    for i in 0..8 {
        // Кожен S-блок обробляє 6 біт
        let start_bit = i * 6;
        let byte_index = start_bit / 8; // з якого байта починаються біти
        let bit_offset = start_bit % 8; // зміщення бітів у байті

        // Витягуємо 6 біт для поточного S-блоку
        let six_bits = if bit_offset <= 2 {
            // Всі 6 біт знаходяться в одному байті
            (input[byte_index] >> (2 - bit_offset)) & 0x3F
        } else {
            // Біти розділені між двома байтами
            let first_part = (input[byte_index] as u16) << (bit_offset - 2);
            let second_part = (input[byte_index + 1] as u16) >> (10 - bit_offset);
            (first_part | second_part) as u8 & 0x3F
        };

        // Визначаємо рядок і стовпець у S-блоці
        let row = ((six_bits & 0x20) >> 4) | (six_bits & 0x01); // біти 6 і 1 формують рядок (0-3)
        let col = (six_bits >> 1) & 0x0F; // біти 2-5 формують стовпець (0-15)

        // Отримуємо значення з таблиці S-блоку
        let s_value = s_boxes_data[i][row as usize][col as usize];

        // Записуємо 4 біти в результат
        let output_index = i / 2; // кожні два S-блоки формують один байт (4+4 біти)
        let shift = if i % 2 == 0 { 4 } else { 0 }; // старші/молодші 4 біти
        output[output_index] |= s_value << shift;
    }

    output
}
