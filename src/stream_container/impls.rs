use stream_container::{StreamContainer};

use std::vec;

macro_rules! try_option 
{
    {$x:expr} => 
    {
        match $x
        {
            Some(x) => x,
            None => return None,
        }
    };
}

macro_rules! dummy 
{
    ($x:expr) => ();
}

macro_rules! expr_arr
{
    [$x: expr; $($ns:tt),*] => {[$({dummy!($ns); $x}),*]};
}

macro_rules! array_impl_stream_container 
{
    {$n:expr, $($ns:expr),+} => 
    {
        impl<T> StreamContainer<T> for [T; $n]
        {
            type Iter = vec::IntoIter<T>;
            fn fill_with<I: Iterator<Item = T>> (stream: &mut I) -> Option<Self>
            {
                Some(expr_arr![
                     try_option!(stream.next()); 
                     $($ns),+])
            }
            fn into_stream(self) -> Self::Iter
              {<Box<[T]> as Into<Vec<T>>>::into(Box::new(self)).into_iter()}
        }

        array_impl_stream_container!{$($ns),+}
    };

    {$n:expr} => {
        impl<T> StreamContainer<T> for [T; $n]
        {
            type Iter = vec::IntoIter<T>;
            fn fill_with<I: Iterator<Item = T>> (stream: &mut I) -> Option<Self>
              {Some([])}
            fn into_stream(self) -> Self::Iter
              {Vec::new().into_iter()}
        }
    };
}

array_impl_stream_container!
{
                                /*32, 31, 30,
    29, 28, 27, 26, 25, 24, 23, 22, 21, 20,
    19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
     9,  8,  7,  6,  5,  4,  3,  2, */ 1,  0
}
