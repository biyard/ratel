use super::*;

pub fn use_space_poll() -> Loader<PollResponse> {
    use_space_poll_context().poll
}
