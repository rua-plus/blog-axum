#![allow(dead_code)]

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

// 状态码定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    // HTTP状态码
    // HttpOk = 200,
    // HttpCreated = 201,
    // HttpBadRequest = 400,
    // HttpUnauthorized = 401,
    // HttpForbidden = 403,
    // HttpNotFound = 404,
    // HttpInternalError = 500,

    // 业务成功状态码
    Success = 200,
    Created = 201,
    Accepted = 202,

    // 业务错误状态码（4xxxx）
    BadRequest = 40000,
    ValidationError = 40001,
    ParamError = 40002,

    // 认证授权错误（401xx）
    Unauthorized = 40100,
    TokenExpired = 40101,
    TokenInvalid = 40102,

    // 权限错误（403xx）
    Forbidden = 40300,
    AccessDenied = 40301,

    // 资源错误（404xx）
    NotFound = 40400,
    ResourceNotFound = 40401,

    // 业务逻辑错误（409xx）
    Conflict = 40900,
    DuplicateResource = 40901,

    // 系统错误（500xx）
    InternalError = 50000,
    ServiceUnavailable = 50001,
    DatabaseError = 50002,

    // 第三方服务错误（502xx）
    ThirdPartyError = 50200,
    ExternalApiError = 50201,
}

// 实现 StatusCode 到 u32 的转换
impl From<StatusCode> for u32 {
    fn from(code: StatusCode) -> Self {
        code as u32
    }
}

// 实现 StatusCode 的序列化
impl serde::Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

// 基础响应结构体
#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub success: bool,
    pub code: StatusCode,
    pub message: String,
    pub timestamp: u64,
    pub request_id: String,
}

// 成功响应结构体
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub code: StatusCode,
    pub message: String,
    pub timestamp: u64,
    pub request_id: String,
    pub data: Option<T>,
    pub version: Option<String>,
}

// 错误响应结构体
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub code: StatusCode,
    pub message: String,
    pub timestamp: u64,
    pub request_id: String,
    pub errors: Option<Vec<ErrorDetail>>,
    pub path: Option<String>,
    pub debug: Option<String>,
}

// 错误详情结构体
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ErrorDetail {
    pub field: Option<String>,
    pub message: String,
}

// 分页信息结构体
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
    pub total_pages: u32,
}

// 分页响应结构体
#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub success: bool,
    pub code: StatusCode,
    pub message: String,
    pub timestamp: u64,
    pub request_id: String,
    pub data: PaginationData<T>,
    pub version: Option<String>,
}

// 分页数据结构体
#[derive(Debug, Serialize)]
pub struct PaginationData<T> {
    pub list: Vec<T>,
    pub pagination: PaginationInfo,
}

impl BaseResponse {
    // 生成当前时间戳
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    // 生成默认的请求ID（简单实现，实际项目中应使用更复杂的方法）
    fn default_request_id() -> String {
        format!("{}", Self::current_timestamp())
    }
}

impl<T> SuccessResponse<T> {
    // 创建成功响应
    pub fn new(code: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            success: true,
            code,
            message: message.into(),
            timestamp: BaseResponse::current_timestamp(),
            request_id: BaseResponse::default_request_id(),
            data,
            version: option_env!("GIT_VERSION").map(|v| v.to_string()),
        }
    }

    // 创建带版本信息的成功响应
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

impl ErrorResponse {
    // 创建错误响应
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            success: false,
            code,
            message: message.into(),
            timestamp: BaseResponse::current_timestamp(),
            request_id: BaseResponse::default_request_id(),
            errors: None,
            path: None,
            debug: None,
        }
    }

    // 添加错误详情
    pub fn with_errors(mut self, errors: Vec<ErrorDetail>) -> Self {
        self.errors = Some(errors);
        self
    }

    // 添加请求路径
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    // 添加调试信息
    pub fn with_debug(mut self, debug: impl Into<String>) -> Self {
        self.debug = Some(debug.into());
        self
    }
}

impl<T> PaginationResponse<T> {
    // 创建分页响应
    pub fn new(
        code: StatusCode,
        message: impl Into<String>,
        list: Vec<T>,
        pagination: PaginationInfo,
    ) -> Self {
        Self {
            success: true,
            code,
            message: message.into(),
            timestamp: BaseResponse::current_timestamp(),
            request_id: BaseResponse::default_request_id(),
            data: PaginationData { list, pagination },
            version: option_env!("GIT_VERSION").map(|v| v.to_string()),
        }
    }

    // 创建带版本信息的分页响应
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

// 方便的构造函数
impl StatusCode {
    // 成功响应构造函数
    pub fn success<T>(data: Option<T>) -> SuccessResponse<T> {
        SuccessResponse::new(StatusCode::Success, "Success", data)
    }

    // 创建响应构造函数
    pub fn created<T>(data: Option<T>) -> SuccessResponse<T> {
        SuccessResponse::new(StatusCode::Created, "Created", data)
    }

    // 接受响应构造函数
    pub fn accepted<T>(data: Option<T>) -> SuccessResponse<T> {
        SuccessResponse::new(StatusCode::Accepted, "Accepted", data)
    }

    // 错误响应构造函数
    pub fn bad_request() -> ErrorResponse {
        ErrorResponse::new(StatusCode::BadRequest, "Bad Request")
    }

    pub fn validation_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ValidationError, "Validation Error")
    }

    pub fn param_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ParamError, "Parameter Error")
    }

    pub fn unauthorized() -> ErrorResponse {
        ErrorResponse::new(StatusCode::Unauthorized, "Unauthorized")
    }

    pub fn token_expired() -> ErrorResponse {
        ErrorResponse::new(StatusCode::TokenExpired, "Token Expired")
    }

    pub fn token_invalid() -> ErrorResponse {
        ErrorResponse::new(StatusCode::TokenInvalid, "Token Invalid")
    }

    pub fn forbidden() -> ErrorResponse {
        ErrorResponse::new(StatusCode::Forbidden, "Forbidden")
    }

    pub fn access_denied() -> ErrorResponse {
        ErrorResponse::new(StatusCode::AccessDenied, "Access Denied")
    }

    pub fn not_found() -> ErrorResponse {
        ErrorResponse::new(StatusCode::NotFound, "Not Found")
    }

    pub fn resource_not_found() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ResourceNotFound, "Resource Not Found")
    }

    pub fn conflict() -> ErrorResponse {
        ErrorResponse::new(StatusCode::Conflict, "Conflict")
    }

    pub fn duplicate_resource() -> ErrorResponse {
        ErrorResponse::new(StatusCode::DuplicateResource, "Duplicate Resource")
    }

    pub fn internal_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::InternalError, "Internal Server Error")
    }

    pub fn service_unavailable() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ServiceUnavailable, "Service Unavailable")
    }

    pub fn database_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::DatabaseError, "Database Error")
    }

    pub fn third_party_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ThirdPartyError, "Third Party Error")
    }

    pub fn external_api_error() -> ErrorResponse {
        ErrorResponse::new(StatusCode::ExternalApiError, "External API Error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response_creation() {
        let data = "test data";
        let response = StatusCode::success(Some(data));

        assert!(response.success);
        assert_eq!(response.code, StatusCode::Success);
        assert_eq!(response.message, "Success");
        assert!(response.timestamp > 0);
        assert!(!response.request_id.is_empty());
        assert_eq!(response.data.unwrap(), data);
    }

    #[test]
    fn test_created_response_creation() {
        let data = "created data";
        let response = StatusCode::created(Some(data));

        assert!(response.success);
        assert_eq!(response.code, StatusCode::Created);
        assert_eq!(response.message, "Created");
        assert!(response.timestamp > 0);
        assert!(!response.request_id.is_empty());
        assert_eq!(response.data.unwrap(), data);
    }

    #[test]
    fn test_accepted_response_creation() {
        let data = "accepted data";
        let response = StatusCode::accepted(Some(data));

        assert!(response.success);
        assert_eq!(response.code, StatusCode::Accepted);
        assert_eq!(response.message, "Accepted");
        assert!(response.timestamp > 0);
        assert!(!response.request_id.is_empty());
        assert_eq!(response.data.unwrap(), data);
    }

    #[test]
    fn test_success_response_with_version() {
        let version = "1.0.0";
        let response: SuccessResponse<()> = StatusCode::success(None).with_version(version);

        assert_eq!(response.version.unwrap(), version);
    }

    #[test]
    fn test_error_response_creation() {
        let response = StatusCode::bad_request();

        assert!(!response.success);
        assert_eq!(response.code, StatusCode::BadRequest);
        assert_eq!(response.message, "Bad Request");
        assert!(response.timestamp > 0);
        assert!(!response.request_id.is_empty());
        assert!(response.errors.is_none());
        assert!(response.path.is_none());
        assert!(response.debug.is_none());
    }

    #[test]
    fn test_error_response_with_errors() {
        let errors = vec![
            ErrorDetail {
                field: Some("email".to_string()),
                message: "Email is invalid".to_string(),
            },
            ErrorDetail {
                field: Some("password".to_string()),
                message: "Password must be at least 8 characters".to_string(),
            },
        ];

        let response = StatusCode::validation_error().with_errors(errors.clone());

        assert_eq!(response.errors.unwrap(), errors);
    }

    #[test]
    fn test_error_response_with_path_and_debug() {
        let path = "/api/users";
        let debug = "Internal server error: connection timeout";

        let response = StatusCode::internal_error()
            .with_path(path)
            .with_debug(debug);

        assert_eq!(response.path.unwrap(), path);
        assert_eq!(response.debug.unwrap(), debug);
    }

    #[test]
    fn test_pagination_response_creation() {
        let list = vec!["item1", "item2", "item3"];
        let pagination = PaginationInfo {
            page: 1,
            page_size: 10,
            total: 25,
            total_pages: 3,
        };

        let response = PaginationResponse::new(
            StatusCode::Success,
            "Items retrieved successfully",
            list.clone(),
            pagination.clone(),
        );

        assert!(response.success);
        assert_eq!(response.code, StatusCode::Success);
        assert_eq!(response.message, "Items retrieved successfully");
        assert!(response.timestamp > 0);
        assert!(!response.request_id.is_empty());
        assert_eq!(response.data.list, list);
        assert_eq!(response.data.pagination, pagination);
    }

    #[test]
    fn test_pagination_response_with_version() {
        let list = vec!["item1"];
        let pagination = PaginationInfo {
            page: 1,
            page_size: 10,
            total: 1,
            total_pages: 1,
        };

        let version = "2.1.3";
        let response = PaginationResponse::new(
            StatusCode::Success,
            "Items retrieved successfully",
            list,
            pagination,
        )
        .with_version(version);

        assert_eq!(response.version.unwrap(), version);
    }

    #[test]
    fn test_response_serialization() {
        let data = "test data";
        let success_response = StatusCode::success(Some(data));
        let success_json =
            serde_json::to_string(&success_response).expect("Failed to serialize success response");

        assert!(success_json.contains("\"success\":true"));
        assert!(success_json.contains("\"code\":200"));
        assert!(success_json.contains("\"message\":\"Success\""));
        assert!(success_json.contains(&format!("\"data\":\"{}\"", data)));

        let error_response = StatusCode::unauthorized();
        let error_json =
            serde_json::to_string(&error_response).expect("Failed to serialize error response");

        assert!(error_json.contains("\"success\":false"));
        assert!(error_json.contains("\"code\":40100"));
        assert!(error_json.contains("\"message\":\"Unauthorized\""));
    }

    #[test]
    fn test_all_status_codes_have_constructors() {
        // 测试所有成功状态码的构造函数
        let _: SuccessResponse<()> = StatusCode::success(None);
        let _: SuccessResponse<()> = StatusCode::created(None);
        let _: SuccessResponse<()> = StatusCode::accepted(None);

        // 测试所有错误状态码的构造函数
        let _ = StatusCode::bad_request();
        let _ = StatusCode::validation_error();
        let _ = StatusCode::param_error();
        let _ = StatusCode::unauthorized();
        let _ = StatusCode::token_expired();
        let _ = StatusCode::token_invalid();
        let _ = StatusCode::forbidden();
        let _ = StatusCode::access_denied();
        let _ = StatusCode::not_found();
        let _ = StatusCode::resource_not_found();
        let _ = StatusCode::conflict();
        let _ = StatusCode::duplicate_resource();
        let _ = StatusCode::internal_error();
        let _ = StatusCode::service_unavailable();
        let _ = StatusCode::database_error();
        let _ = StatusCode::third_party_error();
        let _ = StatusCode::external_api_error();
    }
}
