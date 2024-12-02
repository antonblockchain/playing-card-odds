use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split_whitespace().collect::<Vec<_>>();
    let r = parse_input!(inputs[0], usize);
    let s = parse_input!(inputs[1], usize);

    // Создаем векторы для хранения удаленных и запрошенных карт
    let mut removed_cards = Vec::new();
    for _ in 0..r {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        removed_cards.push(input_line.trim_matches('\n').into());
    }

    let mut sought_cards = Vec::new();
    for _ in 0..s {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        sought_cards.push(input_line.trim_matches('\n').into());
    }

    // Создаем карты в виде множества, чтобы избежать дублирования
    use std::collections::HashSet;
    let mut all_possible: HashSet<String> = HashSet::new();

    // Заполняем множество всех возможных карт
    let ranks = vec!["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
    let suits = vec!["C", "D", "H", "S"];

    for rank in &ranks {
        for suit in &suits {
            all_possible.insert(format!("{}{}", rank, suit));
        }
    }

    // Удаляем удаленные карты
    for removed in &removed_cards {
        if suits.contains(removed) {
            // Это масть
            let suit = removed;
            for rank in &ranks {
                all_possible.remove(&format!("{}{}", rank, suit));
            }
        } else if removed.len() == 2 {
            // Это ранг + масть
            let rank = &removed[0..1];
            let suit = &removed[1..2];
            all_possible.remove(&format!("{}{}", rank, suit));
        } else if ranks.contains(removed) {
            // Это только ранг
            let rank = removed;
            for suit in &suits {
                all_possible.remove(&format!("{}{}", rank, suit));
            }
        }
    }

    // Создаем множество всех запрошенных карт
    let mut all_matching: HashSet<String> = HashSet::new();

    for card in &sought_cards {
        if suits.contains(card) {
            // Это масть
            let suit = card;
            for rank in &ranks {
                all_matching.insert(format!("{}{}", rank, suit));
            }
        } else if card.len() == 2 {
            // Это ранг + масть
            let rank = &card[0..1];
            let suit = &card[1..2];
            all_matching.insert(format!("{}{}", rank, suit));
        } else if ranks.contains(card) {
            // Это только ранг
            let rank = card;
            for suit in &suits {
                all_matching.insert(format!("{}{}", rank, suit));
            }
        } else if false {
            // Это '8'
            for rank in &ranks {
                for suit in &suits {
                    all_matching.insert(format!("{}{}", rank, suit));
                }
            }
        }
    }

    // Удаляем пересекающиеся карты
    let intersection: HashSet<String> = all_possible.intersection(&all_matching).cloned().collect();

    // Количество оставшихся нужных карт
    let matching_cards = intersection.len();

    // Размер оставшейся колоды
    let deck_size = all_possible.len();

    // Вычисляем вероятность
    if deck_size == 0 {
        println!("0%");
    } else {
        let percentage = ((matching_cards as f64 / deck_size as f64) * 100.0).round() as i32;
        println!("{}%", percentage);
    }
}
