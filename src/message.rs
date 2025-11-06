use iced::time::Instant;

#[derive(Debug, Clone)]
pub enum Message {
    Wink,
    AdminButtonPress,
    ReloadDevices,
    OathButtonPress,
    TOTPLabelPress(String /* Label */),
    CopyTOTP(String /* Label */),
    OathTOTPLifeRefresh(Instant),
    AddTOTPScreen,
    AddTOTP,
    CancelAddingTOTP,
    DeleteTOTP(String /* Label */),
    UpdateLabelInput(String /* Label */),
    UpdateSecretInput(String /* Label */),
}
