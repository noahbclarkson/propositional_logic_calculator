use propositional_logic_calculator::proof::{Proof, SearchSettings, SearchSettingsBuilder};

const MAX_ITERATIONS: usize = 100000;
const MAX_LINE_LENGTH: usize = 17;

fn create_and_test_proof(assumptions: Vec<&str>, conclusion: &str) {
    let assumptions: Vec<String> = assumptions.iter().map(|x| x.to_string()).collect();
    let conclusion = conclusion.to_string();
    let mut proof = Proof::new(assumptions, conclusion, build_settings()).unwrap();
    let result = proof.search();
    match result {
        Ok(_) => println!("Found proof: \n{}", proof),
        Err(state) => panic!("Did not find proof, state: {:?}", state),
    }
}

fn build_settings() -> Option<SearchSettings> {
    SearchSettingsBuilder::default()
        .iterations(MAX_ITERATIONS)
        .max_line_length(MAX_LINE_LENGTH)
        .build()
        .ok()
}

#[test]
fn test_mpp() {
    create_and_test_proof(vec!["P", "P>Q"], "Q");
}

#[test]
fn test_double_mpp() {
    create_and_test_proof(vec!["P", "P>Q", "Q>R"], "R");
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
fn test_and_introduction_and_mpp() {
    create_and_test_proof(vec!["P", "Q", "(P&Q)>W"], "W");
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

#[test]
fn test_disjunction_elimination() {
    create_and_test_proof(vec!["PvQ", "P>W", "Q>W"], "W");
}

#[test]
fn test_double_negation() {
    create_and_test_proof(vec!["P"], "-(-P)");
}

#[test]
fn test_double_negation_reverse() {
    create_and_test_proof(vec!["-(-P)"], "P");
}

#[test]
fn test_multi_line_disjunction_elimination() {
    create_and_test_proof(vec!["Pv(Q>R)", "Q", "P>W", "R>W"], "W");
}
