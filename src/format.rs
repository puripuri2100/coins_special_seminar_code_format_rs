use crate::{ColumnConfig, Context, Rule, RuleWithComment};

#[allow(unreachable_patterns)]
pub fn code_format(ctx: &Context, rule_with_comment: &RuleWithComment) -> (Vec<String>, bool) {
  let rule = &rule_with_comment.rule;
  match rule {
    Rule::AST(ast) => {
      let mut v = vec![];
      if let Some(mut code_vec) = before_comments_format(ctx, &rule_with_comment.before_comments) {
        v.append(&mut code_vec)
      }
      let mut rule_format_vec = code_format(ctx, &*ast).0;
      v.append(&mut rule_format_vec);
      if let Some(after_comment) = &rule_with_comment.after_comment {
        v.push(after_comment.to_string());
        (v, true)
      } else {
        (v, false)
      }
    }
    Rule::Raw(str) => {
      let mut v = vec![];
      if let Some(mut code_vec) = before_comments_format(ctx, &rule_with_comment.before_comments) {
        v.append(&mut code_vec)
      }
      if let Some(after_comment) = &rule_with_comment.after_comment {
        let comment = (ctx.oneline_comment_format)(after_comment.to_string());
        v.push(format!("{str} {comment}"));
        (v, true)
      } else {
        v.push(str.to_string());
        (v, false)
      }
    }
    Rule::Paren(open, child_rule_with_comment, close) => {
      let (str_lst, is_exist_after_comment) = code_format(ctx, child_rule_with_comment);
      let mut v = Vec::new();
      if let Some(mut code_vec) = before_comments_format(ctx, &rule_with_comment.before_comments) {
        v.append(&mut code_vec)
      }
      if str_lst.len() <= 1 && !is_exist_after_comment {
        if let Some(after_comment) = &rule_with_comment.after_comment {
          v.push(format!("{open}{}{close} {after_comment}", str_lst.join("")));
          (v, true)
        } else {
          v.push(format!("{open}{}{close}", str_lst.join("")));
          (v, false)
        }
      } else {
        v.push(open.to_string());
        for str in code_format(&ctx.increment_depth(), &child_rule_with_comment).0 {
          v.push(str.to_string())
        }
        if let Some(after_comment) = &rule_with_comment.after_comment {
          let comment = (ctx.oneline_comment_format)(after_comment.to_string());
          v.push(format!("{close} {comment}"));
          (v, true)
        } else {
          v.push(close.to_string());
          (v, false)
        }
      }
    }
    Rule::List(join, lst) => break_token_list(
      ctx,
      join,
      &rule_with_comment.before_comments,
      lst,
      &rule_with_comment.after_comment,
    ),
    Rule::Column(lst) => break_token_column(
      ctx,
      &rule_with_comment.before_comments,
      lst,
      &rule_with_comment.after_comment,
    ),
    _ => (vec![], false),
  }
}

/// Listルールをフォーマットする
fn break_token_list(
  ctx: &Context,
  join: &str,
  before_comments: &[String],
  lst: &[RuleWithComment],
  after_comment_opt: &Option<String>,
) -> (Vec<String>, bool) {
  let tab = ctx.indent();
  let mut is_multiline = false;
  for (i, new_rule_with_comment) in lst.iter().enumerate() {
    let (code_str_lst, is_last_exists_after_comment) =
      code_format(&ctx.set_is_lst_break_force(None), new_rule_with_comment);
    if
    // 要素の前のコメントが存在する要素が一つでもあるか、
    !new_rule_with_comment.before_comments.is_empty()
    // 最後の要素以外の要素で、要素直後のコメントが一つでも存在するか、
    || (i < lst.len() - 1 && is_last_exists_after_comment)
    // 出力結果が複数行のとき
    || code_str_lst.len() > 1
    {
      is_multiline = true;
      break;
    }
  }
  if let Some(true) = ctx.is_lst_break_force {
    is_multiline = true
  }
  let mut is_oneline_last_comment_exsits = false;
  if !is_multiline {
    let str = lst
      .iter()
      .enumerate()
      .map(|(i, child_rule_with_comment)| {
        let (code_str, is_last_exsits_after_comment) =
          code_format(&ctx.set_is_lst_break_force(None), child_rule_with_comment);
        if i == lst.len() - 1 && is_last_exsits_after_comment {
          // 最後の要素の直後にコメントがあった場合にフラグをたてる
          is_oneline_last_comment_exsits = true
        }
        // 一行であることが保障されている
        code_str.join("")
      })
      .collect::<Vec<_>>()
      .join(&format!("{join} "));
    if str.len() < ctx.len_max() {
      // 内部が一行で表せて、かつその長さが設定されている一行の長さよりも短い場合にonelineとなる
      let mut v = Vec::new();
      if let Some(mut code_vec) = before_comments_format(ctx, before_comments) {
        v.append(&mut code_vec)
      }
      if let Some(after_comment) = after_comment_opt {
        let comment = (ctx.oneline_comment_format)(after_comment.to_string());
        v.push(format!("{str} {comment}"));
        return (v, true);
      } else {
        v.push(str);
        return (v, is_oneline_last_comment_exsits);
      }
    }
  }
  let mut v = Vec::new();
  if let Some(mut code_vec) = before_comments_format(ctx, before_comments) {
    v.append(&mut code_vec)
  }
  let mut lst = lst
    .iter()
    .map(|new_rule_with_comment| {
      code_format(
        &ctx.increment_depth().set_is_lst_break_force(None),
        new_rule_with_comment,
      )
    })
    .peekable();
  loop {
    match lst.next() {
      Some((code_lst, _)) => {
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
  if let Some(after_comment) = after_comment_opt {
    v.push(after_comment.to_string());
    (v, true)
  } else {
    (v, false)
  }
}

/// 貪欲法で分割する
fn break_token_column(
  ctx: &Context,
  before_comments: &[String],
  lst: &[(RuleWithComment, ColumnConfig)],
  after_comment_opt: &Option<String>,
) -> (Vec<String>, bool) {
  let mut v = vec![];
  if let Some(mut code_vec) = before_comments_format(ctx, before_comments) {
    v.append(&mut code_vec)
  }
  let mut buf1 = String::new();
  let mut buf1_after_spaces = 1;
  let mut buf2 = String::new();
  let mut buf2_after_spaces = 1;
  let mut lst = lst.iter().peekable();
  loop {
    if let Some((rule_with_comment, config)) = lst.next() {
      let (mut str_lst, is_last_exists_after_comment) = code_format(ctx, rule_with_comment);
      if str_lst.len() > 1 || is_last_exists_after_comment {
        // 複数行
        let new_code_str = format!("{buf1}{}{buf2}", " ".repeat(buf1_after_spaces));
        v.push(new_code_str);
        v.append(&mut str_lst);
        buf1 = String::new();
        buf1_after_spaces = 1;
        buf2 = String::new();
        buf2_after_spaces = 1;
      } else {
        // 一行
        let buf1_len = buf1.len();
        let buf2_len = buf2.len();
        // 一行であることが保証されている
        let code_str = str_lst.join("");
        let code_str_len = code_str.len();
        if buf1_len + buf1_after_spaces + buf2_len + buf2_after_spaces + code_str_len
          <= ctx.len_max()
        {
          // 行長が制限を超えなかったため、そのまま一行にする
          match config.is_break {
            Some(true) => {
              // そのあとで絶対に改行
              // 更新する
              let new_code_str = format!(
                "{buf1}{}{buf2}{}{code_str}",
                " ".repeat(buf1_after_spaces),
                " ".repeat(buf2_after_spaces)
              );
              v.push(new_code_str);
              buf1 = String::new();
              buf1_after_spaces = 1;
              buf2 = String::new();
              buf2_after_spaces = 1;
            }
            Some(false) => {
              // 改行不可ポイント
              buf2.push_str(&code_str);
              buf2_after_spaces = config.space_size.unwrap_or_else(|| 1);
            }
            None => {
              // 改行可能ポイント
              // 全てbuf1に入れてbuf2を初期化
              buf1.push_str(&" ".repeat(buf1_after_spaces));
              buf1.push_str(&buf2);
              buf1.push_str(&" ".repeat(buf2_after_spaces));
              buf1.push_str(&code_str);
              buf1_after_spaces = config.space_size.unwrap_or_else(|| 1);
              buf2 = String::new();
              buf2_after_spaces = 1;
            }
          }
        } else {
          // 複数に改行しなければならない
          if buf2_len == 0 {
            // 直前が改行可能ポイントである
            let new_line_code_str = format!("{buf1}{}{buf2}", " ".repeat(buf1_after_spaces));
            v.push(new_line_code_str);
            buf1 = code_str;
            buf1_after_spaces = config.space_size.unwrap_or_else(|| 1);
            buf2 = String::new();
            buf2_after_spaces = 1;
          } else {
            // 直前が改行不可ポイントである
            if buf2_len + buf2_after_spaces + code_str_len <= ctx.len_max() {
              // buf2とcode_strをくっつけてよい
              v.push(buf1);
              let new_line_code_str = format!("{buf2}{}{code_str}", " ".repeat(buf2_after_spaces));
              v.push(new_line_code_str);
              buf1 = String::new();
              buf1_after_spaces = 1;
              buf2 = String::new();
              buf2_after_spaces = 1;
            } else {
              // buf2とcode_strをくっつけると行数オーバーする
              // はみ出す量がより少ない方を取る
              if buf1_len + buf1_after_spaces + buf2_len
                > buf2_len + buf2_after_spaces + code_str_len
              {
                // buf1とbuf2をくっつけた方がはみ出しが少ない
                let new_line_code_str = format!("{buf1}{}{buf2}", " ".repeat(buf1_after_spaces));
                v.push(new_line_code_str);
                buf1 = code_str;
                buf1_after_spaces = config.space_size.unwrap_or_else(|| 1);
                buf2 = String::new();
                buf2_after_spaces = 1;
              } else {
                // buf2とcode_strをくっつけた方がはみ出しが少ない
                v.push(buf1);
                let new_line_code_str =
                  format!("{buf2}{}{code_str}", " ".repeat(buf2_after_spaces));
                v.push(new_line_code_str);
                buf1 = String::new();
                buf1_after_spaces = 1;
                buf2 = String::new();
                buf2_after_spaces = 1;
              }
            }
          }
        }
      }
    } else {
      break;
    }
  }
  if let Some(after_comment) = after_comment_opt {
    v.push(after_comment.to_string());
    (v, true)
  } else {
    (v, false)
  }
}

fn before_comments_format(ctx: &Context, comments: &[String]) -> Option<Vec<String>> {
  if comments.is_empty() {
    None
  } else if comments.len() == 1 {
    Some(vec![(ctx.oneline_comment_format)(comments[0].clone())])
  } else {
    Some((ctx.block_comment_format)(ctx.clone(), comments.to_vec()))
  }
}
