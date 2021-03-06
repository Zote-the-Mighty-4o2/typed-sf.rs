/// Type-level enum of [`True`] and [`False`].
pub trait Bit {
    /// Result of negation of bit
    type Neg: Bit;

    /// `Real`, represented with rust's objects value of type.
    fn val() -> bool;

    /// `Real` value of type.
    fn new() -> Self;
}

/// Type-level True value, [`Bit`].
#[derive(Debug)]
pub struct True;

/// Type-level False value, [`Bit`].
#[derive(Debug)]
pub struct False;

impl Bit for True {
    type Neg = False;

    #[inline]
    fn val() -> bool {
        true
    }

    #[inline]
    fn new() -> Self {
        True
    }
}

impl Bit for False {
    type Neg = True;

    #[inline]
    fn val() -> bool {
        false
    }

    #[inline]
    fn new() -> Self {
        False
    }
}
