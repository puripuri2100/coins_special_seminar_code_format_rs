use crate::{ColumnConfig, Context};
use core::panic;
use std::{collections::HashMap, hash::Hash};

pub type Tag = String;

pub type IsConfirmed = bool;

pub type BeforeComments = Vec<String>;
pub type AfterComment = Option<String>;

#[derive(Clone, Debug)]
pub enum ListedRule {
  Unconfirmed(Tag),
  Link(String),
  Raw(String),
  Open(OpenRule),
  Close(CloseRule),
}

#[derive(Debug, Clone)]
enum OpenRule {
  Paren(Option<Tag>, String, BeforeComments),
  List(Option<Tag>, String),
  Column(Option<Tag>),
  ColumnContents(ColumnConfig, BeforeComments),
  Contents(BeforeComments),
}

#[derive(Debug, Clone)]
enum CloseRule {
  Paren(String, AfterComment),
  List,
  Column,
  ColumnContents(AfterComment),
  Contents(AfterComment),
}

#[derive(Clone, Debug)]
pub enum Rule {
  Unconfirmed(Tag),
  AST(Box<RuleWithComment>),
  Raw(String),
  //-------!!!!!!!!!!!!!!!!!!!!!!!!!!
  // Tagの有無とcontentsの有無は連動する気がする
  // Tagがある→コンテンツは無い、もしくはアップデートされるものであることが期待される
  // Tagがない→コンテンツはアップデートされない確定したものが存在しなければならない
  //
  // Tagがあって且つコンテンツが存在する場合はtag_dataにデータ入れないておかないといけない
  /// Tag無しコンテンツ無しは実行時エラーで良いのでは
  //-------!!!!!!!!!!!!!!!!!!!!!!!!!!
  List(Option<Tag>, String, Vec<RuleWithComment>),
  Paren(Option<Tag>, String, Box<RuleWithComment>, String),
  Column(Option<Tag>, Vec<(RuleWithComment, ColumnConfig)>),
}

#[derive(Clone, Debug)]
pub struct RuleWithComment {
  pub before_comments: Vec<String>,
  pub rule: Rule,
  pub after_comment: Option<String>,
}

fn is_not_with_comment(r: &RuleWithComment) -> bool {
  r.before_comments.is_empty() && r.after_comment.is_none()
}

fn with_comment(r: &Rule) -> RuleWithComment {
  RuleWithComment {
    before_comments: vec![],
    rule: r.clone(),
    after_comment: None,
  }
}

type FormattedRuleStack = Vec<OpenRule>;

#[derive(Clone, Debug)]
struct InternalRule {
  rules: Vec<ListedRule>,
}

#[derive(Clone, Debug)]
pub struct Data {
  tag_data: HashMap<Tag, InternalRule>,
  stack: FormattedRuleStack,
}

fn merge_hash_map<K, V>(base: &mut HashMap<K, V>, add: &HashMap<K, V>) -> HashMap<K, V>
where
  K: Clone + PartialEq + Eq + Hash,
  V: Clone,
{
  for (k, v) in add.iter() {
    base.insert(k.clone(), v.clone());
  }
  base.clone()
}

fn rule_to_listedrule(rule: &Rule) -> (Vec<ListedRule>, HashMap<String, InternalRule>) {
  let mut lst = vec![];
  let mut base_hashmap = HashMap::new();
  match rule {
    Rule::Unconfirmed(tag) => {
      lst.push(ListedRule::Unconfirmed(tag.clone()));
    }
    Rule::AST(rule_with_comment) => {
      lst.push(ListedRule::Open(OpenRule::Contents(
        rule_with_comment.clone().before_comments,
      )));
      let (mut rule_lst, new_tag_data) = rule_to_listedrule(&rule_with_comment.rule);
      merge_hash_map(&mut base_hashmap, &new_tag_data);
      lst.append(&mut rule_lst);
      lst.push(ListedRule::Close(CloseRule::Contents(
        rule_with_comment.clone().after_comment,
      )));
    }
    Rule::Raw(s) => lst.push(ListedRule::Raw(s.to_string())),
    Rule::List(tag_opt, join, contents) => {
      lst.push(ListedRule::Open(OpenRule::List(
        tag_opt.clone(),
        join.to_string(),
      )));
      let mut listed_rules = vec![];
      for content in contents.iter() {
        let (mut rule_lst, new_tag_data) = rule_to_listedrule(&content.rule);
        merge_hash_map(&mut base_hashmap, &new_tag_data);
        listed_rules.push(ListedRule::Open(OpenRule::Contents(
          content.clone().before_comments,
        )));
        listed_rules.append(&mut rule_lst);
        listed_rules.push(ListedRule::Close(CloseRule::Contents(
          content.clone().after_comment,
        )));
      }
      match tag_opt {
        Some(tag) => {
          base_hashmap.insert(
            tag.to_string(),
            InternalRule {
              rules: listed_rules,
            },
          );
        }
        None => lst.append(&mut listed_rules),
      }
      lst.push(ListedRule::Close(CloseRule::List));
    }
    Rule::Paren(tag_opt, open_str, content, close_str) => {
      lst.push(ListedRule::Open(OpenRule::Paren(
        tag_opt.clone(),
        open_str.to_string(),
        content.clone().before_comments,
      )));
      let (mut rule_lst, add_data) = rule_to_listedrule(&content.rule);
      merge_hash_map(&mut base_hashmap, &add_data);
      match tag_opt {
        Some(tag) => {
          base_hashmap.insert(
            tag.to_string(),
            InternalRule {
              rules: rule_lst.to_vec(),
            },
          );
        }
        None => lst.append(&mut rule_lst),
      }
      lst.push(ListedRule::Close(CloseRule::Paren(
        close_str.clone(),
        content.clone().after_comment,
      )));
    }
    Rule::Column(tag_opt, contents) => {
      lst.push(ListedRule::Open(OpenRule::Column(tag_opt.clone())));
      for (rule_with_comment, config) in contents.iter() {
        lst.push(ListedRule::Open(OpenRule::ColumnContents(
          config.clone(),
          rule_with_comment.clone().before_comments,
        )));
        let (mut rule_lst, add_data) = rule_to_listedrule(&rule_with_comment.rule);
        merge_hash_map(&mut base_hashmap, &add_data);
        match tag_opt {
          Some(tag) => {
            base_hashmap.insert(
              tag.to_string(),
              InternalRule {
                rules: rule_lst.to_vec(),
              },
            );
          }
          None => lst.append(&mut rule_lst),
        }
        lst.push(ListedRule::Close(CloseRule::ColumnContents(
          rule_with_comment.clone().after_comment,
        )));
      }
      lst.push(ListedRule::Close(CloseRule::Column));
    }
  };
  (lst.to_vec(), base_hashmap)
}

/// リンクしている場所などをすべて一つのリストにつぶす
fn flat_listedrule(
  listed_rules: &[ListedRule],
  tag_data: &HashMap<Tag, InternalRule>,
) -> Vec<ListedRule> {
  let mut v = vec![];
  for listed_rule in listed_rules.iter() {
    match listed_rule {
      ListedRule::Link(tag) => {
        let internal_rule_opt = tag_data.get(tag);
        if let Some(internal_rule) = internal_rule_opt {
          let mut children = flat_listedrule(&internal_rule.rules, tag_data);
          v.append(&mut children)
        }
      }
      ListedRule::Open(OpenRule::List(Some(tag), join)) => {
        v.push(ListedRule::Open(OpenRule::List(None, join.to_string())));
        let internal_rule_opt = tag_data.get(tag);
        if let Some(internal_rule) = internal_rule_opt {
          let mut children = flat_listedrule(&internal_rule.rules, tag_data);
          v.append(&mut children)
        }
      }
      ListedRule::Open(OpenRule::Paren(Some(tag), open_str, comments)) => {
        v.push(ListedRule::Open(OpenRule::Paren(
          None,
          open_str.to_string(),
          comments.clone(),
        )));
        let internal_rule_opt = tag_data.get(tag);
        if let Some(internal_rule) = internal_rule_opt {
          let mut children = flat_listedrule(&internal_rule.rules, tag_data);
          v.append(&mut children)
        }
      }
      ListedRule::Open(OpenRule::Column(Some(tag))) => {
        v.push(ListedRule::Open(OpenRule::Column(None)));
        let internal_rule_opt = tag_data.get(tag);
        if let Some(internal_rule) = internal_rule_opt {
          let mut children = flat_listedrule(&internal_rule.rules, tag_data);
          v.append(&mut children)
        }
      }
      _ => v.push(listed_rule.clone()),
    }
  }
  v
}

/// hashmapを使って分散しているルールを一つのリストにつぶされたことを前提に木構造化する
/// 元のリストをかなり信頼してよく、チェックもあまり行わず、パニックしてよい
fn listedrule_to_rule(
  listed_rules: &[ListedRule],
  count: usize,
) -> (RuleWithComment, Option<ColumnConfig>, usize) {
  match listed_rules.get(count) {
    Some(ListedRule::Raw(str)) => (with_comment(&Rule::Raw(str.to_string())), None, count + 1),
    Some(ListedRule::Unconfirmed(tag)) => (
      with_comment(&Rule::Unconfirmed(tag.to_string())),
      None,
      count + 1,
    ),
    Some(ListedRule::Open(OpenRule::List(_, join))) => {
      let mut v = vec![];
      let mut c = count + 1;
      loop {
        match listed_rules.get(c) {
          Some(ListedRule::Close(CloseRule::List)) => break,
          _ => {
            let (rule_with_comment, _, new_c) = listedrule_to_rule(listed_rules, c);
            v.push(rule_with_comment);
            c = new_c;
          }
        }
      }
      let rule = Rule::List(None, join.to_string(), v);
      (with_comment(&rule), None, c + 1)
    }
    Some(ListedRule::Open(OpenRule::Paren(_, open_str, before_comments))) => {
      let (rule_with_comment, _, count) = listedrule_to_rule(listed_rules, count + 1);
      match listed_rules.get(count) {
        Some(ListedRule::Close(CloseRule::Paren(close_str, after_comment))) => {
          let rule_with_comment = RuleWithComment {
            before_comments: before_comments.clone(),
            rule: rule_with_comment.rule,
            after_comment: after_comment.clone(),
          };
          let rule = Rule::Paren(
            None,
            open_str.to_string(),
            Box::new(rule_with_comment),
            close_str.clone(),
          );
          (with_comment(&rule), None, count + 1)
        }
        _ => unreachable!(),
      }
    }
    Some(ListedRule::Open(OpenRule::Contents(before_comments))) => {
      let (rule_with_comment, _, count) = listedrule_to_rule(listed_rules, count + 1);
      match listed_rules.get(count) {
        Some(ListedRule::Close(CloseRule::Contents(after_comment))) => {
          let rule_with_comment = RuleWithComment {
            before_comments: before_comments.clone(),
            rule: rule_with_comment.rule,
            after_comment: after_comment.clone(),
          };
          let rule = Rule::AST(Box::new(rule_with_comment));
          (with_comment(&rule), None, count + 1)
        }
        _ => unreachable!(),
      }
    }
    Some(ListedRule::Open(OpenRule::ColumnContents(column_config, before_comments))) => {
      let (rule_with_comment, _, count) = listedrule_to_rule(listed_rules, count + 1);
      match listed_rules.get(count) {
        Some(ListedRule::Close(CloseRule::ColumnContents(after_comment))) => {
          let rule_with_comment = RuleWithComment {
            before_comments: before_comments.clone(),
            rule: rule_with_comment.rule,
            after_comment: after_comment.clone(),
          };
          (rule_with_comment, Some(column_config.clone()), count + 1)
        }
        _ => unreachable!(),
      }
    }
    Some(ListedRule::Open(OpenRule::Column(_))) => {
      let mut v = vec![];
      let mut c = count + 1;
      loop {
        match listed_rules.get(c) {
          Some(ListedRule::Close(CloseRule::Column)) => break,
          _ => {
            let (rule_with_comment, column_config_opt, new_c) = listedrule_to_rule(listed_rules, c);
            match column_config_opt {
              Some(config) => {
                v.push((rule_with_comment, config));
                c = new_c;
              }
              None => panic!(),
            }
          }
        }
      }
      let rule = Rule::Column(None, v);
      (with_comment(&rule), None, c + 1)
    }
    _ => unreachable!(),
  }
}

impl Data {
  /// 新規データを木構造から生成する
  /// "root"が予約されており、そこを起点に探索やプリントが行われる
  pub fn new(rule: &Rule) -> Self {
    let mut tag_data = HashMap::new();
    let (rules, new_data) = rule_to_listedrule(rule);
    let internal_rule = InternalRule { rules };
    merge_hash_map(&mut tag_data, &new_data);
    tag_data.insert("root".to_string(), internal_rule);
    let stack = Vec::new();
    Data { tag_data, stack }
  }

  /// 値を挿入する
  /// すでにタグがある場合はエラーを返し、上書きはしない
  pub fn insert(&mut self, tag: &str, rule: &Rule) {
    let (rules, new_data) = rule_to_listedrule(rule);
    let internal_rule = InternalRule { rules };
    merge_hash_map(&mut self.tag_data, &new_data);
    match self.tag_data.get(tag) {
      Some(_) => {
        panic!()
      }
      None => {
        self.tag_data.insert(tag.to_string(), internal_rule);
      }
    }
  }

  /// タグにすでにある値を上書きする
  pub fn replace(&mut self, tag: &str, rule: &Rule) {
    let (rules, new_data) = rule_to_listedrule(rule);
    let internal_rule = InternalRule { rules };
    merge_hash_map(&mut self.tag_data, &new_data);
    self.tag_data.insert(tag.to_string(), internal_rule);
  }

  /// タグの先にある値を木構造の形で取り出す
  /// これとreplaceを組み合わせることで「安全に」値の更新を行うことができる
  pub fn get(&mut self, tag: &str) -> Option<RuleWithComment> {
    let listed_rules_opt = self.tag_data.get(tag);
    let flatted_listed_rules_opt =
      listed_rules_opt.map(|i| flat_listedrule(&i.rules, &self.tag_data));
    match flatted_listed_rules_opt {
      Some(l) => {
        let (rule, column_config_opt, count) = listedrule_to_rule(&l, 0);
        if count == l.len() && column_config_opt.is_none() {
          Some(rule)
        } else {
          None
        }
      }
      None => None,
    }
  }
  /// 値を確定させる
  /// 内部の実装としては`Unconfirmed(Tag)`を`Link(Tag)`にし、`Some(Tag)`を`None`にする
  /// タグは重複しないことが保証されている
  /// 最初は"root"で検索を行うが、リンクが存在する場合はリンク先まで追っていく。
  pub fn confirmed(&mut self, tag: &str) {
    let _ = self.confirmed_with_tag("root", tag);
  }

  /// 引数はそれぞれ
  /// 1. 更新できるデータセット
  /// 2. 今作業しているデータのタグの名前
  /// 3. 値を確定させたい対象のタグの名前
  /// となっている
  fn confirmed_with_tag(&mut self, tag: &str, target_tag_name: &str) -> bool {
    let mut new_rules = vec![];
    let mut is_confirmed = false;
    let internal_rules = self.tag_data.get(tag).unwrap().clone().rules;
    for listed_rule in internal_rules.iter() {
      if is_confirmed {
        new_rules.push(listed_rule.clone())
      } else {
        match listed_rule {
          // 目的のタグが発見できたので、更新して終了
          ListedRule::Unconfirmed(unconfirmed_tag_name)
            if unconfirmed_tag_name == target_tag_name =>
          {
            new_rules.push(ListedRule::Link(tag.to_string()));
            is_confirmed = true;
          }
          ListedRule::Open(OpenRule::Paren(Some(open_tag_name), open_str, comments))
            if open_tag_name == target_tag_name =>
          {
            new_rules.push(ListedRule::Open(OpenRule::Paren(
              None,
              open_str.clone(),
              comments.clone(),
            )));
            new_rules.push(ListedRule::Link(open_tag_name.clone()));
            is_confirmed = true;
          }
          ListedRule::Open(OpenRule::List(Some(open_tag_name), join))
            if open_tag_name == target_tag_name =>
          {
            new_rules.push(ListedRule::Open(OpenRule::List(None, join.clone())));
            new_rules.push(ListedRule::Link(open_tag_name.clone()));
            is_confirmed = true;
          }
          ListedRule::Open(OpenRule::Column(Some(open_tag_name)))
            if open_tag_name == target_tag_name =>
          {
            new_rules.push(ListedRule::Open(OpenRule::Column(None)));
            new_rules.push(ListedRule::Link(open_tag_name.clone()));
            is_confirmed = true;
          }
          // 目標とするタグ名ではなかったため、リンク先のルールを見に行き、
          // そこに目標があったら終了
          ListedRule::Unconfirmed(unconfirmed_tag_name) => {
            let new_is_confirmed = self.confirmed_with_tag(unconfirmed_tag_name, target_tag_name);
            new_rules.push(ListedRule::Unconfirmed(unconfirmed_tag_name.to_string()));
            if new_is_confirmed {
              is_confirmed = new_is_confirmed
            }
          }
          ListedRule::Link(linked_tag_name) => {
            let new_is_confirmed = self.confirmed_with_tag(linked_tag_name, target_tag_name);
            new_rules.push(ListedRule::Link(linked_tag_name.to_string()));
            if new_is_confirmed {
              is_confirmed = new_is_confirmed
            }
          }
          ListedRule::Open(OpenRule::Paren(Some(linked_tag_name), open_str, comments)) => {
            let new_is_confirmed = self.confirmed_with_tag(linked_tag_name, target_tag_name);
            new_rules.push(ListedRule::Open(OpenRule::Paren(
              Some(linked_tag_name.to_string()),
              open_str.to_string(),
              comments.clone(),
            )));
            if new_is_confirmed {
              is_confirmed = new_is_confirmed
            }
          }
          ListedRule::Open(OpenRule::List(Some(linked_tag_name), join)) => {
            let new_is_confirmed = self.confirmed_with_tag(linked_tag_name, target_tag_name);
            new_rules.push(ListedRule::Open(OpenRule::List(
              Some(linked_tag_name.to_string()),
              join.to_string(),
            )));
            if new_is_confirmed {
              is_confirmed = new_is_confirmed
            }
          }
          ListedRule::Open(OpenRule::Column(Some(linked_tag_name))) => {
            let new_is_confirmed = self.confirmed_with_tag(linked_tag_name, target_tag_name);
            new_rules.push(ListedRule::Open(OpenRule::Column(Some(
              linked_tag_name.to_string(),
            ))));
            if new_is_confirmed {
              is_confirmed = new_is_confirmed
            }
          }
          // 以上以外の何も無い普通の要素はそのまま追加
          _ => new_rules.push(listed_rule.clone()),
        }
      }
    }
    // 目標が見つかっても見つからなくてもデータは更新しておく
    let new_internal_rule = InternalRule { rules: new_rules };
    self.tag_data.insert(tag.to_string(), new_internal_rule);
    is_confirmed
  }

  /// コードフォーマット
  pub fn format(&mut self, ctx: Context) -> Vec<String> {
    let mut code_lst = vec![];
    let root = self.tag_data.get("root").unwrap();
    let root_rules = root.clone().rules;
    for root_rule in root_rules.iter() {}
    code_lst
  }
}
