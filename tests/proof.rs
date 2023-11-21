use propositional_logic_calculator::proof::Proof;

fn create_and_test_proof(assumptions: Vec<&str>, conclusion: &str) {
    let assumptions: Vec<String> = assumptions.iter().map(|x| x.to_string()).collect();
    let conclusion = conclusion.to_string();
    let mut proof = Proof::new(assumptions, conclusion, None).unwrap();
    let result = proof.search();
    match result {
        Ok(_) => println!("Found proof: {}", proof),
        Err(state) => panic!("Did not find proof, state: {:?}", state),
    }
}

#[test]
fn test_mpp() {
    create_and_test_proof(vec!["P", "P>Q"], "Q");
}

#[test]
fn test_mtt() {
    create_and_test_proof(vec!["P>Q", "-Q"], "-P");
}

#[test]
fn test_and_introduction() {
    create_and_test_proof(vec!["P", "Q"], "P&Q");
}

#[test]
fn test_and_elimination() {
    create_and_test_proof(vec!["P&Q"], "P");
    create_and_test_proof(vec!["P&Q"], "Q");
}

#[test]
fn test_disjunction_introduction() {
    create_and_test_proof(vec!["P"], "PvQ");
}

