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

    // Определяем ранги и масти
    let ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let suits = ['C', 'D', 'H', 'S'];

    // Заполняем колоду всеми возможными картами
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

        if removed.len() == 1 { // Один ранг
            let rank = removed.chars().next().unwrap();
            for &suit in &suits {
                remaining_cards.remove(&format!("{}{}", rank, suit));
            }
        } else if removed.len() == 2 { // Один ранг и одна масть
            remaining_cards.remove(&removed);
        } else { // Много рангов и мастей
            let ranks_set: HashSet<_> = removed.chars().filter(|c| ranks.contains(c)).collect();
            let suits_set: HashSet<_> = removed.chars().filter(|c| suits.contains(c)).collect();

            for rank in ranks_set {
                for &suit in &suits_set {
                    remaining_cards.remove(&format!("{}{}", rank, suit));
                }
            }
        }
    }

    // Подсчитываем уникальные искомые карты
    let mut sought_cards = HashSet::new();

    for _ in 0..s {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sought = input_line.trim().to_string();

        if sought.len() == 1 { // Один ранг
            let rank = sought.chars().next().unwrap();
            for &suit in &suits {
                sought_cards.insert(format!("{}{}", rank, suit));
            }
        } else if sought.len() == 2 { // Один ранг и одна масть
            sought_cards.insert(sought.clone());
        } else { // Много рангов и мастей
            let ranks_set: HashSet<_> = sought.chars().filter(|c| ranks.contains(c)).collect();
            let suits_set: HashSet<_> = sought.chars().filter(|c| suits.contains(c)).collect();

            for rank in ranks_set {
                for &suit in &suits_set {
                    sought_cards.insert(format!("{}{}", rank, suit));
                }
            }
        }

        // Удаляем карты, которые отсутствуют в оставшихся картах
        sought_cards.retain(|card| remaining_cards.contains(card));
    }

    // Вычисляем вероятность
    let remaining_count = remaining_cards.len(); // Количество оставшихся карт
    let sought_count = sought_cards.len(); // Количество уникальных искомых карт

    let odds_percentage = if remaining_count > 0 { // Использование целых чисел
        sought_count * 100 / remaining_count // Вероятность в процентах
    } else {
        0 // Если нет оставшихся карт, вероятность 0%
    };

    // Выводим результат
    println!("{}%", odds_percentage);
}