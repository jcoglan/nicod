use crate::expr::Expr;
use crate::state::State;
use im::vector::Vector;
use std::cmp;
use std::fmt;
use std::rc::Rc;

pub struct Proof {
    rule: String,
    state: State,
    parents: Vector<Rc<Proof>>,
    conclusion: (usize, Rc<Expr>),
}

impl Proof {
    pub fn new(
        rule: &str,
        state: &State,
        proofs: Vector<Rc<Proof>>,
        conclusion: (usize, &Rc<Expr>),
    ) -> Proof {
        Proof {
            rule: String::from(rule),
            state: state.clone(),
            parents: proofs,
            conclusion: (conclusion.0, Rc::clone(conclusion.1)),
        }
    }

    fn conclusion(&self) -> Rc<Expr> {
        let (scope, expr) = &self.conclusion;
        self.state.resolve_scoped((*scope, expr))
    }
}

impl fmt::Debug for Proof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_nested(self, f, 1)
    }
}

fn display_nested(proof: &Proof, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
    let indent = " ".repeat(4 * level);
    writeln!(f, "{}[{}] {}", indent, proof.rule, proof.conclusion())?;
    for parent in proof.parents.iter() {
        display_nested(parent, f, level + 1)?;
    }
    Ok(())
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut layout = Layout::new(self);
        layout.render(f)
    }
}

const DIVIDER: char = '\u{2500}';
const PADDING: usize = 3;

struct Layout<'a> {
    proof: &'a Proof,
    parents: Vec<Layout<'a>>,
    premise_indent: usize,
    divider: Bounds,
    conclusion: Bounds,
    region: Bounds,
}

impl Layout<'_> {
    fn new(proof: &Proof) -> Layout<'_> {
        let parents = proof.parents.iter().map(|t| Layout::new(&**t)).collect();

        Layout {
            proof,
            parents,
            premise_indent: 0,
            divider: Bounds::default(),
            conclusion: Bounds::default(),
            region: Bounds::default(),
        }
    }

    fn render(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut plan = Plan::default();
        self.prepare(0);
        self.generate(&mut plan, 1, 0);
        plan.render(f)
    }

    fn conclusion(&self) -> String {
        format!("{}", self.proof.conclusion())
    }

    fn prepare(&mut self, offset: usize) {
        self.parents.iter_mut().fold(offset, |ofs, layout| {
            layout.prepare(ofs);
            layout.region.right + PADDING
        });

        let mut parents_width = 0;
        let mut premise_left = offset;
        let mut premise_width = 0;

        if let (Some(first), Some(last)) = (self.parents.first(), self.parents.last()) {
            parents_width = last.region.right - first.region.left;
            premise_left = first.conclusion.left;
            premise_width = last.conclusion.right - premise_left;
        }

        let rule_width = self.proof.rule.chars().count() + 1;
        let conc_width = self.conclusion().chars().count();
        let divider_width = cmp::max(conc_width, premise_width);
        let conc_indent = (divider_width - conc_width) / 2;

        if premise_width < conc_width {
            let premise_offset = 2 * (premise_left - offset);
            let premise_indent = conc_width - premise_width;

            premise_left -= cmp::min(premise_offset, premise_indent) / 2;

            if premise_offset < premise_indent {
                self.premise_indent = premise_indent - premise_offset;
            }
        }

        self.divider.position(premise_left, divider_width);

        self.conclusion
            .position(self.divider.left + conc_indent, conc_width);

        self.region.left = offset;
        self.region.right = cmp::max(
            self.region.left + parents_width,
            self.divider.right + rule_width,
        );
    }

    fn generate(&self, plan: &mut Plan, depth: usize, indent: usize) {
        for layout in &self.parents {
            layout.generate(plan, depth + 2, indent + self.premise_indent);
        }

        let divider = format!("{} {}", self.divider.repeat(DIVIDER), self.proof.rule);
        let indent = indent / 2;

        plan.write(depth + 1, indent + self.divider.left, &divider);
        plan.write(depth, indent + self.conclusion.left, &self.conclusion());
    }
}

#[derive(Default)]
struct Bounds {
    left: usize,
    right: usize,
}

impl Bounds {
    fn position(&mut self, left: usize, width: usize) {
        self.left = left;
        self.right = left + width;
    }

    fn repeat(&self, ch: char) -> String {
        let width = self.right - self.left;
        ch.to_string().repeat(width)
    }
}

#[derive(Default)]
struct Plan {
    lines: Vec<String>,
}

impl Plan {
    fn write(&mut self, depth: usize, offset: usize, text: &str) {
        while self.lines.len() < depth {
            self.lines.push(String::new());
        }

        if let Some(line) = self.lines.get_mut(depth - 1) {
            line.push_str(&" ".repeat(offset - line.chars().count()));
            line.push_str(text);
        }
    }

    fn render(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.lines.iter().rev() {
            writeln!(f, "    {}", line)?;
        }
        Ok(())
    }
}
