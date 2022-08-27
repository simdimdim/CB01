use std::fmt::Debug;

pub type NameType = &'static str;
pub trait NameExtractor {
    type OutputName = NameType;
    fn name(&self) -> Self::OutputName;
}
impl<T: Debug> Debug for dyn NameExtractor<OutputName = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name:{{name:{:?}}}", self.name()))
    }
}
