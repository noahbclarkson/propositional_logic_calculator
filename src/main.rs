use propositional_logic_calculator::proof::{Proof, SearchSettingsBuilder};

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
    let settings = SearchSettingsBuilder::default()
        .max_line_length(10)
        .iterations(10000000)
        .build()
        .unwrap();
    let mut proof = Proof::new(assumptions, conclusion, Some(settings)).unwrap();
    let result = proof.search();
    match result {
        Ok(_) => println!("Found proof: {}", proof),
        Err(state) => panic!("Did not find proof, state: {:?}", state),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap().to_string();
    input.trim().to_string()
}
