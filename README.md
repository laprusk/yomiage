# Yomiage-zan Automatic Generation Library

読上算、読上暗算の問題を自動生成するRustライブラリです。

## Example

```rust
use yomiage::{YomiageProblem, YomiageConfig};

fn main() {
    let config = YomiageConfig {
        min_digit: 3,
        max_digit: 6,
        length: 10,
        subtractions: 3,
        allow_negative: false,
    };
    let yp = YomiageProblem::new(config).unwrap();

    println!("Problem: {:?}", yp.problem);
    println!("Answer: {}", yp.answer);

    let script = yp.script_meta() + &yp.script_problem() + &yp.script_answer();
    println!("Script: {}", script);
}
```
