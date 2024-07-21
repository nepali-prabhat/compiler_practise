mod lexer;
mod straight_line_prog;

use straight_line_prog::*;


fn main() {
    let prog = AStm::Compound(
        &AStm::Compound(
            &AStm::Assign("a", &AExp::Op(&AExp::Num(5), ABinop::Plus, &AExp::Num(3))),
            &AStm::Compound(
                &AStm::Assign(
                    "b",
                    &AExp::Eseq(
                        &AStm::Print(&AExpList::Pair(
                            &AExp::Id("a"),
                            &AExpList::Last(&AExp::Op(
                                &AExp::Id("a"),
                                ABinop::Minus,
                                &AExp::Num(1),
                            )),
                        )),
                        &AExp::Op(&AExp::Num(10), ABinop::Times, &AExp::Id("a")),
                    ),
                ),
                &AStm::Print(&AExpList::Last(&AExp::Id("b"))),
            ),
        ),
        &AStm::Assign("a", &AExp::Num(50)),
    );
    let (_, context) = prog.interp(None);
    println!("response: {context:?}",);
    if let Some(context) = context {
        println!("a: {:?}", context.find("a"));
        println!("b: {:?}", context.find("b"));
        println!("c: {:?}", context.find("c"));
    }
}
