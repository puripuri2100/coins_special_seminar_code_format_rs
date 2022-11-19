pub mod dynamic;
pub mod tree;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ColumnConfig {
  pub is_break: Option<bool>,
  /// トークン間に入れるスペースの数
  /// `None`であればデフォルトは1つの空白を入れる
  /// 0であれば空白無しで結合する
  pub space_size: Option<usize>,
}

impl ColumnConfig {
  pub fn set_is_break(&self, penalty: Option<bool>) -> Self {
    ColumnConfig {
      is_break: penalty,
      ..*self
    }
  }
  pub fn set_space_size(&self, size: usize) -> Self {
    ColumnConfig {
      space_size: Some(size),
      ..*self
    }
  }
}

#[derive(Clone)]
pub struct Context<'a> {
  pub depth: usize,
  pub tab_spaces: usize,
  pub line_width: usize,
  pub break_str: String,
  pub list_join_str: Option<String>,
  pub oneline_comment_format: &'a dyn Fn(String) -> String,
  pub block_comment_format: &'a dyn Fn(Context, Vec<String>) -> Vec<String>,
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
