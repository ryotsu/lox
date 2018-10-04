use std::fmt::Display;

macro_rules! match_next {
    ( $iter:expr, $( $match:expr ),+ ) => {
        if Some(true) == $iter.peek().map(|i| $( i == &$match )||+ ) {
            $iter.next();
            Some(true)
        } else {
            Some(false)
        }
    };
}

macro_rules! check_next {
    ( $iter:expr, $( $match:expr ),+ ) => {
        $iter.peek().map(|i| $( i == &$match )||+ )
    };
}

macro_rules! check_func {
    ( $iter:expr, $( $func:ident ),+ ) => {
        if Some(true) == $iter.peek().map(|&i| $( $func(i) )||+ ) {
            true
        } else {
            false
        }
    };
}

pub fn error<T: Display>(line: usize, offset: usize, message: T) {
    eprintln!("L{}:{} {}", line, offset, message);
}
