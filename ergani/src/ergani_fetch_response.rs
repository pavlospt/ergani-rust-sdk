use bon::Builder;

use crate::auth::authenticator::ErganiAuthenticationState;

#[derive(Builder)]
pub struct ErganiFetchResponse<T> {
    response: Option<T>,
    auth_state: ErganiAuthenticationState,
}

impl<T> ErganiFetchResponse<T> {
    pub fn response(&self) -> Option<&T> {
        self.response.as_ref()
    }

    pub fn auth_state(&self) -> &ErganiAuthenticationState {
        &self.auth_state
    }
}
