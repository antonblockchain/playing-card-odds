use std::io;
use std::collections::HashSet;

macro_rules! parse_input {
    ($input:expr, $t:ident) => {
        $input.trim().parse::<$t>().unwrap()
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    // Считываем количество удалённых и искомых карточных описаний
    let inputs = input_line.split_whitespace().collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32);
    let s = parse_input!(inputs[1], i32);

    // Создаём множество для оставшихся карт
    let mut remaining_cards = HashSet::new();

    // Заполняем колоду всеми возможными картами
    let ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let suits = ['C', 'D', 'H', 'S'];

    for &rank in &ranks {
        for &suit in &suits {
            remaining_cards.insert(format!("{}{}", rank, suit));
        }
    }

    // Удаляем указанные карты
    for _ in 0..r {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let removed = input_line.trim().to_string();

        // Обрабатываем удалённые карты
        process_card_description(&removed, &mut remaining_cards);
    }

    // Подсчитываем искомые карты
    let mut sought_count = 0;

    for _ in 0..s {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sought = input_line.trim().to_string();

        // Обрабатываем искомые карты
        sought_count += count_matching_cards(&sought, &remaining_cards);
    }

    // Вычисляем вероятность
    let remaining_count = remaining_cards.len() as f64;
    let odds_percentage = if remaining_count > 0 {
        (sought_count as f64 / remaining_count * 100.0).round() as i32
    } else {
        0 // Если нет оставшихся карт, вероятность 0%
    };

    // Выводим результат
    println!("{}%", odds_percentage);
}

// Функция для обработки описания карт (удаление)
fn process_card_description(description: &str, remaining_cards: &mut HashSet<String>) {
    if description.len() == 1 { // Один ранг
        let rank = description.chars().next().unwrap();
        for suit in ['C', 'D', 'H', 'S'] {
            remaining_cards.remove(&format!("{}{}", rank, suit));
        }
    } else if description.len() == 2 { // Один ранг и одна масть
        remaining_cards.remove(description);
    } else { // Много рангов и мастей
        let ranks: HashSet<_> = description.chars().filter(|c| c.is_digit(10) || "TJQKA".contains(*c)).collect();
        let suits: HashSet<_> = description.chars().filter(|c| "CDHS".contains(*c)).collect();

        for rank in ranks {
            for suit in suits.iter() {
                remaining_cards.remove(&format!("{}{}", rank, suit));
            }
        }
    }
}

// Функция для подсчёта подходящих карт по описанию
fn count_matching_cards(description: &str, remaining_cards: &HashSet<String>) -> usize {
    let mut count = 0;

    if description.len() == 1 { // Один ранг
        let rank = description.chars().next().unwrap();
        for suit in ['C', 'D', 'H', 'S'] {
            if remaining_cards.contains(&format!("{}{}", rank, suit)) {
                count += 1;
            }
        }
    } else if description.len() == 2 { // Один ранг и одна масть
        if remaining_cards.contains(description) {
            count += 1;
        }
    } else { // Много рангов и мастей
        let ranks: HashSet<_> = description.chars().filter(|c| c.is_digit(10) || "TJQKA".contains(*c)).collect();
        let suits: HashSet<_> = description.chars().filter(|c| "CDHS".contains(*c)).collect();

        for rank in ranks {
            for suit in suits.iter() {
                if remaining_cards.contains(&format!("{}{}", rank, suit)) {
                    count += 1;
                }
            }
        }
    }

    count
}