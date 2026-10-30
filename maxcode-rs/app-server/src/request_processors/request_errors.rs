use super::*;
use maxcode_protocol::error::MaxcodeErrorDetails;

pub(super) fn environment_selection_error(err: MaxcodeErr) -> JSONRPCErrorError {
    match err.details() {
        MaxcodeErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
