#![allow(dead_code)]
use std::io::{stdout, Write};

#[derive(Debug)]
pub(crate) enum ABinop {
    Times,
    Div,
    Plus,
    Minus,
}
impl ABinop {
    fn apply(&self, l: u32, r: u32) -> u32 {
        match self {
            ABinop::Times => l * r,
            ABinop::Div => l / r,
            ABinop::Plus => l + r,
            ABinop::Minus => l - r,
        }
    }
}

#[derive(Debug)]
pub(crate) enum AExp<'a> {
    Id(&'a str),
    Num(u32),
    Op(&'a AExp<'a>, ABinop, &'a AExp<'a>),
    Eseq(&'a AStm<'a>, &'a AExp<'a>),
}

#[derive(Debug)]
pub(crate) enum AExpList<'a> {
    Pair(&'a AExp<'a>, &'a AExpList<'a>),
    Last(&'a AExp<'a>),
}

impl AExpList<'_> {
    fn collect_values<'a, 'b>(&'a self, context: Option<Box<Context<'b>>>, collector: &mut Vec<u32>) -> Option<Box<Context>> where 'b :'a {
        match self {
            AExpList::Pair(exp, next_list) => {
                let (val, context) = exp.interp(context);
                let val = val.expect("print arg must have some value");
                collector.push(val);
                next_list.collect_values(context, collector)
            }
            AExpList::Last(exp) => {
                let (val, context) = exp.interp(context);
                let val = val.expect("print arg must have some value");
                collector.push(val);
                context
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum AStm<'a> {
    Compound(&'a AStm<'a>, &'a AStm<'a>),
    Assign(&'a str, &'a AExp<'a>),
    Print(&'a AExpList<'a>),
}

#[derive(Debug)]
pub(crate) struct Context<'a> {
    value: (&'a str, u32),
    next: Option<Box<Context<'a>>>,
}

impl Context<'_> {
    pub(crate) fn find(&self, id: &str) -> Option<u32> {
        if self.value.0 == id {
            return Some(self.value.1);
        } else if let Some(ctx) = self.next.as_ref() {
            ctx.find(id)
        } else {
            None
        }
    }
}

pub(crate) trait Interp {
    fn interp<'a, 'b>(&'a self, context: Option<Box<Context<'b>>>) -> (Option<u32>, Option<Box<Context>>) where 'b:'a;
}

impl Interp for AExp<'_> {
    fn interp<'a, 'b>(&'a self, context: Option<Box<Context<'b>>>) -> (Option<u32>, Option<Box<Context>>) where 'b:'a {
        match self {
            AExp::Id(id) => {
                let v = context.as_ref().map_or(None, |v| v.find(id));
                (v, context)
            },
            AExp::Num(n) => (Some(*n), context),
            AExp::Op(e1, op, e2) => {
                let (l, context) = e1.interp(context);
                let (r, context) = e2.interp(context);
                let l = l.expect("left value required");
                let r = r.expect("right value required");
                (Some(op.apply(l, r)), context)
            }
            AExp::Eseq(stm, exp) => {
                let (_, context) = stm.interp(context);
                exp.interp(context)
            },
        }
    }
}

impl Interp for AStm<'_> {
    fn interp<'a, 'b>(&'a self, context: Option<Box<Context<'b>>>) -> (Option<u32>, Option<Box<Context>>) where 'b:'a {
        match self {
            AStm::Compound(stm1, stm2) => {
                let (_, context) = stm1.interp(context);
                let (_, context) = stm2.interp(context);
                (None, context)
            }
            AStm::Assign(id, exp) => {
                let (val, context) = exp.interp(context);
                let val = val.expect("value required to assign");
                let context = Some(Box::new(Context {
                    value: (id, val),
                    next: context 
                }));
                (None, context)
            }
            AStm::Print(exp_list) => {
                let mut collector: Vec<u32> = Vec::new();
                let context = exp_list.collect_values(context, &mut collector);
                let print_str = collector.iter().map(|v| format!("{v}")).reduce(|acc, v| format!("{acc} {v}") );
                if let Some(str) = print_str {
                    println!("{str}");
                }
                (None, context)
            },
        }
    }
}
