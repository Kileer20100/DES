use std::io;

// Функція для введення ключа від користувача
// Ключ повинен бути рівно 8 символів
pub fn read_key() -> [u8; 8] {
    let result;

    loop {
        println!("Please input key (8 characters):"); // Просимо ввести ключ
        let mut key_input_encryption = String::new();
        io::stdin()
            .read_line(&mut key_input_encryption)
            .expect("Error reading input"); // Зчитуємо введений рядок

        let key_input = key_input_encryption.trim().to_string(); // Обрізаємо пробіли/Enter

        // Перевіряємо ключ
        let (key, valid) = key_checker(key_input);

        if valid {
            // Якщо ключ правильний – виходимо з циклу
            result = key;
            break;
        } else {
            println!("The key must be exactly 8 characters long."); // Інакше повідомляємо помилку
        }
    }

    result // Повертаємо масив байтів ключа
}

// Функція перевірки ключа
// Повертає масив [u8;8] та булеву змінну valid
fn key_checker(text: String) -> ([u8; 8], bool) {
    let bytes = text.as_bytes(); // Перетворюємо рядок у байти

    if bytes.len() == 8 {
        let mut key = [0u8; 8];
        key.copy_from_slice(bytes); // Копіюємо байти в масив
        (key, true) // Ключ валідний
    } else {
        ([0u8; 8], false) // Ключ невалідний
    }
}
