use crate::{
    app_error,
    types::{AppError, AppErrorCode, AppResult},
};
use serde_json::json;
use validator::Validate;

pub fn validate_request_data<T: Validate>(data: &T) -> AppResult<()> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(errors) => Err(app_error!(
            AppErrorCode::BadRequest,
            json!(errors).to_string()
        )),
    }
}
