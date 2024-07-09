#![allow(dead_code)]

#[derive(Debug)]
pub(crate) enum ABinop {
    Times,
    Div,
    Plus,
    Minus,
}

#[derive(Debug)]
pub(crate) enum AExp<'a> {
    Id(&'a str),
    Num(i32),
    Op(&'a AExp<'a>, ABinop, &'a AExp<'a>),
    Eseq(&'a AStm<'a>, &'a AExp<'a>),
}

#[derive(Debug)]
pub(crate) enum AExpList<'a> {
    Pair(&'a AExp<'a>, &'a AExpList<'a>),
    Last(&'a AExp<'a>),
}

#[derive(Debug)]
pub(crate) enum AStm<'a> {
    Compound(&'a AStm<'a>, &'a AStm<'a>),
    Assign(&'a str, &'a AExp<'a>),
    Print(&'a AExpList<'a>),
}
