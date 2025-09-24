use serde::Serialize;

/// 本プロジェクトで使用するエラー種別の列挙体
///
/// ## Notes
/// * `non_exhaustive` - 将来的に列挙子が追加される可能性があることを示す
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub enum ErrorKind {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    RequestTimeout,
    Conflict,
    UnprocessableEntity,
    TooManyRequests,
    UnavailableForLegalReasons,
    InternalServerError,
    ServiceUnavailable,
    UnexpectedServerError,
}

impl ErrorKind {
    /// [`ErrorKind`]をユーザー向けに表示する文字列リテラル表現に変換する
    ///
    /// ## Arguments
    /// * `&self` - 変換対象の[`ErrorKind`]
    ///
    /// ## Returns
    /// * 変換対象の[`ErrorKind`]に対応する文字列リテラル表現
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::kind::ErrorKind;
    /// assert_eq!(ErrorKind::BadRequest.as_str(), "Bad Request");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::BadRequest => "Bad Request",
            ErrorKind::Unauthorized => "Unauthorized",
            ErrorKind::Forbidden => "Forbidden",
            ErrorKind::NotFound => "Not Found",
            ErrorKind::RequestTimeout => "Request Timeout",
            ErrorKind::Conflict => "Conflict",
            ErrorKind::UnprocessableEntity => "Unprocessable Entity",
            ErrorKind::TooManyRequests => "Too Many Requests",
            ErrorKind::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            ErrorKind::InternalServerError => "Internal Server Error",
            ErrorKind::ServiceUnavailable => "Service Unavailable",
            ErrorKind::UnexpectedServerError => "Unexpected Server Error",
        }
    }

    /// [`ErrorKind`]をHTTPステータスコードに準拠する数値表現に変換する
    ///
    /// ## Arguments
    /// * `&self` - 変換対象の[`ErrorKind`]
    ///
    /// ## Returns
    /// * 変換対象の[`ErrorKind`]に対応するHTTPステータスコードに準拠した数値表現（RFC 7231準拠）
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::kind::ErrorKind;
    /// assert_eq!(ErrorKind::BadRequest.as_code(), 400);
    /// ```
    pub const fn as_code(&self) -> u16 {
        match self {
            ErrorKind::BadRequest => 400,
            ErrorKind::Unauthorized => 401,
            ErrorKind::Forbidden => 403,
            ErrorKind::NotFound => 404,
            ErrorKind::RequestTimeout => 408,
            ErrorKind::Conflict => 409,
            ErrorKind::UnprocessableEntity => 422,
            ErrorKind::TooManyRequests => 429,
            ErrorKind::UnavailableForLegalReasons => 451,
            ErrorKind::InternalServerError => 500,
            ErrorKind::ServiceUnavailable => 503,
            ErrorKind::UnexpectedServerError => 599,
        }
    }
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_error_kind_as_str() {
        assert_eq!(ErrorKind::BadRequest.as_str(), "Bad Request");
        assert_eq!(ErrorKind::Unauthorized.as_str(), "Unauthorized");
        assert_eq!(ErrorKind::Forbidden.as_str(), "Forbidden");
        assert_eq!(ErrorKind::NotFound.as_str(), "Not Found");
        assert_eq!(ErrorKind::RequestTimeout.as_str(), "Request Timeout");
        assert_eq!(ErrorKind::Conflict.as_str(), "Conflict");
        assert_eq!(
            ErrorKind::UnprocessableEntity.as_str(),
            "Unprocessable Entity"
        );
        assert_eq!(ErrorKind::TooManyRequests.as_str(), "Too Many Requests");
        assert_eq!(
            ErrorKind::UnavailableForLegalReasons.as_str(),
            "Unavailable For Legal Reasons"
        );
        assert_eq!(
            ErrorKind::ServiceUnavailable.as_str(),
            "Service Unavailable"
        );
        assert_eq!(
            ErrorKind::InternalServerError.as_str(),
            "Internal Server Error"
        );
        assert_eq!(
            ErrorKind::UnexpectedServerError.as_str(),
            "Unexpected Server Error"
        );
    }

    #[test]
    fn test_error_kind_as_code() {
        assert_eq!(ErrorKind::BadRequest.as_code(), 400);
        assert_eq!(ErrorKind::Unauthorized.as_code(), 401);
        assert_eq!(ErrorKind::Forbidden.as_code(), 403);
        assert_eq!(ErrorKind::NotFound.as_code(), 404);
        assert_eq!(ErrorKind::RequestTimeout.as_code(), 408);
        assert_eq!(ErrorKind::Conflict.as_code(), 409);
        assert_eq!(ErrorKind::UnprocessableEntity.as_code(), 422);
        assert_eq!(ErrorKind::TooManyRequests.as_code(), 429);
        assert_eq!(ErrorKind::UnavailableForLegalReasons.as_code(), 451);
        assert_eq!(ErrorKind::InternalServerError.as_code(), 500);
        assert_eq!(ErrorKind::ServiceUnavailable.as_code(), 503);
        assert_eq!(ErrorKind::UnexpectedServerError.as_code(), 599);
    }
}
