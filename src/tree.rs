mod format;

use crate::{ColumnConfig, Context};

/// コードの形態を4つに分類した。
/// 
/// - リテラル
/// - 同じ階層のトークン列
/// - 括弧
/// - 途中で改行を行ったりできるトークン列
/// 
/// これをそれぞれRaw・List・Paren・Columnとして名前を付けた。
#[derive(Clone, Debug)]
pub enum Rule {
  AST(Box<RuleWithComment>),
  Raw(String),
  List(String, Vec<RuleWithComment>),
  Paren(String, Box<RuleWithComment>, String),
  Column(Vec<(RuleWithComment, ColumnConfig)>),
}

/// コメントはそのトークンの前に出現するものと、そのトークンの直後に出現するものにわかれる。
/// トークンの前に出現するコメントは複数行が存在するため、リストで管理する。
/// トークンの直後に出現するコメントは「存在する」か「存在しない」なのでOptionで管理する
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
