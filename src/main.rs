mod straight_line_prog;

use straight_line_prog::*;

fn interp_stm(stm: &AStm){
    println!("stm: {:?}", stm);
}

fn main() {
    let prog = AStm::Compound(
        &AStm::Assign("a", &AExp::Op(&AExp::Num(5), ABinop::Plus, &AExp::Num(3))),
        &AStm::Compound(
            &AStm::Assign(
                "b",
                &AExp::Eseq(
                    &AStm::Print(&AExpList::Pair(
                        &AExp::Id("a"),
                        &AExpList::Last(&AExp::Op(&AExp::Id("a"), ABinop::Minus, &AExp::Num(1))),
                    )),
                    &AExp::Op(&AExp::Num(10), ABinop::Times, &AExp::Id("a")),
                ),
            ),
            &AStm::Print(&AExpList::Last(&AExp::Id("b"))),
        ),
    );
    interp_stm(&prog);
}
