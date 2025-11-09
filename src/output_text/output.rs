// Enum для режиму роботи: шифрування або дешифрування
pub enum Mode {
    Encrypt,
    Decrypt,
}

// Функція для перетворення масиву байтів у рядок
// Простіше кажучи, кожен байт перетворюється на char
fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

// Основна функція для виводу результату шифрування/дешифрування
pub fn output_text(text: Vec<Vec<u8>>, key: [u8; 8], modes: Mode) {
    // Конвертуємо ключ у рядок для зручного відображення
    let key_str = bytes_to_string(&key);

    // Вивід ключа залежно від режиму
    match modes {
        Mode::Encrypt => println!("Ключ яким було зашифровано: \"{}\"", key_str),
        Mode::Decrypt => println!("Ключ яким було розшифровано: \"{}\"", key_str),
    }

    // Об'єднуємо всі блоки байтів в один вектор
    let all_bytes: Vec<u8> = text.into_iter().flatten().collect();

    match modes {
        Mode::Encrypt => {
            // 🔹 Конвертація байтів у HEX для зручного виводу
            let hex_text: String = all_bytes.iter()
                .map(|b| format!("{:02X}", b)) // Кожен байт → двозначний HEX
                .collect();
            println!("Зашифрований текст (HEX): \"{}\"", hex_text);
        }

        Mode::Decrypt => {
            // 🔹 Спроба перетворити байти назад у читабельний текст (UTF-8)
            match String::from_utf8(all_bytes.clone()) {
                Ok(decoded_text) => {
                    // Якщо текст валідний UTF-8, виводимо його
                    println!("Розшифрований текст: \"{}\"", decoded_text);
                }
                Err(_) => {
                    // Якщо не вдалось розшифрувати у UTF-8, показуємо байти
                    println!("Розшифрований байтовий текст: {:?}", all_bytes);
                }
            }
        }
    }
}
