mod counter;
mod couple_finder;

type Tester = fn(&str) -> bool;

fn nested_parentheses(n: usize) -> String {
    "{[(".repeat(n) + &")]}".repeat(n)
}

fn list_of_coupled_parentheses(n: usize) -> String {
    "{}[]()".repeat(n)
}

fn run_test(label: &str, input: &str, testers: &[(&str, Tester)]) {
    println!("\nInput: {}", label);
    for (name, tester) in testers {
        let start = std::time::Instant::now();
        let result = tester(input);
        let duration = start.elapsed();

        println!(
            "  {} -> {} in {:?}",
            name,
            if result { "[VALID]  " } else { "[INVALID]" },
            duration
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Error: Please provide a number or a parenthesis string as an argument.");
        eprintln!("Examples:");
        eprintln!("cargo run 1000");
        eprintln!("cargo run \"([])\"");
        std::process::exit(1);
    }

    let testers: [(&str, Tester); 2] = [
        ("counter", counter::is_correct_parenthesis),
        ("couple_finder", couple_finder::is_correct_parenthesis),
    ];

    for arg in &args {
        if let Ok(count) = arg.parse::<usize>() {
            let repeated_inputs = [
                (format!("coupled (n = {})", count), list_of_coupled_parentheses(count)),
                (format!("nested  (n = {})", count), nested_parentheses(count)),
            ];

            for (name, input) in &repeated_inputs {
                run_test(name, input, &testers);
            }
        } else {
            run_test(arg, arg, &testers);
        }
    }
}