use propositional_logic_calculator::proof::Proof;

fn main() {
    println!("Enter the propositional logic statement: ");
    let input = get_input();
    // Assumptions sepereated by commas, e.g. A,B->C,BvC,D&E
    // Conclusion is seperated by a '/' e.g. A,B->C,BvC,D&E/A
    let mut assumptions: Vec<&str> = input.split('/').collect();
    let conclusion = assumptions
        .pop()
        .unwrap_or_else(|| panic!("No conclusion found"));
    let assumptions: Vec<&str> = assumptions.pop().unwrap().split(',').collect();
    let assumptions: Vec<String> = assumptions.iter().map(|x| x.to_string()).collect();
    let conclusion = conclusion.to_string();
    let mut proof = Proof::new(assumptions, conclusion, None);
    println!("Proof: {}", proof);
    proof.run();
    println!("Proof: {}", proof);
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap().to_string();
    input.trim().to_string()
}
