pub struct Label(pub String);
impl From<String> for Label {
    fn from(s: String) -> Self { Self(s) }
}
