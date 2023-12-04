use std::io::stdin;

fn main() {
    let mut last_numbers = Vec::new();
    let mut last_symbols = Vec::new();
    let mut last_added: Vec<(usize, u32)> = Vec::new();

    let answer: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (numbers, symbols) = gather_data(&line);
            let numbers_form_last_line = get_adjacent_numbers(&last_numbers, &symbols);
            let mut cur = 0;
            let mut sum: u32 = numbers_form_last_line
                .iter()
                .filter_map(|(start, num)| {
                    while cur < last_added.len() && last_added[cur].0 < *start {
                        cur += 1;
                    }

                    if cur >= last_added.len() || last_added[cur].0 != *start {
                        Some(num)
                    } else {
                        None
                    }
                })
                .sum();
            let numbers_due_to_prev_line = get_adjacent_numbers(&numbers, &last_symbols);
            let numbers_due_to_cur_line = get_adjacent_numbers(&numbers, &symbols);

            let mut cur = 0;
            let numbers_due_to_cur_line: Vec<_> = numbers_due_to_cur_line
                .iter()
                .filter_map(|(start, num)| {
                    while cur < numbers_due_to_prev_line.len()
                        && numbers_due_to_prev_line[cur].0 < *start
                    {
                        cur += 1;
                    }

                    if cur >= numbers_due_to_prev_line.len()
                        || numbers_due_to_prev_line[cur].0 != *start
                    {
                        Some((*start, *num))
                    } else {
                        None
                    }
                })
                .collect();

            let mut prev_index = 0;
            let mut cur_index = 0;
            let mut numbers_added = Vec::new();

            while prev_index < numbers_due_to_prev_line.len()
                && cur_index < numbers_due_to_cur_line.len()
            {
                if numbers_due_to_prev_line[prev_index].0 < numbers_due_to_cur_line[cur_index].0 {
                    numbers_added.push(numbers_due_to_prev_line[prev_index]);
                    prev_index += 1;
                } else {
                    numbers_added.push(numbers_due_to_cur_line[cur_index]);
                    cur_index += 1;
                }
            }

            if prev_index < numbers_due_to_prev_line.len() {
                numbers_added.extend_from_slice(&numbers_due_to_prev_line[prev_index..])
            } else {
                numbers_added.extend_from_slice(&numbers_due_to_cur_line[cur_index..])
            }

            sum += numbers_added.iter().map(|(_, num)| num).sum::<u32>();

            last_numbers = numbers;
            last_symbols = symbols;
            last_added = numbers_due_to_prev_line;

            sum
        })
        .sum();

    println!("{answer}");
}

fn gather_data(s: &str) -> (Vec<(usize, usize, u32)>, Vec<usize>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut number = 0;

    s.chars().enumerate().for_each(|(i, c)| {
        if c.is_ascii_digit() {
            if number == 0 {
                numbers.push((i, i, 0));
            }
            number = 10 * number + c.to_digit(10).unwrap();
        } else {
            if number != 0 {
                let last_number = numbers.last_mut().unwrap();
                last_number.1 = i - 1;
                last_number.2 = number;
                number = 0;
            }
            if c != '.' {
                symbols.push(i);
            }
        }
    });

    if number != 0 {
        let last_number = numbers.last_mut().unwrap();
        last_number.1 = s.len() - 1;
        last_number.2 = number;
    }
    (numbers, symbols)
}

fn get_adjacent_numbers(numbers: &[(usize, usize, u32)], symbols: &[usize]) -> Vec<(usize, u32)> {
    if symbols.is_empty() {
        return Vec::new();
    }

    let mut cur_symbol = 0;

    numbers
        .iter()
        .filter_map(|(start, end, number)| {
            while cur_symbol < symbols.len() && symbols[cur_symbol] + 1 < *start {
                cur_symbol += 1;
            }

            if cur_symbol < symbols.len() && symbols[cur_symbol] <= *end + 1 {
                Some((*start, *number))
            } else {
                None
            }
        })
        .collect()
}
