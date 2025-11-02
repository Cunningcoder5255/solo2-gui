#[derive(Debug, Clone)]
pub enum Message {
    OathButtonPress,
    TOTPLabelPress(String /* Label */),
}
