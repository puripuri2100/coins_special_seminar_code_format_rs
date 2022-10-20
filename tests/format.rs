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

#[test]

fn check1() {
  let test = Test::B(3.14);
  let ok_str = format!("(3.14)");
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

#[test]

fn check2() {
  let test = make_rule_with_comment(Rule::AST(Box::new(make_rule_with_comment(Rule::Paren(
    "<".to_string(),
    Box::new(make_rule_with_comment(Rule::Raw(42.to_string()))),
    ">".to_string(),
  )))));
  let ok_str = format!("<42>");
  assert_eq!(ok_str, code_format(&test))
}

#[test]

fn check3() {
  let test = Test::AorB(Box::new(Test::A(42)));
  let ok_str = format!("<42>");
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

#[test]

fn check4() {
  let test = Test::C(vec![
    Test::AorB(Box::new(Test::A(42))),
    Test::C(vec![Test::A(42), Test::B(3.14)]),
  ]);
  let ok_str = format!("[<42>, [42, (3.14)]]");
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

#[test]
fn check5() {
  let test = Test::C(vec![
    Test::AorB(Box::new(Test::A(42))),
    Test::C(vec![Test::A(42), Test::B(3.14), Test::B(3.141)]),
    Test::AorB(Box::new(Test::A(3333333))),
    Test::A(3333333),
    Test::AorB(Box::new(Test::A(3333333))),
  ]);
  let ok_str = format!(
    "[
  <42>,
  [42, (3.14), (3.141)],
  <3333333>,
  3333333,
  <3333333>
]"
  );
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

#[test]
fn check6() {
  let test = Test::C(vec![
    Test::AorB(Box::new(Test::A(42))),
    Test::C(vec![
      Test::A(33333333333),
      Test::B(33333333333.14),
      Test::B(33333333333.141),
    ]),
    Test::AorB(Box::new(Test::A(3333333))),
    Test::A(3333333),
    Test::AorB(Box::new(Test::A(3333333))),
  ]);
  let ok_str = format!(
    "[
  <42>,
  [
    33333333333,
    (33333333333.14),
    (33333333333.141)
  ],
  <3333333>,
  3333333,
  <3333333>
]"
  );
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

//#[derive(Clone, Debug)]
//enum Test2 {
//  Value(Value2),
//  Op(String, Value2, Value2),
//  NonRecLet(String, Value2),
//  RecLet(String, Value2),
//  LetMutable(String, Value2),
//}
//
//#[derive(Clone, Debug)]
//enum Value2 {
//  Int(isize),
//  Float(f64),
//  String(String),
//}
//
//impl Ast2Rule for Test2 {
//  fn to_rule(&self) -> Rule {
//    match self {
//      Test2::Value(v) => Rule::Raw(value2_to_string(v)),
//      Test2::Op(op, v1, v2) => Rule::List(
//        "".to_string(),
//        vec![
//          Rule::Raw(value2_to_string(v1)),
//          Rule::Raw(op.to_string()),
//          Rule::Raw(value2_to_string(v2)),
//        ],
//      ),
//      Test2::NonRecLet(name, v) => Rule::List(
//        "".to_string(),
//        vec![
//          Rule::Raw("let".to_string()),
//          Rule::NonBreak,
//          Rule::Raw(name.to_string()),
//          Rule::NonBreak,
//          Rule::Raw("=".to_string()),
//          Rule::Raw(value2_to_string(v)),
//        ],
//      ),
//      Test2::RecLet(name, v) => Rule::List(
//        "".to_string(),
//        vec![
//          Rule::Raw("let".to_string()),
//          Rule::NonBreak,
//          Rule::Raw("rec".to_string()),
//          Rule::NonBreak,
//          Rule::Raw(name.to_string()),
//          Rule::NonBreak,
//          Rule::Raw("=".to_string()),
//          Rule::Raw(value2_to_string(v)),
//        ],
//      ),
//      Test2::LetMutable(name, v) => Rule::List(
//        "".to_string(),
//        vec![
//          Rule::Raw("let".to_string()),
//          Rule::NonBreak,
//          Rule::Raw("mutable".to_string()),
//          Rule::NonBreak,
//          Rule::Raw(name.to_string()),
//          Rule::NonBreak,
//          Rule::Raw("<-".to_string()),
//          Rule::Raw(value2_to_string(v)),
//        ],
//      ),
//    }
//  }
//}
//
//fn value2_to_string(v: &Value2) -> String {
//  match v {
//    Value2::Int(n) => n.to_string(),
//    Value2::Float(f) => f.to_string(),
//    Value2::String(s) => s.to_string(),
//  }
//}
//
//#[test]
//fn check6() {
//  let test = Test2::NonRecLet("hoge".to_string(), Value2::Int(4));
//  let ok_str = format!("let hoge = 4");
//  assert_eq!(ok_str, code_format(&test.to_rule()))
//}
//
