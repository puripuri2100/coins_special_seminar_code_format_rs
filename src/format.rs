use crate::{ColumnConfig, Context, Rule};

pub fn code_format(ctx: &Context, is_lst_break_force: &Option<bool>, rule: &Rule) -> Vec<String> {
  match rule {
    Rule::AST(ast) => code_format(ctx, is_lst_break_force, ast),
    Rule::Raw(str) => vec![str.to_string()],
    Rule::Paren(open, rule, close) => {
      let str_lst = code_format(ctx, is_lst_break_force, rule);
      if str_lst.len() <= 1 {
        vec![format!("{open}{}{close}", str_lst.join(""))]
      } else {
        let mut v = Vec::new();
        v.push(open.to_string());
        for str in code_format(&ctx.increment_depth(), is_lst_break_force, rule) {
          v.push(str.to_string())
        }
        v.push(close.to_string());
        v
      }
    }
    Rule::List(join, lst) => {
      let tab = ctx.indent();
      let mut is_multiline = false;
      for rule in lst.iter() {
        if code_format(ctx, &None, rule).len() > 1 {
          is_multiline = true;
          break;
        }
      }
      if let Some(true) = *is_lst_break_force {
        is_multiline = true
      }
      if !is_multiline {
        let str = lst
          .iter()
          .map(|rule| code_format(ctx, &None, rule).join(""))
          .collect::<Vec<_>>()
          .join(&format!("{join} "));
        if str.len() < ctx.len_max() {
          // 内部が一行で表せて、かつその長さが設定されている一行の長さよりも短い場合にonelineとなる
          return vec![str];
        }
      }
      let mut v = vec![];
      let mut lst = lst
        .iter()
        .map(|rule| code_format(&ctx.increment_depth(), &None, rule))
        .peekable();
      loop {
        match lst.next() {
          Some(code_lst) => {
            let is_last = lst.peek().is_none(); // 全体の最後
            let mut code_iter = code_lst.iter().peekable();
            loop {
              match code_iter.next() {
                Some(code) => {
                  let is_local_last = code_iter.peek().is_none();
                  if !is_last && is_local_last {
                    v.push(format!("{tab}{}{join}", code))
                  } else {
                    v.push(format!("{tab}{}", code))
                  }
                }
                None => break,
              }
            }
          }
          None => break,
        }
      }
      v
    }
    Rule::Column(lst) => break_token_column(ctx, lst),
    _ => vec![],
  }
}

/// Knuth and Plass line breaking algorithmをもとにトークン列を分割するアルゴリズム
/// - RuleがListであるとき、それを強制分割させることもでき、その場合文字の長さとペナルティ値が変化する
/// - 行分割されると、分割後はdepthが深くなる
/// - 行分割されるとその個所に`space_size`は`0`でよくなる
fn break_token_column(ctx: &Context, lst: &[(Rule, ColumnConfig)]) -> Vec<String> {
  let mut v = vec![];
  todo!();
  v
}
