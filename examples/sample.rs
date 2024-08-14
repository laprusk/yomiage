use yomiage;

fn main() {
    let config = yomiage::Config {
        min_digit: 3,
        max_digit: 6,
        length: 10,
        subtractions: 3,
        allow_negative: false,
    };
    let yp = yomiage::Problem::new(config).unwrap();

    println!("Problem: {:?}", yp.problem);
    // Problem: [382, 8054, 920, 2893, 394815, -70392, -154, 284396, -476985, 38516]
    println!("Answer: {}", yp.answer);
    // Answer: 182445

    println!("{}", yp.script_meta());
    // 3桁から6桁、10口、加減算です。ねがいましては。
    println!("{}", yp.script_problem());
    // 382円なり、8054円なり、920円なり、... 、引いては476985円なり、加えて38516円では。
    println!("{}", yp.script_answer());
    // その答え、182445円です。
}
