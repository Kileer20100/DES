use std::io;
use colored::*; // Для кольорового тексту у консолі

use crate::input_byte::help::help_text;   // Функція для показу довідки
use crate::input_byte::key::key::read_key; // Функція для введення ключа

// Функція для отримання вводу користувача
pub fn input_text() -> ([u8;8],Vec<Vec<u8>>, u8){
    println!("Please select action"); // Просимо обрати дію
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).expect("Error input message");
    let text = text_input.trim(); // Обрізаємо пробіли/Enter
    let text = text.to_string();  // Перетворюємо у String

    match text.as_str() {
        "1" => { // Шифрування
            let mut text_input_encryption = String::new();
            println!("Input text for encryption");
            io::stdin().read_line(&mut text_input_encryption).expect("Error input message encryption text");
            let text_input = text_input_encryption.trim().to_string();

            let key = read_key(); // Отримуємо ключ від користувача

            let (byte, _) = slice(text_input); // Розбиваємо текст на блоки по 8 байт

            (key, byte, 1) // Повертаємо ключ, байти та номер дії
        }

        "2" => { // Дешифрування
            let mut text_input_encryption = String::new();
            println!("Input HEX text for decryption:");
            io::stdin().read_line(&mut text_input_encryption).expect("Error input message");

            let text_input = text_input_encryption.trim().to_string();

            // 🔹 Конвертуємо HEX рядок у Vec<u8>
            let decoded_bytes: Vec<u8> = match hex_to_bytes(&text_input) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("❌ Invalid HEX input: {}", e);
                    return ([0u8;8], vec![vec![]], 0);
                }
            };

            // 🔹 Перетворюємо байти у String для slice()
            let decoded_text: String = decoded_bytes.iter().map(|&b| b as char).collect();

            let key = read_key(); // Вводимо ключ для дешифрування

            let (byte, _) = slice(decoded_text); // Розбиваємо текст на блоки по 8 байт

            (key, byte, 2)
        }

        "3" => { // Пункт "Допомога"
            help_text(); // Відображаємо довідку
            ([0u8;8],vec![vec![]],3)
        }

        "4" => { // Вихід з програми
            println!("{}", "Вихід".bright_red());
            std::process::exit(0);
        }

        _ => { // Якщо вибір некоректний
            println!("Будь ласка, виберіть один із пунктів");
            ([0u8;8],vec![vec![]],0)
        }
    }
}

// Функція для розбиття тексту на блоки по 8 символів
fn slice(text:String) -> (Vec<Vec<u8>>,Vec<Vec<char>>){
    let mut vec_matrix_output_slice: Vec<Vec<char>> = Vec::new();

    let vec_text_char: Vec<char> = text.chars().collect(); // Розбиваємо на символи
    let length = vec_text_char.len();

    // Кількість блоків по 8 символів
    let mut frequency = (length / 8) as i32;
    if length % 8 != 0 { frequency += 1; }

    for i in 0..frequency {
        let start = (i * 8) as usize;
        let end = ((i + 1) * 8) as usize;

        let slice = if end <= length {
            &vec_text_char[start..end]
        } else {
            &vec_text_char[start..length]
        };

        let mut row: Vec<char> = slice.to_vec();

        // Додаємо пробіли, якщо блок менше 8 символів
        while row.len() < 8 { row.push(' '); }

        vec_matrix_output_slice.push(row);
    }

    // Конвертуємо блоки символів у блоки байтів
    let vec_matrix_output_slice_bytes: Vec<Vec<u8>> =
        vec_matrix_output_slice.iter()
        .map(|row| row.iter().map(|&i| i as u8).collect::<Vec<u8>>())
        .collect();

    (vec_matrix_output_slice_bytes, vec_matrix_output_slice)
}

// Функція для конвертації HEX рядка у Vec<u8>
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string length must be even".into());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex at position {}", i))
        })
        .collect()
}
