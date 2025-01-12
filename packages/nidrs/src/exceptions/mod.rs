use nidrs_extern::{anyhow, axum::http::StatusCode};

#[macro_export]
macro_rules! define_http_exception {
    ($exception_name:ident, $status_code:expr, $default_message:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $exception_name {
            pub message: String,
            pub reason: Option<String>,
        }

        impl $exception_name {
            pub fn new<T: Into<String>>(message: T) -> Self {
                Self { message: message.into(), reason: None }
            }

            pub fn with_reason<T: Into<String>, U: Into<String>>(message: T, reason: U) -> Self {
                Self { message: message.into(), reason: Some(reason.into()) }
            }

            pub fn default() -> Self {
                Self { message: $default_message.to_string(), reason: None }
            }
        }

        impl std::fmt::Display for $exception_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(reason) = &self.reason {
                    write!(f, "{}: {}", self.message, reason)
                } else {
                    write!(f, "{}", self.message)
                }
            }
        }

        impl std::error::Error for $exception_name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }

        impl Into<$crate::Exception> for $exception_name {
            fn into(self) -> $crate::Exception {
                $crate::Exception::new($status_code, anyhow::anyhow!(self.message))
            }
        }

        impl Into<$crate::AppError> for $exception_name {
            fn into(self) -> $crate::AppError {
                $crate::AppError::Exception(self.into())
            }
        }
    };
}

// 4xx Client Errors
define_http_exception!(BadRequestException, StatusCode::BAD_REQUEST, "Bad Request");
define_http_exception!(UnauthorizedException, StatusCode::UNAUTHORIZED, "Unauthorized");
define_http_exception!(ForbiddenException, StatusCode::FORBIDDEN, "Forbidden");
define_http_exception!(NotFoundException, StatusCode::NOT_FOUND, "Not Found");
define_http_exception!(MethodNotAllowedException, StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed");
define_http_exception!(NotAcceptableException, StatusCode::NOT_ACCEPTABLE, "Not Acceptable");
define_http_exception!(RequestTimeoutException, StatusCode::REQUEST_TIMEOUT, "Request Timeout");
define_http_exception!(ConflictException, StatusCode::CONFLICT, "Conflict");
define_http_exception!(LengthRequiredException, StatusCode::LENGTH_REQUIRED, "Length Required");
define_http_exception!(PreconditionFailedException, StatusCode::PRECONDITION_FAILED, "Precondition Failed");
define_http_exception!(PayloadTooLargeException, StatusCode::PAYLOAD_TOO_LARGE, "Payload Too Large");
define_http_exception!(UnsupportedMediaTypeException, StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type");
define_http_exception!(RangeNotSatisfiableException, StatusCode::RANGE_NOT_SATISFIABLE, "Range Not Satisfiable");
define_http_exception!(ExpectationFailedException, StatusCode::EXPECTATION_FAILED, "Expectation Failed");
define_http_exception!(MisdirectedRequestException, StatusCode::MISDIRECTED_REQUEST, "Misdirected Request");
define_http_exception!(UnprocessableEntityException, StatusCode::UNPROCESSABLE_ENTITY, "Unprocessable Entity");
define_http_exception!(LockedException, StatusCode::LOCKED, "Locked");
define_http_exception!(FailedDependencyException, StatusCode::FAILED_DEPENDENCY, "Failed Dependency");
define_http_exception!(UpgradeRequiredException, StatusCode::UPGRADE_REQUIRED, "Upgrade Required");
define_http_exception!(PreconditionRequiredException, StatusCode::PRECONDITION_REQUIRED, "Precondition Required");
define_http_exception!(TooManyRequestsException, StatusCode::TOO_MANY_REQUESTS, "Too Many Requests");
define_http_exception!(UnavailableForLegalReasonsException, StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons");

// 5xx Server Errors
define_http_exception!(InternalServerException, StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error");
define_http_exception!(NotImplementedException, StatusCode::NOT_IMPLEMENTED, "Not Implemented");
define_http_exception!(BadGatewayException, StatusCode::BAD_GATEWAY, "Bad Gateway");
define_http_exception!(ServiceUnavailableException, StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable");
define_http_exception!(GatewayTimeoutException, StatusCode::GATEWAY_TIMEOUT, "Gateway Timeout");
define_http_exception!(HttpVersionNotSupportedException, StatusCode::HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported");
