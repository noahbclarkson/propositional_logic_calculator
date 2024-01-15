use anyhow::Context;
use propositional_logic_calculator::proof::{parse_expression, Proof, SearchSettings};

fn main() -> anyhow::Result<()> {
    println!("Enter the propositional logic statement: ");
    let input = get_input();
    // Assumptions sepereated by commas, e.g. A,B->C,BvC,D&E
    // Conclusion is seperated by a '/' e.g. A,B->C,BvC,D&E/A
    let (assumptions_str, conclusion_str) = input
        .split_once('/')
        .context("Need a '/' to delimit assumptions and conclusion")?;

    let assumptions = assumptions_str
        .split(',')
        .map(parse_expression)
        .collect::<Result<_, _>>()?;
    let conclusion = parse_expression(conclusion_str)?;

    let mut proof = Proof::with_settings(
        assumptions,
        conclusion,
        SearchSettings {
            max_line_length: 20,
            iterations: 100_000,
        },
    );

    proof.search().context("Did not find proof")?;
    println!("{}", proof);
    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
