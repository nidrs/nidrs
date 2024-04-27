// #[exception]
pub enum AppException {
    // #[exception(Exception::Http(StatusCode::INTERNAL_SERVER_ERROR, anyhow::Error::msg("{0}")))]
    ServiceException(String),
}
