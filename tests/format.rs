extern crate code_format;

use std::default;

use code_format::{code_format, Ast2RuleWithComment, ColumnConfig, Rule, RuleWithComment};

#[derive(Clone, Debug)]
enum Test {
  A(isize),
  B(f64),
  AWithComment(Vec<String>, isize, Option<String>),
  BWithComment(Vec<String>, f64, Option<String>),
  AorB(Box<Test>),
  C(Vec<Test>),
  D(Vec<String>, Vec<Test>),
  Let(Vec<String>, String, Box<Test>),
}

fn make_rule_with_comment_none(rule: Rule) -> RuleWithComment {
  RuleWithComment {
    before_comments: vec![],
    rule: rule,
    after_comment: None,
  }
}

fn make_rule_with_comment(
  before_comments: Vec<String>,
  rule: Rule,
  after_comment: Option<String>,
) -> RuleWithComment {
  RuleWithComment {
    before_comments,
    rule: rule,
    after_comment,
  }
}

impl Ast2RuleWithComment for Test {
  fn to_rule(&self) -> RuleWithComment {
    match self {
      Test::A(int) => make_rule_with_comment_none(Rule::Raw(int.to_string())),
      Test::B(float) => make_rule_with_comment_none(Rule::Paren(
        "(".to_string(),
        Box::new(make_rule_with_comment_none(Rule::Raw(float.to_string()))),
        ")".to_string(),
      )),
      Test::AWithComment(before_comments, int, after_comment) => make_rule_with_comment(
        before_comments.clone(),
        Rule::Raw(int.to_string()),
        after_comment.clone(),
      ),
      Test::BWithComment(before_comments, float, after_comment) => make_rule_with_comment(
        before_comments.clone(),
        Rule::Paren(
          "(".to_string(),
          Box::new(make_rule_with_comment_none(Rule::Raw(float.to_string()))),
          ")".to_string(),
        ),
        after_comment.clone(),
      ),
      Test::AorB(t) => {
        let ast = Box::new(t.to_rule());
        let rule = make_rule_with_comment_none(Rule::AST(ast));
        make_rule_with_comment_none(Rule::Paren(
          "<".to_string(),
          Box::new(rule),
          ">".to_string(),
        ))
      }
      Test::C(lst) => make_rule_with_comment_none(Rule::Paren(
        "[".to_string(),
        Box::new(make_rule_with_comment_none(Rule::List(
          ",".to_string(),
          lst.iter().map(|t| t.to_rule()).collect::<Vec<_>>(),
        ))),
        "]".to_string(),
      )),
      Test::D(before_comments, lst) => make_rule_with_comment(
        before_comments.clone(),
        Rule::Paren(
          "[".to_string(),
          Box::new(make_rule_with_comment_none(Rule::List(
            ",".to_string(),
            lst.iter().map(|t| t.to_rule()).collect::<Vec<_>>(),
          ))),
          "]".to_string(),
        ),
        None,
      ),
      Test::Let(before_comments, name, inner) => {
        let default_cc = ColumnConfig::default();
        make_rule_with_comment(
          before_comments.clone(),
          Rule::Column(vec![
            (
              make_rule_with_comment_none(Rule::Raw("let".to_string())),
              default_cc.set_is_break(Some(false)),
            ),
            (
              make_rule_with_comment_none(Rule::Raw(name.to_string())),
              default_cc.set_is_break(Some(false)).set_space_size(2),
            ),
            (
              make_rule_with_comment_none(Rule::Raw("=".to_string())),
              default_cc.clone(),
            ),
            (
              make_rule_with_comment_none(Rule::Paren(
                "{".to_string(),
                Box::new(make_rule_with_comment_none(Rule::AST(Box::new(
                  inner.to_rule(),
                )))),
                "}".to_string(),
              )),
              default_cc.set_is_break(None),
            ),
          ]),
          None,
        )
      }
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
  let test = make_rule_with_comment_none(Rule::AST(Box::new(make_rule_with_comment_none(
    Rule::Paren(
      "<".to_string(),
      Box::new(make_rule_with_comment_none(Rule::Raw(42.to_string()))),
      ">".to_string(),
    ),
  ))));
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

#[test]
fn check7() {
  let test = Test::D(
    vec!["hoge".to_string(), "fuga".to_string()],
    vec![
      Test::AorB(Box::new(Test::A(42))),
      Test::C(vec![
        Test::A(33333333333),
        Test::B(33333333333.14),
        Test::B(33333333333.141),
      ]),
      Test::AorB(Box::new(Test::A(3333333))),
      Test::A(3333333),
      Test::AorB(Box::new(Test::A(3333333))),
      Test::C(vec![
        Test::AWithComment(vec!["hoge".to_string()], 333333, Some("fuga".to_string())),
        Test::BWithComment(vec!["hoge".to_string()], 333.14, Some("fuga".to_string())),
        Test::B(33333333333.141),
      ]),
    ],
  );
  let ok_str = format!(
    "/*
hoge
fuga
*/
[
  <42>,
  [
    33333333333,
    (33333333333.14),
    (33333333333.141)
  ],
  <3333333>,
  3333333,
  <3333333>,
  [
    // hoge
    333333, // fuga
    // hoge
    (333.14), // fuga
    (33333333333.141)
  ]
]"
  );
  assert_eq!(ok_str, code_format(&test.to_rule()))
}

#[test]
fn check8() {
  let test = Test::D(
    vec!["hoge".to_string(), "fuga".to_string()],
    vec![
      Test::AorB(Box::new(Test::A(42))),
      Test::C(vec![
        Test::A(33333333333),
        Test::B(33333333333.14),
        Test::B(33333333333.141),
      ]),
      Test::AorB(Box::new(Test::A(3333333))),
      Test::A(3333333),
      Test::AorB(Box::new(Test::A(3333333))),
      Test::C(vec![
        Test::AWithComment(vec!["hoge".to_string()], 333333, Some("fuga".to_string())),
        Test::Let(
          vec!["短めのcolumnのテストです".to_string()],
          "name".to_string(),
          Box::new(Test::A(3333333)),
        ),
        Test::BWithComment(vec!["hoge".to_string()], 333.14, Some("fuga".to_string())),
        Test::B(33333333333.141),
      ]),
    ],
  );
  let ok_str = format!(
    "/*
hoge
fuga
*/
[
  <42>,
  [
    33333333333,
    (33333333333.14),
    (33333333333.141)
  ],
  <3333333>,
  3333333,
  <3333333>,
  [
    // hoge
    333333, // fuga
    // 短めのcolumnのテストです
    let name  = {{3333333}},
    // hoge
    (333.14), // fuga
    (33333333333.141)
  ]
]"
  );
  assert_eq!(ok_str, code_format(&test.to_rule()))
}
