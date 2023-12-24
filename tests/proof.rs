use propositional_logic_calculator::proof::{parse_expression, Proof, SearchSettings};

fn create_and_test_proof(assumptions: Vec<&str>, conclusion: &str) {
    let assumptions = assumptions
        .into_iter()
        .map(parse_expression)
        .collect::<Result<_, _>>()
        .unwrap();
    let conclusion = parse_expression(conclusion).unwrap();

    let mut proof = Proof::with_settings(
        assumptions,
        conclusion,
        SearchSettings {
            max_line_length: 12,
            iterations: 25_000,
        },
    );
    let result = proof.search();
    match result {
        Ok(_) => println!("Found proof: \n{}", proof),
        Err(state) => panic!("Did not find proof, state: {:?}", state),
    }
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
    create_and_test_proof(vec!["Q"], "PvQ");
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

#[test]
fn test_disjunction_elimination_reversal() {
    create_and_test_proof(vec!["PvQ"], "QvP");
}

#[test]
fn test_dn_and_mtt() {
    create_and_test_proof(vec!["P>-Q", "Q"], "-P");
}

#[test]
fn test_conditional_proof() {
    create_and_test_proof(vec!["P>R", "R>Q"], "P>Q");
}
