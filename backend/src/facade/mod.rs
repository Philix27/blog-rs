pub(crate) mod asset;
pub(crate) mod image;
pub(crate) mod management;
pub(crate) mod post;
pub(crate) mod tag;
pub(crate) mod user;

use core::{convert::Infallible, result::Result};

use blog_common::{
    dto::Response as ApiResponse,
    result::{Error, ErrorResponse},
    val,
};
use bytes::Buf;
use hyper::header::{self, HeaderMap, HeaderValue};
use serde::Serialize;
use warp::{
    filters::multipart::FormData,
    http::{response::Response, StatusCode},
    reply::{Json, Response as WarpResponse},
    Rejection, Reply,
};

use crate::util::result::Result as CommonResult;

// lazy_static_include_str!(INDEX_PAGE_BYTES, "./src/resource/index.html");

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    dbg!(&err);

    let code;
    let error;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        error = Error::NotFound;
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        error = Error::BadRequest;
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        error = Error::MethodNotAllowed;
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        error = Error::InternalServerError;
    }

    let json = wrap_json_err(code.as_u16(), error);

    Ok(warp::reply::with_status(json, code))
}

#[inline]
fn wrap_json_data<D: Serialize>(data: D) -> Json {
    let r = ApiResponse::<D> {
        status: 0,
        error: None,
        data: Some(data),
    };

    warp::reply::json(&r)
}

#[inline]
fn wrap_json_err(status: u16, error: Error) -> Json {
    let r = ApiResponse::<String> {
        status,
        error: Some(ErrorResponse {
            detail: format!("{}", error),
            code: error,
        }),
        data: None,
    };

    warp::reply::json(&r)
}

#[inline]
fn response<D: Serialize>(result: CommonResult<D>) -> Result<impl Reply, Rejection> {
    let r = match result {
        Ok(d) => wrap_json_data(d),
        Err(ew) => {
            let e = ew.0;
            match e {
                Error::BusinessException(m) => wrap_json_err(400, Error::BusinessException(m)),
                _ => wrap_json_err(500, e),
            }
        },
    };
    Ok(r)
}

// https://stackoverflow.com/questions/62964013/how-can-two-headers-of-the-same-name-be-attached-to-a-warp-reply

#[inline]
fn auth_cookie(token: &str) -> String {
    format!(
        // "{}={}; Domain=songday.com; Secure; HttpOnly; Path=/",
        "{}={}; HttpOnly; Path=/",
        val::AUTH_HEADER_NAME,
        token,
    )
}
