pub use rcket_macros::Node;

pub trait Node {
    type Output;
    fn parse(input: &str) -> Option<Self::Output>;
}
