#[macro_export]
macro_rules! var {
    ($n:ident) => {
        Variable(std::rc::Rc::new(String::from(stringify!($n))))
    };
}

#[macro_export]
macro_rules! expr {
    (@wrap $t:ident $( $e:tt )*) => {
        std::rc::Rc::new(Expr::$t($( $e )*))
    };
    (var($x:ident)) => {
        expr!(@wrap Var var!($x))
    };
    (wrd($x:tt)) => {
        expr!(@wrap Wrd Word(String::from(stringify!($x))))
    };
    (seq($( $n:ident $a:tt ),+)) => {
        expr!(@wrap Seq Sequence(vec![$( expr!($n $a) ),+]))
    };
    (lst([$( $elem:tt )*])) => {
        expr!(lst(__tag, [$( $elem )*]))
    };
    (lst($tag:ident, $items:tt)) => {
        expr!(@wrap Lst List {
            tag: std::rc::Rc::new(String::from(stringify!($tag))),
            pair: expr!(@list_items $tag $items),
        })
    };
    (@list_items $tag:ident []) => {
        None
    };
    (@list_items $tag:ident [$n:ident $a:tt $( $rest:tt )*]) => {
        Some(Pair {
            head: expr!($n $a),
            tail: expr!(@list_tail $tag $( $rest )*),
        })
    };
    (@list_tail $tag:ident , $( $rest:tt )*) => {
        expr!(lst($tag, [$( $rest )*]))
    };
    (@list_tail $tag:ident | $( $rest:tt )*) => {
        expr!($( $rest )*)
    };
}

#[macro_export]
macro_rules! unify {
    ($xn:ident $xa:tt, $yn:ident $ya:tt) => {
        State::new().unify((0, &expr!($xn $xa)), (0, &expr!($yn $ya)))
    };
}
