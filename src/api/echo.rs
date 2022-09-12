use std::str::FromStr;
use std::env;
use std::path;
use log::{info, error};
use derive_more::Display;
use actix_web::{
    get, 
    Error,
    Responder,
    Result,
    HttpRequest, 
    HttpResponse,
    web, 
    web::Json,
    error::ResponseError,
    http::{header::ContentType, StatusCode}
};
use pyo3::prelude::*;
use pyo3::types::{PyTuple, IntoPyDict};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Display)]
pub enum ModelServerErrorCode {
    ModelServerOk,
    ModelServerError
}

impl ResponseError for ModelServerErrorCode {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode{
        match self {
            ModelServerOk => StatusCode::NOT_FOUND,
            ModelServerError => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Serialize)]
pub struct ModelServerResult {
    pub code: i32,
    pub message: String,
    pub result: serde_json::Value
}


#[get("/app/ins/python_echo")]
async fn echo(python_mod: web::Data<Py<PyAny>>, req: HttpRequest) -> Result<Json<ModelServerResult>, ModelServerErrorCode> {
    let res = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[req.query_string()]);
        python_mod.call0(py)
    });
    Ok(Json(ModelServerResult{
        code: 0,
        message: String::from_str("success").unwrap(),
        result: serde_json::from_str(res.unwrap().to_string().as_str()).unwrap()
    }))
}