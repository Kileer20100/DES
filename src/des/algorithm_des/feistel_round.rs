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
