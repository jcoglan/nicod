#[macro_export]
macro_rules! var {
    ($n:ident) => {
        Variable::new(stringify!($n))
    };
}

#[macro_export]
macro_rules! expr {
    (var($name:ident)) => {
        expr::var(stringify!($name))
    };
    (wrd($name:ident)) => {
        expr::word(stringify!($name))
    };
    (seq($( $n:ident $a:tt ),+)) => {
        expr::seq(&[$( expr!($n $a) ),+])
    };
    (lst([$( $elem:tt )*])) => {
        expr!(lst(__tag, [$( $elem )*]))
    };
    (lst($tag:ident, [])) => {
        expr::null(stringify!($tag))
    };
    (lst($tag:ident, [$n:ident $a:tt , $( $rest:tt )*])) => {
        expr!(@list $tag $n $a expr!(lst($tag, [$( $rest )*])))
    };
    (lst($tag:ident, [$n:ident $a:tt | $( $rest:tt )*])) => {
        expr!(@list $tag $n $a expr!($( $rest )*))
    };
    (@list $tag:ident $n:ident $a:tt $( $e:tt )*) => {
        expr::pair(stringify!($tag), expr!($n $a), $( $e )*)
    };
}

#[macro_export]
macro_rules! unify {
    ($xn:ident $xa:tt, $yn:ident $ya:tt) => {
        State::new().unify(&expr!($xn $xa), &expr!($yn $ya))
    };
}
