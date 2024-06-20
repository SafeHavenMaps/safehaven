use serde::Deserialize;

#[derive(Deserialize)]
struct HCaptchaResponse {
    success: bool,

    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>,
}

pub enum HCaptchaValidationError {
    NetworkError(),
    HCaptchaError(Vec<String>),
}

pub async fn check_hcaptcha(
    client_response: String,
    secret: String,
    remote_ip: Option<String>,
) -> Result<bool, HCaptchaValidationError> {
    let client = reqwest::Client::new();

    let mut form = vec![("response", client_response), ("secret", secret)];

    if let Some(remote_ip) = remote_ip {
        form.push(("remoteip", remote_ip));
    }

    let response = client
        .post("https://hcaptcha.com/siteverify")
        .form(&form)
        .send()
        .await
        .map_err(|_| HCaptchaValidationError::NetworkError())?;

    let response: HCaptchaResponse = response
        .json()
        .await
        .map_err(|_| HCaptchaValidationError::NetworkError())?;

    if response.success {
        Ok(true)
    } else {
        Err(HCaptchaValidationError::HCaptchaError(
            response.error_codes.unwrap_or_default(),
        ))
    }
}
