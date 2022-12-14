use core::panic;

use crate::{
  dynamic::{
    AfterComment, BeforeComments, CloseRule, Data, InternalRule, ListedRule, OnHoldInfo, OpenRule,
  },
  ColumnConfig, Context,
};

pub fn code_format(ctx: &Context, data: &mut Data) -> Vec<String> {
  let internal_rules = match &data.process_tag {
    Some(tag) => match data.clone().tag_data.get(tag) {
      Some(rules) => rules.clone(),
      None => panic!(),
    },
    None => data.clone().root,
  };
  let mut v = vec![];
  let mut count = 0;
  let max = internal_rules.rules.len();
  while count < max {
    let listed_rule = &internal_rules.rules[count];
    match listed_rule {
      ListedRule::Raw(str) => {
        count += 1;
        v.push(str.to_string())
      }
      // 子要素があるため処理を行う
      // 基本的にここで処理を完結させるつもりで
      ListedRule::Open(OpenRule::Paren(None, open_str, before_comments)) => {
        count += 1;
        let len_max = ctx.len_max();
        let mut is_multiline_opt = None;
        let mut tmp_s = String::new();
        let mut tmp_count = count;
        // 括弧の中の要素が
        // - 複数行
        // - 一行
        // - 判定不能
        // のどれになるかを判定する
        match &internal_rules.rules[count] {
          ListedRule::Unconfirmed(_) => {
            // 判定不能
            is_multiline_opt = None;
          }
          ListedRule::Raw(str) => {
            if str.len() > len_max {
              // 文字の長さが許される長さよりも長いため、複数行になることが確定
              is_multiline_opt = Some(true)
            } else {
              //
            }
          }
          ListedRule::Open(OpenRule::Contents(before_comments)) if !before_comments.is_empty() => {
            // トークンの前にコメントがある時点で複数行が確定
            is_multiline_opt = Some(true);
          }
          _ => (),
        }
      }
      ListedRule::Close(CloseRule::Paren(close_str, after_comment_opt)) => {
        count += 1;
      }
      ListedRule::Open(OpenRule::List(None, join)) => {
        count += 1;
      }
      ListedRule::Close(CloseRule::List) => {
        count += 1;
      }
      ListedRule::Open(OpenRule::Column(None)) => {
        count += 1;
      }
      ListedRule::Close(CloseRule::Column) => {
        count += 1;
      }
      ListedRule::Open(OpenRule::Contents(before_comments)) => {
        count += 1;
      }
      ListedRule::Close(CloseRule::Contents(after_comment_opt)) => {
        count += 1;
      }
      ListedRule::Open(OpenRule::ColumnContents(column_config, begore_comments)) => {
        count += 1;
      }
      ListedRule::Close(CloseRule::ColumnContents(after_comment_opt)) => {
        count += 1;
      }
      // 未確定要素なので終了する
      ListedRule::Unconfirmed(_) => break,
      ListedRule::Open(OpenRule::Paren(_, _, _)) => break,
      ListedRule::Open(OpenRule::List(_, _)) => break,
      ListedRule::Open(OpenRule::Column(_)) => break,
    }
  }
  let new_listedrule_lst = internal_rules
    .rules
    .iter()
    .skip(count)
    .cloned()
    .collect::<Vec<_>>();
  // 情報のアップデートを行う
  match &data.process_tag {
    Some(tag) => {
      data.tag_data.insert(
        tag.to_string(),
        InternalRule {
          rules: new_listedrule_lst,
        },
      );
    }
    None => {
      data.root = InternalRule {
        rules: new_listedrule_lst,
      }
    }
  }

  v
}
