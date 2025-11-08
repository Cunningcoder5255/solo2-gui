use cosmic::iced::time::Instant;
use cosmic::widget::segmented_button;

#[derive(Debug, Clone)]
pub enum Message {
    Wink,
    SidebarButtonPress(segmented_button::Entity),
    ReloadDevices,
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
