use std::collections::HashMap;

mod format;

pub type EscapeConfig = HashMap<char, String>;

#[derive(Clone, Debug)]
pub enum Rule {
  AST(Box<Rule>),
  Raw(String),
  Escape(EscapeConfig, String),
  List(String, Vec<Rule>),
  Paren(String, Box<Rule>, String),
  Column(Vec<(Rule, ColumnConfig)>),
}

#[derive(Clone, Debug)]
pub struct ColumnConfig {
  /// Knuth/Plas行分割アルゴリズムで使う
  pub break_penalty: isize,
  /// トークン間に入れるスペースの数
  /// `None`であればデフォルトは1つの空白を入れる
  /// 0であれば空白無しで結合する
  pub space_size: Option<usize>,
}

impl Default for ColumnConfig {
  fn default() -> Self {
    ColumnConfig {
      break_penalty: 0,
      space_size: None,
    }
  }
}

impl ColumnConfig {
  fn set_break_penalty(&self, penalty: isize) -> Self {
    ColumnConfig {
      break_penalty: penalty,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Context {
  depth: usize,
  tab_spaces: usize,
  line_width: usize,
  break_str: String,
}

impl Context {
  fn increment_depth(&self) -> Self {
    Context {
      depth: self.depth + 1,
      ..self.clone()
    }
  }
  fn decrement_depth(&self) -> Self {
    let d = if self.depth == 0 { 0 } else { self.depth - 1 };
    Context {
      depth: d,
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
}

pub fn code_format(rule: &Rule) -> String {
  let ctx = Context {
    depth: 0,
    tab_spaces: 2,
    line_width: 35,
    break_str: "\n".to_string(),
  };
  format::code_format(&ctx, &None, rule).join("\n")
}

pub trait Ast2Rule {
  fn to_rule(&self) -> Rule;
}
