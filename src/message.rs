use iced::time::Instant;

#[derive(Debug, Clone)]
pub enum Message {
    OathButtonPress,
    TOTPLabelPress(String /* Label */),
    OathTOTPLifeRefresh(Instant),
}
