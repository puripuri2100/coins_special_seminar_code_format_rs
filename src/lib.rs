mod format;

#[derive(Clone, Debug)]
pub enum Rule {
  AST(Box<RuleWithComment>),
  Raw(String),
  List(String, Vec<RuleWithComment>),
  Paren(String, Box<RuleWithComment>, String),
  Column(Vec<(RuleWithComment, ColumnConfig)>),
}

#[derive(Clone, Debug)]
pub struct RuleWithComment {
  pub before_comments: Vec<String>,
  pub rule: Rule,
  pub after_comment: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ColumnConfig {
  pub is_break: Option<bool>,
  /// トークン間に入れるスペースの数
  /// `None`であればデフォルトは1つの空白を入れる
  /// 0であれば空白無しで結合する
  pub space_size: Option<usize>,
}

impl ColumnConfig {
  fn set_is_break(&self, penalty: Option<bool>) -> Self {
    ColumnConfig {
      is_break: penalty,
      ..*self
    }
  }
  fn set_space_size(&self, size: usize) -> Self {
    ColumnConfig {
      space_size: Some(size),
      ..*self
    }
  }
}

#[derive(Clone)]
pub struct Context<'a> {
  depth: usize,
  tab_spaces: usize,
  line_width: usize,
  break_str: String,
  list_join_str: Option<String>,
  oneline_comment_format: &'a dyn Fn(String) -> String,
  block_comment_format: &'a dyn Fn(Context, Vec<String>) -> Vec<String>,
}

impl<'a> Context<'a> {
  fn increment_depth(&self) -> Self {
    Context {
      depth: self.depth + 1,
      ..self.clone()
    }
  }
  fn indent(&self) -> String {
    " ".repeat(self.tab_spaces)
  }
  fn len_max(&self) -> usize {
    let indent_len = self.tab_spaces * self.depth;
    self.line_width - indent_len
  }
  fn set_list_join_str(&self, j_opt: Option<String>) -> Self {
    Context {
      list_join_str: j_opt,
      ..self.clone()
    }
  }
}

fn oneline_comment_format(s: String) -> String {
  format!("// {s}")
}
fn block_comment_format(_ctx: Context, s: Vec<String>) -> Vec<String> {
  let mut v = vec![String::from("/*")];
  for s in s {
    v.push(s)
  }
  v.push(String::from("*/"));
  v
}

pub fn code_format(rule_with_comment: &RuleWithComment) -> String {
  let ctx = Context {
    depth: 0,
    tab_spaces: 2,
    line_width: 35,
    break_str: "\n".to_string(),
    list_join_str: None,
    oneline_comment_format: &oneline_comment_format,
    block_comment_format: &block_comment_format,
  };
  format::code_format(&ctx, rule_with_comment)
    .0
    .join(&ctx.break_str)
}

pub trait Ast2RuleWithComment {
  fn to_rule(&self) -> RuleWithComment;
}
