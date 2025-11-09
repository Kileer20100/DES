// ===== ПОЛНЫЙ КОД DES НА RUST =====
// Сгенеровано атоматично
// Всі строчки: 823
// ===================================

// ===== Файл: ./main.rs =====
// ########  ########  ######
// ##     ## ##       ##    ##
// ##     ## ##       ##
// ##     ## ######    ######
// ##     ## ##             ##
// ##     ## ##       ##    ##
// ########  ########  #####
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-//
//  #####   #######  ######   ######   ##  ##    ##  ##  ###  ##           ###  ##   ######  ######    ######   ####     ####             ###  ##   ##  ##             ####    ######   #####      ##    //
// ##   ##   ##  ##   ##  ##   ##  ##  ##  ##    ##  ##   ## ##             ## ##      ##     ##  ##     ##      ##       ##               ## ##    ##  ##            ##  ##      ##   ##   ##    ###    //
// ##        ##       ##  ##   ##  ##  ##  ##    ##  ##   ####              ####       ##     ##  ##     ##      ##       ##               ####     ##  ##                ##     ##    ##  ###     ##    //
//  #####    ####     #####    ##  ##   ####     ##  ##   ###               ###        ##     #####      ##      ##       ##               ###      ######   ######      ##       ##   ## # ##     ##    //
//      ##   ##       ####     ##  ##    ##      ##  ##   ####              ####       ##     ####       ##      ##       ##               ####     ##  ##              ##         ##  ###  ##     ##    //
// ##   ##   ##  ##   ## ##    ##  ##    ##      ##  ##   ## ##             ## ##      ##     ## ##      ##      ## ##    ## ##            ## ##    ##  ##             ##      ##  ##  ##   ##     ##    //
//  #####   #######  ###  ##  ######    ####      ####   ###  ##           ###  ##   ######  ###  ##   ######   ######   ######           ###  ##   ##  ##            ######    ####    #####    ######  //
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-//

// Підключення модулів програми
mod data;          // Модуль з даними (можливо, константи, конфігурації)
mod input_byte;    // Модуль для роботи з введенням тексту та ключів
mod des;           // Основний модуль DES алгоритму
mod title;         // Модуль для відображення заголовку та меню
mod output_text;   // Модуль для виводу результатів шифрування/дешифрування

// Імпортуємо функції та типи для роботи з шифруванням та виводом
use crate::output_text::output::output_text;
use crate::output_text::output::Mode::{Encrypt,Decrypt}; // Enum для режиму роботи (шифрування/дешифрування)
use crate::input_byte::input::input_text;                 // Функція для отримання вводу користувача
use crate::title::title::{help_menu,title_text};         // Функції для заголовку та меню

use crate::des::des_main::{encryption_start,decryption_start}; // Функції шифрування та дешифрування

// Точка входу програми
#[tokio::main]
async fn main() {
    title_text();  // Відображення заголовку програми
    work();        // Запуск основного циклу роботи програми
}

// Основний цикл програми
fn work(){
    loop { // Безкінечний цикл, поки користувач не вийде
        help_menu(); // Відображення головного меню користувача

        // Отримання даних від користувача: ключ, текст, дія
        // key -> [u8;8], text_byte -> Vec<Vec<u8>>, action -> число 1 або 2
        let (key,text_byte,action)= input_text();

        let encrypt;
        let de_encrypt;

        // Обробка дії користувача
        match action {
            1 => { // Якщо обрали шифрування
               encrypt = encryption_start(text_byte, key);  // Шифрування введеного тексту з ключем
               output_text(encrypt,key,Encrypt);             // Вивід результату шифрування
            },
            2 => { // Якщо обрали дешифрування
                de_encrypt = decryption_start(text_byte, key); // Дешифрування введеного тексту з ключем
                output_text(de_encrypt,key,Decrypt);          // Вивід результату дешифрування
            },
            _ => (), // Для інших дій (наприклад, вихід або допомога) нічого не робимо
        }

    }

}

// ===== Конец ./main.rs =====

// ===== Файл: ./data.rs =====
pub struct Data;

impl Data {
pub const START_IP: [u8; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
    57, 49, 41, 33, 25, 17, 9, 1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7
];

    pub const E_EXPANSION: [u8; 48] = [
        32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9,
        8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17,
        16, 17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25,
        24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1
    ];

    pub const FINAL_FP: [u8; 64] = [
        40, 8, 48, 16, 56, 24, 64, 32,
        39, 7, 47, 15, 55, 23, 63, 31,
        38, 6, 46, 14, 54, 22, 62, 30,
        37, 5, 45, 13, 53, 21, 61, 29,
        36, 4, 44, 12, 52, 20, 60, 28,
        35, 3, 43, 11, 51, 19, 59, 27,
        34, 2, 42, 10, 50, 18, 58, 26,
        33, 1, 41, 9, 49, 17, 57, 25
    ];

    pub fn get_s_boxes() -> [[[u8; 16]; 4]; 8] {
        [
            // S1
            [
                [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
                [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
                [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
                [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
            ],
            // S2
            [
                [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
                [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
                [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
                [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
            ],
            // S3
            [
                [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
                [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
                [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
                [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
            ],
            // S4
            [
                [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
                [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
                [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
                [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
            ],
            // S5
            [
                [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
                [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
                [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
                [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
            ],
            // S6
            [
                [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
                [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
                [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
                [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
            ],
            // S7
            [
                [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
                [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
                [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
                [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
            ],
            // S8
            [
                [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
                [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
                [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
                [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
            ],
        ]
    }
    
    pub fn get_p_permutation() -> [u8; 32] {
        [
            16, 7, 20, 21, 29, 12, 28, 17,
            1, 15, 23, 26, 5, 18, 31, 10,
            2, 8, 24, 14, 32, 27, 3, 9,
            19, 13, 30, 6, 22, 11, 4, 25
        ]
    }

}

// ===== Конец ./data.rs =====

// ===== Файл: ./input_byte/mod.rs =====
pub mod key;


pub mod input;
mod help;
// ===== Конец ./input_byte/mod.rs =====

// ===== Файл: ./input_byte/input.rs =====
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

// ===== Конец ./input_byte/input.rs =====

// ===== Файл: ./input_byte/help.rs =====
use std::io::{self, Write};
use colored::*;

pub fn help_text() {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃                  🛈 Допомога                    ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    println!();
    println!("{}", "Ця програма дозволяє шифрувати та розшифровувати текст за допомогою DES.".cyan());
    println!();
    println!("{}", "Як користуватися:".green());
    println!("1. Оберіть пункт меню 1 для шифрування тексту.");
    println!("   - Введіть текст, який хочете зашифрувати (підтримується тільки латиниця).");
    println!("   - Введіть ключ довжиною 8 символів.");
    println!("   - Програма виведе зашифрований текст у HEX-форматі.");
    println!();
    println!("2. Оберіть пункт меню 2 для розшифрування тексту.");
    println!("   - Введіть HEX-код зашифрованого тексту.");
    println!("   - Введіть той самий ключ, який використовувався при шифруванні.");
    println!("   - Програма виведе розшифрований текст.");
    println!();
    println!("3. Оберіть пункт меню 3, щоб переглянути цю довідку.");
    println!("4. Оберіть пункт меню 4, щоб вийти з програми.");
    println!();
    println!("{}", "Поради:".yellow());
    println!(" - Ключ повинен бути рівно 8 символів.");
    println!(" - HEX-текст вводьте без пробілів і додаткових символів.");
    println!(" - Під час шифрування та розшифрування програма автоматично обробляє текст і HEX.");
    println!(" - Програма працює тільки з латиницею (A-Z, a-z).");
    println!();
    println!("{}", "Програму було створено Кирилом Сердюком з групи КН-2301 на мові програмування Rust".magenta());
    println!();


    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{}", "Натисніть Enter, щоб продовжити...".bright_blue());
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

// ===== Конец ./input_byte/help.rs =====

// ===== Файл: ./input_byte/key/mod.rs =====
pub mod key;
// ===== Конец ./input_byte/key/mod.rs =====

// ===== Файл: ./input_byte/key/key.rs =====
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

// ===== Конец ./input_byte/key/key.rs =====

// ===== Файл: ./des/mod.rs =====

pub mod algorithm_des;

pub mod des_main;

// ===== Конец ./des/mod.rs =====

// ===== Файл: ./des/des_main.rs =====
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

// ===== Конец ./des/des_main.rs =====

// ===== Файл: ./des/algorithm_des/mod.rs =====
pub mod e_expansion;
pub mod f_function;
pub mod feistel_round;
pub mod generate_round_keys;
pub mod p_permutation;
pub mod s_boxes;
pub mod xor_xbit;
pub mod permutations;
// ===== Конец ./des/algorithm_des/mod.rs =====

// ===== Файл: ./des/algorithm_des/feistel_round.rs =====
use crate::des::algorithm_des::f_function::f_function;
use crate::des::algorithm_des::xor_xbit::xor_32bit;

/// Один раунд Feistel-мережі DES
/// Вхід: 
/// - `l` - лівий блок 32 біти (4 байти)
/// - `r` - правий блок 32 біти (4 байти)
/// - `round_key` - ключ для цього раунду 48 біт (6 байт)
/// Вихід: кортеж (новий лівий блок, новий правий блок)
pub fn feistel_round(l: [u8; 4], r: [u8; 4], round_key: [u8; 6]) -> ([u8; 4], [u8; 4]) {
    // 🔹 L_i = R_{i-1}  
    // Лівий блок поточного раунду стає правим блоком з попереднього раунду
    let new_l = r;

    // 🔹 R_i = L_{i-1} XOR f(R_{i-1}, K_i)  
    // Функція f() обробляє правий блок з ключем, результат XOR з лівим блоком
    let f_result = f_function(r, round_key);
    let new_r = xor_32bit(l, f_result);

    // Повертаємо нову пару блоків
    (new_l, new_r)
}

// ===== Конец ./des/algorithm_des/feistel_round.rs =====

// ===== Файл: ./des/algorithm_des/generate_round_keys.rs =====
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

// ===== Конец ./des/algorithm_des/generate_round_keys.rs =====

// ===== Файл: ./des/algorithm_des/f_function.rs =====
use crate::des::algorithm_des::e_expansion::e_expansion;
use crate::des::algorithm_des::xor_xbit::xor_48bit;
use crate::des::algorithm_des::s_boxes::s_boxes;
use crate::des::algorithm_des::p_permutation::p_permutation;

/// Функція f для одного раунду DES
/// Вхід:
/// - `r` – правий 32-бітний блок (4 байти)
/// - `round_key` – ключ раунду 48 біт (6 байт)
/// Вихід:
/// - новий 32-бітний блок після перетворень
pub fn f_function(r: [u8; 4], round_key: [u8; 6]) -> [u8; 4] {
    // 🔹 Крок 1: Розширення E
    // 32-бітний правий блок перетворюється на 48 біт
    // Це потрібно, щоб можна було XOR з 48-бітним ключем
    let expanded = e_expansion(r);

    // 🔹 Крок 2: XOR з ключем раунду
    // Після розширення виконуємо побітове XOR з ключем
    let after_xor = xor_48bit(expanded, round_key);

    // 🔹 Крок 3: Проходження через S-блоки
    // 48-бітний блок розбивається на 8 частин по 6 біт
    // Кожна частина проходить через відповідний S-блок → 32 біти
    let after_sboxes = s_boxes(after_xor);

    // 🔹 Крок 4: P-перестановка
    // Перестановка бітів для подальшої дифузії
    let result = p_permutation(after_sboxes);

    // 🔹 Повертаємо 32-бітний блок
    result
}

// ===== Конец ./des/algorithm_des/f_function.rs =====

// ===== Файл: ./des/algorithm_des/e_expansion.rs =====
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

// ===== Конец ./des/algorithm_des/e_expansion.rs =====

// ===== Файл: ./des/algorithm_des/xor_xbit.rs =====
// Функція для XOR двох 32-бітних блоків (4 байти)
// Використовується у Feistel-раунді DES
pub fn xor_32bit(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [
        a[0] ^ b[0], // XOR першого байта
        a[1] ^ b[1], // XOR другого байта
        a[2] ^ b[2], // XOR третього байта
        a[3] ^ b[3], // XOR четвертого байта
    ]
}

// Функція для XOR двох 48-бітних блоків (6 байт)
// Використовується для суміщення розширеного правого блоку з раундовим ключем
pub fn xor_48bit(a: [u8; 6], b: [u8; 6]) -> [u8; 6] {
    [
        a[0] ^ b[0], // XOR першого байта
        a[1] ^ b[1], // XOR другого байта
        a[2] ^ b[2], // XOR третього байта
        a[3] ^ b[3], // XOR четвертого байта
        a[4] ^ b[4], // XOR п’ятого байта
        a[5] ^ b[5], // XOR шостого байта
    ]
}

// ===== Конец ./des/algorithm_des/xor_xbit.rs =====

// ===== Файл: ./des/algorithm_des/s_boxes.rs =====
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

// ===== Конец ./des/algorithm_des/s_boxes.rs =====

// ===== Файл: ./des/algorithm_des/p_permutation.rs =====
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

// ===== Конец ./des/algorithm_des/p_permutation.rs =====

// ===== Файл: ./des/algorithm_des/permutations.rs =====
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

// ===== Конец ./des/algorithm_des/permutations.rs =====

// ===== Файл: ./title/mod.rs =====
pub mod title;
// ===== Конец ./title/mod.rs =====

// ===== Файл: ./title/title.rs =====
use colored::*; // Підключення бібліотеки colored для кольорового тексту у консолі

// Функція для відображення заголовку програми
pub fn title_text(){
    // ASCII-арт заголовку з кольором bright_blue
    println!("{}", r#"          _____                    _____                    _____          "#.bright_blue());
    println!("{}", r#"         /\    \                  /\    \                  /\    \         "#.bright_blue());
    println!("{}", r#"        /::\    \                /::\    \                /::\    \        "#.bright_blue());
    println!("{}", r#"       /::::\    \              /::::\    \              /::::\    \       "#.bright_blue());
    println!("{}", r#"      /::::::\    \            /::::::\    \            /::::::\    \      "#.bright_blue());
    println!("{}", r#"     /:::/\:::\    \          /:::/\:::\    \          /:::/\:::\    \     "#.bright_blue());
    println!("{}", r#"    /:::/  \:::\    \        /:::/__\:::\    \        /:::/__\:::\    \    "#.bright_blue());
    println!("{}", r#"   /:::/    \:::\    \      /::::\   \:::\    \       \:::\   \:::\    \   "#.bright_blue());
    println!("{}", r#"  /:::/    / \:::\    \    /::::::\   \:::\    \    ___\:::\   \:::\    \  "#.bright_blue());
    println!("{}", r#" /:::/    /   \:::\ ___\  /:::/\:::\   \:::\    \  /\   \:::\   \:::\    \ "#.bright_blue());
    println!("{}", r#"/:::/____/     \:::|    |/:::/__\:::\   \:::\____\/::\   \:::\   \:::\____\"#.bright_blue());
    println!("{}", r#"\:::\    \     /:::|____|\:::\   \:::\   \::/    /\:::\   \:::\   \::/    /"#.bright_blue());
    println!("{}", r#" \:::\    \   /:::/    /  \:::\   \:::\   \/____/  \:::\   \:::\   \/____/ "#.bright_blue());
    println!("{}", r#"  \:::\    \ /:::/    /    \:::\   \:::\    \       \:::\   \:::\    \     "#.bright_blue());
    println!("{}", r#"   \:::\    /:::/    /      \:::\   \:::\____\       \:::\   \:::\____\    "#.bright_blue());
    println!("{}", r#"    \:::\  /:::/    /        \:::\   \::/    /        \:::\  /:::/    /    "#.bright_blue());
    println!("{}", r#"     \:::\/:::/    /          \:::\   \/____/          \:::\/:::/    /     "#.bright_blue());
    println!("{}", r#"      \::::::/    /            \:::\    \               \::::::/    /      "#.bright_blue());
    println!("{}", r#"       \::::/    /              \:::\____\               \::::/    /       "#.bright_blue());
    println!("{}", r#"        \::/____/                \::/    /                \::/    /        "#.bright_blue());
    println!("{}", r#"         ~~                       \/____/                  \/____/         "#.bright_blue());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━".bright_blue());

    // Відображення інформації про автора та групу
    println!("┃   {} {}    ┃", "Кирил".bright_blue(), "Сердюк".bright_blue());
    println!("┃      {}      ┃", "КН-2301".bright_green());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━".bright_blue());
}

// Функція для відображення головного меню користувача
pub fn help_menu(){
    println!("┏━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ {}    ┃", "1. 🚀 Шифрування ".green());      // Пункт меню: шифрування
    println!("┃ {} ┃", "2. 🔓 Розшифрування ".yellow());   // Пункт меню: дешифрування
    println!("┃ {}      ┃", "3. 🛈  Допомога ".cyan());      // Пункт меню: довідка/поміч
    println!("┃ {}         ┃", "4. 🚪 Вихід ".red());        // Пункт меню: вихід з програми
    println!("┗━━━━━━━━━━━━━━━━━━━━━━┛");
}

// ===== Конец ./title/title.rs =====

// ===== Файл: ./output_text/mod.rs =====
pub mod output;
// ===== Конец ./output_text/mod.rs =====

// ===== Файл: ./output_text/output.rs =====
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

// ===== Конец ./output_text/output.rs =====

// ===== Файл: ./full_des_code.rs =====

// ===== Конец ./full_des_code.rs =====

