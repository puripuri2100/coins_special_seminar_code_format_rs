mod format;

use crate::{ColumnConfig, Context};

/// 
#[derive(Clone, Debug)]
pub enum Rule {
  AST(Box<RuleWithComment>),
  Raw(String),
  List(String, Vec<RuleWithComment>),
  Paren(String, Box<RuleWithComment>, String),
  Column(Vec<(RuleWithComment, ColumnConfig)>),
}

/// コメントはそのトークンの出現する前の
#[derive(Clone, Debug)]
pub struct RuleWithComment {
  pub before_comments: Vec<String>,
  pub rule: Rule,
  pub after_comment: Option<String>,
}

pub fn code_format(ctx: &Context, rule_with_comment: &RuleWithComment) -> String {
  format::code_format(ctx, rule_with_comment)
    .0
    .join(&ctx.break_str)
}

pub trait Ast2RuleWithComment {
  fn to_rule(&self) -> RuleWithComment;
}
