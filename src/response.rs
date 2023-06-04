use bson::doc;
use serde::Serialize;

#[repr(u16)]
pub enum Status {
    Accepted,
    BadRequest,
    Forbidden,
    NotFound,
    Unauthorized,
    Custom(u16),
}

impl Status {
    pub fn convert(&self) -> u16 {
        match self {
            Self::Accepted => 200,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Custom(status) => *status,
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self.convert();
        serializer.serialize_u16(value)
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let status = match self {
            Self::Accepted => 200,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Custom(status) => *status,
        };
        status.to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseBuilder<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> ResponseBuilder<T> {
    pub fn build(code: Status, data: T) -> HttpResult<T> {
        Ok(HttpResponse { code, data })
    }

    pub fn build_err(code: Status, data: impl ToString) -> HttpResult<T> {
        Err(HttpError {
            code,
            data: data.to_string(),
        })
    }
}

#[derive(Serialize)]
pub struct HttpResponse<T> {
    code: Status,
    data: T,
}

#[derive(Serialize)]
pub struct HttpError {
    code: Status,
    data: String,
}

impl<'request, 'output, T> rocket::response::Responder<'request, 'output> for HttpResponse<T>
where
    'output: 'request,
    T: Serialize,
{
    fn respond_to(self, _: &'request rocket::Request<'_>) -> rocket::response::Result<'output> {
        // settings the body as `json`
        let body = serde_json::to_string(&self).unwrap();
        let mut response = rocket::Response::build();
        response
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(
                rocket::http::Status::from_code(self.code.convert())
                    .unwrap_or(rocket::http::Status::InternalServerError),
            )
            .ok()
    }
}

impl<'request, 'output> rocket::response::Responder<'request, 'output> for HttpError
where
    'output: 'request,
{
    fn respond_to(self, _: &'request rocket::Request<'_>) -> rocket::response::Result<'output> {
        // settings the body as `json`
        let body = serde_json::to_string(&self).unwrap();
        let mut response = rocket::Response::build();
        response
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(
                rocket::http::Status::from_code(self.code.convert())
                    .unwrap_or(rocket::http::Status::InternalServerError),
            )
            .ok()
    }
}

pub type HttpResult<T> = Result<HttpResponse<T>, HttpError>;
