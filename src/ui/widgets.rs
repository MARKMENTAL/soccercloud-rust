use crate::instance::SimStatus;

pub fn status_badge(status: &SimStatus) -> &'static str {
    match status {
        SimStatus::Pending => "PENDING",
        SimStatus::Running { .. } => "RUNNING",
        SimStatus::Completed => "COMPLETED",
    }
}
