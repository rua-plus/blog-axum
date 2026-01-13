#![allow(dead_code)]

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

// 状态码定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub field: Option<String>,
    pub message: String,
}

// 分页信息结构体
#[derive(Debug, Serialize)]
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
