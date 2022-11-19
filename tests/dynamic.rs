extern crate code_format;

use code_format::{dynamic::*, ColumnConfig};
use std::collections::HashMap;

#[test]
fn check_rule_to_listedrule_1() {
  let rule = Rule::Column(
    Some("column1".to_string()),
    vec![
      (
        RuleWithComment {
          before_comments: vec!["comment1".to_string()],
          rule: Rule::Raw("let".to_string()),
          after_comment: Some("comment1".to_string()),
        },
        ColumnConfig::default(),
      ),
      (
        RuleWithComment {
          before_comments: vec!["paren1".to_string()],
          rule: Rule::Paren(
            Some("paren1".to_string()),
            "[".to_string(),
            Box::new(RuleWithComment {
              before_comments: vec!["list1".to_string()],
              rule: Rule::List(
                Some("list1".to_string()),
                ",".to_string(),
                vec![
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("abc".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Unconfirmed("tag1".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::AST(Box::new(RuleWithComment {
                      before_comments: vec![],
                      rule: Rule::List(
                        Some("list2".to_string()),
                        ";".to_string(),
                        vec![
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Unconfirmed("tag2".to_string()),
                            after_comment: None,
                          },
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Raw("s".to_string()),
                            after_comment: None,
                          },
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Unconfirmed("tag3".to_string()),
                            after_comment: None,
                          },
                        ],
                      ),
                      after_comment: None,
                    })),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("123".to_string()),
                    after_comment: None,
                  },
                ],
              ),
              after_comment: Some("list1".to_string()),
            }),
            "]".to_string(),
          ),
          after_comment: Some("paren1".to_string()),
        },
        ColumnConfig::default(),
      ),
    ],
  );
  let listedrules = vec![
    ListedRule::Open(OpenRule::Column(Some("column1".to_string()))),
    ListedRule::Close(CloseRule::Column),
  ];
  let mut tag_data = HashMap::new();
  tag_data.insert(
    "column1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::ColumnContents(
          ColumnConfig::default(),
          vec!["comment1".to_string()],
        )),
        ListedRule::Raw("let".to_string()),
        ListedRule::Close(CloseRule::ColumnContents(Some("comment1".to_string()))),
        ListedRule::Open(OpenRule::ColumnContents(
          ColumnConfig::default(),
          vec!["paren1".to_string()],
        )),
        ListedRule::Open(OpenRule::Paren(
          Some("paren1".to_string()),
          "[".to_string(),
          vec!["list1".to_string()],
        )),
        ListedRule::Close(CloseRule::Paren("]".to_string(), Some("list1".to_string()))),
        ListedRule::Close(CloseRule::ColumnContents(Some("paren1".to_string()))),
      ],
    },
  );
  tag_data.insert(
    "paren1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::List(Some("list1".to_string()), ",".to_string())),
        ListedRule::Close(CloseRule::List),
      ],
    },
  );
  tag_data.insert(
    "list1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("abc".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag1".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Open(OpenRule::List(Some("list2".to_string()), ";".to_string())),
        ListedRule::Close(CloseRule::List),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("123".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
      ],
    },
  );
  tag_data.insert(
    "list2".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag2".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("s".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag3".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
      ],
    },
  );
  let generate_listedrule = rule_to_listedrule(&rule);
  assert_eq!((listedrules, tag_data), generate_listedrule)
}

#[test]
fn check_rule_to_listedrule_2() {
  let rule = Rule::Column(
    None,
    vec![
      (
        RuleWithComment {
          before_comments: vec!["comment1".to_string()],
          rule: Rule::Raw("let".to_string()),
          after_comment: Some("comment1".to_string()),
        },
        ColumnConfig::default(),
      ),
      (
        RuleWithComment {
          before_comments: vec!["paren1".to_string()],
          rule: Rule::Paren(
            None,
            "[".to_string(),
            Box::new(RuleWithComment {
              before_comments: vec!["list1".to_string()],
              rule: Rule::List(
                None,
                ",".to_string(),
                vec![
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("abc".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Unconfirmed("tag1".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::AST(Box::new(RuleWithComment {
                      before_comments: vec![],
                      rule: Rule::List(
                        None,
                        ";".to_string(),
                        vec![
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Unconfirmed("tag2".to_string()),
                            after_comment: None,
                          },
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Raw("s".to_string()),
                            after_comment: None,
                          },
                          RuleWithComment {
                            before_comments: vec![],
                            rule: Rule::Unconfirmed("tag3".to_string()),
                            after_comment: None,
                          },
                        ],
                      ),
                      after_comment: None,
                    })),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("123".to_string()),
                    after_comment: None,
                  },
                ],
              ),
              after_comment: Some("list1".to_string()),
            }),
            "]".to_string(),
          ),
          after_comment: Some("paren1".to_string()),
        },
        ColumnConfig::default(),
      ),
    ],
  );
  let listedrules = vec![
    ListedRule::Open(OpenRule::Column(None)),
    ListedRule::Open(OpenRule::ColumnContents(
      ColumnConfig::default(),
      vec!["comment1".to_string()],
    )),
    ListedRule::Raw("let".to_string()),
    ListedRule::Close(CloseRule::ColumnContents(Some("comment1".to_string()))),
    ListedRule::Open(OpenRule::ColumnContents(
      ColumnConfig::default(),
      vec!["paren1".to_string()],
    )),
    ListedRule::Open(OpenRule::Paren(
      None,
      "[".to_string(),
      vec!["list1".to_string()],
    )),
    ListedRule::Open(OpenRule::List(None, ",".to_string())),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("abc".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag1".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Open(OpenRule::List(None, ";".to_string())),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag2".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("s".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag3".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Close(CloseRule::List),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("123".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Close(CloseRule::List),
    ListedRule::Close(CloseRule::Paren("]".to_string(), Some("list1".to_string()))),
    ListedRule::Close(CloseRule::ColumnContents(Some("paren1".to_string()))),
    ListedRule::Close(CloseRule::Column),
  ];
  let tag_data = HashMap::new();
  let generate_listedrule = rule_to_listedrule(&rule);
  assert_eq!((listedrules, tag_data), generate_listedrule)
}

#[test]
fn check_listedrule_to_rule_1() {
  let rule = Rule::Column(
    None,
    vec![
      (
        RuleWithComment {
          before_comments: vec!["comment1".to_string()],
          rule: Rule::Raw("let".to_string()),
          after_comment: Some("comment1".to_string()),
        },
        ColumnConfig::default(),
      ),
      (
        RuleWithComment {
          before_comments: vec!["paren1".to_string()],
          rule: Rule::Paren(
            None,
            "[".to_string(),
            Box::new(RuleWithComment {
              before_comments: vec!["list1".to_string()],
              rule: Rule::List(
                None,
                ",".to_string(),
                vec![
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("abc".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Unconfirmed("tag1".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::List(
                      None,
                      ";".to_string(),
                      vec![
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Unconfirmed("tag2".to_string()),
                          after_comment: None,
                        },
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Raw("s".to_string()),
                          after_comment: None,
                        },
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Unconfirmed("tag3".to_string()),
                          after_comment: None,
                        },
                      ],
                    ),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("123".to_string()),
                    after_comment: None,
                  },
                ],
              ),
              after_comment: Some("list1".to_string()),
            }),
            "]".to_string(),
          ),
          after_comment: Some("paren1".to_string()),
        },
        ColumnConfig::default(),
      ),
    ],
  );
  let listedrules = vec![
    ListedRule::Open(OpenRule::Column(Some("column1".to_string()))),
    ListedRule::Close(CloseRule::Column),
  ];
  let mut tag_data = HashMap::new();
  tag_data.insert(
    "column1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::ColumnContents(
          ColumnConfig::default(),
          vec!["comment1".to_string()],
        )),
        ListedRule::Raw("let".to_string()),
        ListedRule::Close(CloseRule::ColumnContents(Some("comment1".to_string()))),
        ListedRule::Open(OpenRule::ColumnContents(
          ColumnConfig::default(),
          vec!["paren1".to_string()],
        )),
        ListedRule::Open(OpenRule::Paren(
          Some("paren1".to_string()),
          "[".to_string(),
          vec!["list1".to_string()],
        )),
        ListedRule::Close(CloseRule::Paren("]".to_string(), Some("list1".to_string()))),
        ListedRule::Close(CloseRule::ColumnContents(Some("paren1".to_string()))),
      ],
    },
  );
  tag_data.insert(
    "paren1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::List(Some("list1".to_string()), ",".to_string())),
        ListedRule::Close(CloseRule::List),
      ],
    },
  );
  tag_data.insert(
    "list1".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("abc".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag1".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Open(OpenRule::List(Some("list2".to_string()), ";".to_string())),
        ListedRule::Close(CloseRule::List),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("123".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
      ],
    },
  );
  tag_data.insert(
    "list2".to_string(),
    InternalRule {
      rules: vec![
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag2".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Raw("s".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
        ListedRule::Open(OpenRule::Contents(vec![])),
        ListedRule::Unconfirmed("tag3".to_string()),
        ListedRule::Close(CloseRule::Contents(None)),
      ],
    },
  );
  let flat = flat_listedrule(&listedrules, &tag_data);
  let generate_rule = listedrule_to_rule(&flat, 0);
  assert_eq!(
    (
      RuleWithComment {
        before_comments: vec![],
        rule,
        after_comment: None
      },
      None,
      33
    ),
    generate_rule
  )
}

#[test]
fn check_listedrule_to_rule_2() {
  let rule = Rule::Column(
    None,
    vec![
      (
        RuleWithComment {
          before_comments: vec!["comment1".to_string()],
          rule: Rule::Raw("let".to_string()),
          after_comment: Some("comment1".to_string()),
        },
        ColumnConfig::default(),
      ),
      (
        RuleWithComment {
          before_comments: vec!["paren1".to_string()],
          rule: Rule::Paren(
            None,
            "[".to_string(),
            Box::new(RuleWithComment {
              before_comments: vec!["list1".to_string()],
              rule: Rule::List(
                None,
                ",".to_string(),
                vec![
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("abc".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Unconfirmed("tag1".to_string()),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::List(
                      None,
                      ";".to_string(),
                      vec![
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Unconfirmed("tag2".to_string()),
                          after_comment: None,
                        },
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Raw("s".to_string()),
                          after_comment: None,
                        },
                        RuleWithComment {
                          before_comments: vec![],
                          rule: Rule::Unconfirmed("tag3".to_string()),
                          after_comment: None,
                        },
                      ],
                    ),
                    after_comment: None,
                  },
                  RuleWithComment {
                    before_comments: vec![],
                    rule: Rule::Raw("123".to_string()),
                    after_comment: None,
                  },
                ],
              ),
              after_comment: Some("list1".to_string()),
            }),
            "]".to_string(),
          ),
          after_comment: Some("paren1".to_string()),
        },
        ColumnConfig::default(),
      ),
    ],
  );
  let listedrules = vec![
    ListedRule::Open(OpenRule::Column(None)),
    ListedRule::Open(OpenRule::ColumnContents(
      ColumnConfig::default(),
      vec!["comment1".to_string()],
    )),
    ListedRule::Raw("let".to_string()),
    ListedRule::Close(CloseRule::ColumnContents(Some("comment1".to_string()))),
    ListedRule::Open(OpenRule::ColumnContents(
      ColumnConfig::default(),
      vec!["paren1".to_string()],
    )),
    ListedRule::Open(OpenRule::Paren(
      None,
      "[".to_string(),
      vec!["list1".to_string()],
    )),
    ListedRule::Open(OpenRule::List(None, ",".to_string())),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("abc".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag1".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Open(OpenRule::List(None, ";".to_string())),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag2".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("s".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Unconfirmed("tag3".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Close(CloseRule::List),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Open(OpenRule::Contents(vec![])),
    ListedRule::Raw("123".to_string()),
    ListedRule::Close(CloseRule::Contents(None)),
    ListedRule::Close(CloseRule::List),
    ListedRule::Close(CloseRule::Paren("]".to_string(), Some("list1".to_string()))),
    ListedRule::Close(CloseRule::ColumnContents(Some("paren1".to_string()))),
    ListedRule::Close(CloseRule::Column),
  ];
  let generate_rule = listedrule_to_rule(&listedrules, 0);
  assert_eq!(
    (
      RuleWithComment {
        before_comments: vec![],
        rule,
        after_comment: None
      },
      None,
      35
    ),
    generate_rule
  )
}
