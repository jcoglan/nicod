use crate::expr::Expr;
use crate::state::State;
use im::vector::Vector;
use std::rc::Rc;

pub struct Proof {
    rule: String,
    state: State,
    parents: Vector<Rc<Proof>>,
    conclusion: Rc<Expr>,
}

impl Proof {
    pub fn new(
        rule: &str,
        state: &State,
        proofs: Vector<Rc<Proof>>,
        conclusion: &Rc<Expr>,
    ) -> Proof {
        Proof {
            rule: String::from(rule),
            state: state.clone(),
            parents: proofs,
            conclusion: Rc::clone(conclusion),
        }
    }

    fn conclusion(&self) -> Rc<Expr> {
        self.state.resolve(&self.conclusion)
    }
}

pub fn display(proof: &Proof) {
    display_nested(proof, 1);
    println!("");
}

fn display_nested(proof: &Proof, level: usize) {
    let indent = " ".repeat(4 * level);
    println!("{}[{}] {:?}", indent, proof.rule, proof.conclusion());
    for parent in proof.parents.iter() {
        display_nested(parent, level + 1);
    }
}
