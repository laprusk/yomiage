use crate::problem::YomiageConfig;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn generate(config: YomiageConfig) -> Vec<i128> {
    let mut problem = vec![0_i128; config.length as usize];
    let mut rng = rand::thread_rng();

    // 桁数パターンを生成
    let digit_pattern = make_digit_pattern(config, &mut rng);
    // 数字を生成
    for i in 0..config.length as usize {
        problem[i] = make_number(digit_pattern[i], &mut rng);
    }
    // 引き算を設定
    set_subtractions(&mut problem, config, &mut rng);
    // 問題をシャッフル
    shuffle_problem(&mut problem, &mut rng);
    // マイナスを除外
    if !config.allow_negative {
        exclude_negative(&mut problem);
    }

    problem
}

fn make_digit_pattern(YomiageConfig { 
    min_digit,
    max_digit,
    length,
    ..
}: YomiageConfig, rng: &mut impl Rng) -> Vec<u32> {
    // min_digitからmax_digitまでの数字をランダムに並べたリストを作成
    let mut digit_list: Vec<u32> = (min_digit..=max_digit).collect();
    digit_list.shuffle(rng);

    // max_digitが0番目、min_digitが1番目に来るようにする
    for i in 0..digit_list.len() {
        if digit_list[i] == max_digit {
            digit_list.swap(0, i);
        }
        if digit_list[i] == min_digit {
            digit_list.swap(1, i);
        }
    }

    let mut digit_pattern = vec![0; length as usize];

    // digit_listを上からdigit_patternに入れていく
    for i in 0..length as usize {
        digit_pattern[i] = digit_list[i % digit_list.len()];
    }

    digit_pattern
}

fn shuffle_number_set(forbidden_start: u32, rng: &mut impl Rng) -> String {
    // 0から9までの数字をランダムに並べたリストを作成
    let mut number_set: Vec<u32> = (0..=9).collect();
    number_set.shuffle(rng);

    // forbidden_startが0番目に来ないようにする
    if number_set[0] == forbidden_start {
        let target: usize = rng.gen_range(1..=9);
        number_set.swap(0, target);
    }

    // 文字列に変換して返す
    number_set.iter().map(|n| n.to_string()).collect()
}

fn make_number(digit: u32, rng: &mut impl Rng) -> i128 {
    let mut number_str = shuffle_number_set(0, rng);
    for _ in 0..((digit - 1) / 10) {
        let last_number = number_str.chars().last().unwrap().to_digit(10).unwrap();
        number_str += &shuffle_number_set(last_number, rng);
    }

    // digit桁目までの数字を取り出してi128に変換して返す
    number_str[..digit as usize].parse().unwrap()
}

fn set_subtractions(
    problem: &mut Vec<i128>,
    YomiageConfig { 
        length,
        subtractions,
        allow_negative,
        ..
    }: YomiageConfig,
    rng: &mut impl Rng) {
    
    problem.shuffle(rng);

    // マイナス除外の場合、問題を半分に分けて前半の合計が後半の合計より小さくなるようにする
    if !allow_negative {
        let sum_former: i128 = problem[..(length / 2) as usize].iter().sum();
        let sum_latter: i128 = problem[(length / 2) as usize..].iter().sum();
        if sum_former > sum_latter {
            for i in 0..(length / 2) as usize {
                problem.swap(i, (length / 2) as usize + i);
            }
        }
    }

    // 残り引き算回数 / 残りの数字の数で引き算を行うかどうかを決定
    let mut remain = subtractions;
    let max_idx = if allow_negative { length as usize } else { (length / 2) as usize };
    for i in 0..max_idx {
        if rng.gen::<f64>() < (remain as f64 / (max_idx - i) as f64) {
            problem[i] *= -1;
            remain -= 1;
        }
    }
}

fn shuffle_problem(problem: &mut Vec<i128>, rng: &mut impl Rng) {
    problem.shuffle(rng);

    // 先頭が引き算になる場合、先頭と0番目以外の数字を入れ替える
    if problem[0] < 0 {
        let mut target: usize = rng.gen_range(1..problem.len());
        while problem[target] < 0 {
            target = (target + 1) % problem.len();
        }
        problem.swap(0, target);
    }
}

fn exclude_negative(problem: &mut Vec<i128>) {
    // 和がマイナスになる場合、
    let mut sum = 0;
    for i in 0..problem.len() {
        if sum + problem[i] < 0 {
            let mut j = i + 1;
            while sum + problem[j] < 0 {
                j = (j + 1) % problem.len();
            }
            problem.swap(i, j);
        }
        sum += problem[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate() {
        let config = YomiageConfig {
            min_digit: 3,
            max_digit: 6,
            length: 10,
            subtractions: 3,
            allow_negative: true,
        };
        let mut problem: Vec<i128>;
        for _ in 0..10 {
            problem = generate(config);
            println!("{:?}", problem);
        }
    }

    #[test]
    fn test_make_digit_pattern() {
        let config = YomiageConfig {
            min_digit: 3,
            max_digit: 6,
            length: 3,
            subtractions: 3,
            allow_negative: true,
        };
        let mut rng = rand::thread_rng();
        let digit_pattern = make_digit_pattern(config, &mut rng);
        println!("{:?}", digit_pattern);
    }

    #[test]
    fn test_shuffle_number_set() {
        let mut rng = rand::thread_rng();
        let number_set = shuffle_number_set(0, &mut rng);
        println!("{:?}", number_set);
    }

    #[test]
    fn test_make_number() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let number = make_number(12, &mut rng);
            println!("{:?}", number);
        }
    }

    #[test]
    fn test_set_subtractions() {
        let problem = vec![1, 2, 9, 10];
        let config = YomiageConfig {
            min_digit: 3,
            max_digit: 6,
            length: 4,
            subtractions: 2,
            allow_negative: false,
        };
        let mut rng = rand::thread_rng();
        let mut problem_copy: Vec<i128>;
        for _ in 0..10 {
            problem_copy = problem.clone();
            set_subtractions(&mut problem_copy, config, &mut rng);
            println!("{:?}", problem_copy);
        }
    }

    #[test]
    fn test_exclude_negative() {
        let mut problem = vec![-1, 2, -3, 4];
        exclude_negative(&mut problem);
        println!("{:?}", problem);
    }
}