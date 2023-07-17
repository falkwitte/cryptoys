pub trait ToString{
    fn to_string(&self) -> String;    
}

/// Solves a Ciphtertext
pub trait Solve{
    fn solve(&self) -> String;
}