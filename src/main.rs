use std::io;

use left_pad::leftpad;

fn vec_is_0s<T: PartialEq<i64>>(vec: &Vec<T>) -> bool {
    vec.iter().all(|x| *x == 0)
}

fn differences_between_numbers<T>(vec: &Vec<T>) -> Vec<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    let mut differences = Vec::new();
    for i in 0..vec.len() - 1 {
        differences.push(vec[i + 1] - vec[i]);
    }
    differences
}

fn print_rank(rank: &Vec<Vec<i64>>) {
    for i in 0..rank.len() {
        let s = "  ".repeat(i);
        print!("{}", s);
        println!(
            "{} ",
            rank[i]
                .clone()
                .into_iter()
                .map(|x| leftpad(x.to_string(), 3).into_owned())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    println!("");
}

fn falling_factorial(n: i64, k: i64) -> i64 {
    let mut ret = 1;
    for i in 0..k {
        ret *= n - i;
    }
    ret
}

/// Returns the first n rows of the Stirling numbers of the first kind
/// Also adds signs
/// Return order is ascending
/// skips 0
/// ex)
/// ```sh
/// input: 5
///
/// 1
/// -1 1
/// 2 -3 1
/// -6 11 -6 1
/// 24 -50 35 -10 1
/// ```
fn stirling_first(n: usize) -> Vec<Vec<i64>> {
    let mut rows: Vec<Vec<i64>> = vec![vec![1]];

    // represents
    // ()x + ()x^2 + ()x^3 + ()x^4 ...

    for i in 1..n {
        // multiplying x - i
        let prev = rows.last().unwrap();
        let mut row = prev.clone();
        row.insert(0, 0);

        for idx in 0..prev.len() {
            row[idx] += -(i as i64) * prev[idx];
        }

        rows.push(row);
    }

    rows
}

fn summation_intermediate(deg: i64, c: i64, x: i64) -> i64 {
    c * falling_factorial(x, deg) / (1..=deg).product::<i64>()
}

fn x_to_the_power_of(deg: usize, mut c: f64, first: bool) -> String {
    // Hide 1 in front of x and if - then add space in between
    let mut s = String::new();

    if c == 0.0 {
        return s;
    }

    if c < 0.0 {
        if first {
            s.push_str("-");
        } else {
            s.push_str(" - ");
        }

        c = -c;
    } else if !first {
        s.push_str(" + ");
    }

    if c != 1.0 || deg == 0 {
        s.push_str(&format!("{}", c));
    }

    if deg == 0 {
        return s;
    }

    s.push_str("x");

    if deg > 1 {
        s.push_str(&format!("^{}", deg));
    }

    s
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;

    println!("Numbers: {}", buffer);
    let nums = buffer.split_whitespace();
    let mut numbers: Vec<i64> = Vec::new();
    for num in nums {
        let number = num.parse::<i64>().unwrap();
        numbers.push(number);
    }

    let mut rank = vec![numbers];

    let mut nopattern = false;

    while !vec_is_0s(&rank[rank.len() - 1]) {
        let differences = differences_between_numbers(&rank[rank.len() - 1]);

        let l = differences.len();

        rank.push(differences);

        if l == 1 {
            nopattern = true;
            break;
        }
    }

    print_rank(&rank);

    if !nopattern {
        println!("A pattern was found. Attempting to generate polynomial using voodoo magic...")
    } else {
        println!("No pattern was found. The following sequence generated fits the numbers given");
        println!("but does not imply that this is a sequence of a polynomial");
    }

    let final_form = |x| {
        let mut sum = 0;
        for i in 0..rank.len() {
            sum += summation_intermediate(i as i64, rank[i][0], x);
        }
        sum
    };

    for i in 0..20 {
        print!("{} ", final_form(i));
    }

    println!();

    // x^2 + 0x + 0
    // 0 - 0 1 4 9
    // 1 - 1 2 5
    // 2 - 2 2
    // 3 - 0
    //
    // 0 -> 0 / 1
    // 1 -> 1 * x / 1
    // 2 -> 2 * x * (x - 1) / 2
    //
    // 0 -> 0
    // 1 -> x
    // 2 -> x^2 - x

    let eff = polynomial_coefficients(&rank);

    println!(
        "Polynomial equation: {:?}",
        polynomial_coefficients_to_string(&eff)
    );

    Ok(())
}

fn polynomial_coefficients(rank: &Vec<Vec<i64>>) -> Vec<f64> {
    let mut coefficients = vec![0f64; rank.len()];

    // constant coefficient
    coefficients[0] = rank[0][0] as f64;

    for (idx, stirling_row) in stirling_first(rank.len() - 1).iter().enumerate() {
        let p = (1..=(idx + 1) as i64).product::<i64>() as f64;
        let c = rank[idx + 1][0] as i64;
        for idx in 0..stirling_row.len() {
            coefficients[idx + 1] += (c * stirling_row[idx]) as f64 / p;
        }
    }

    coefficients
}

fn polynomial_coefficients_to_string(coefficients: &Vec<f64>) -> String {
    // Write in descending order
    let mut s = String::new();
    let mut first = true;
    for (deg, c) in coefficients.iter().enumerate().rev() {
        if *c == 0f64 {
            continue;
        }

        s.push_str(&x_to_the_power_of(deg, *c, first));

        first = false;
    }

    s
}
