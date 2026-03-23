pub(super) fn require_controller() {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        ic_cdk::api::trap("unauthorized: controller only");
    }
}
