extern crate code_format;

use code_format::{code_format, Ast2RuleWithComment, Rule, RuleWithComment};

#[derive(Clone, Debug)]
enum Test {
  A(isize),
  B(f64),
  AorB(Box<Test>),
  C(Vec<Test>),
}

fn make_rule_with_comment(rule: Rule) -> RuleWithComment {
  RuleWithComment {
    before_comments: vec![],
    rule: rule,
    after_comment: None,
  }
}

impl Ast2RuleWithComment for Test {
  fn to_rule(&self) -> RuleWithComment {
    match self {
      Test::A(int) => make_rule_with_comment(Rule::Raw(int.to_string())),
      Test::B(float) => make_rule_with_comment(Rule::Paren(
        "(".to_string(),
        Box::new(make_rule_with_comment(Rule::Raw(float.to_string()))),
        ")".to_string(),
      )),
      Test::AorB(t) => {
        let ast = Box::new(t.to_rule());
        let rule = make_rule_with_comment(Rule::AST(ast));
        make_rule_with_comment(Rule::Paren(
          "<".to_string(),
          Box::new(rule),
          ">".to_string(),
        ))
      }
      Test::C(lst) => make_rule_with_comment(Rule::Paren(
        "[".to_string(),
        Box::new(make_rule_with_comment(Rule::List(
          ",".to_string(),
          lst.iter().map(|t| t.to_rule()).collect::<Vec<_>>(),
        ))),
        "]".to_string(),
      )),
    }
  }
}

fn main() {
  let test = Test::C(vec![
    Test::AorB(Box::new(Test::A(42))),
    Test::C(vec![Test::A(42), Test::B(3.14), Test::B(3.141)]),
    Test::AorB(Box::new(Test::A(3333333))),
    Test::A(3333333),
    Test::AorB(Box::new(Test::A(3333333))),
  ]);
  let code_str = code_format::code_format(&test.to_rule());
  println!("{code_str}")
}
