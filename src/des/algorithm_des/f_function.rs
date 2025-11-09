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
