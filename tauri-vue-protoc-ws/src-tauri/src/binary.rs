use poem::{
  error::ReadBodyError, http::header, web::RequestBody, FromRequest, IntoResponse, Request,
  Response, Result,
};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Binary(pub Vec<u8>);

impl Deref for Binary {
  type Target = Vec<u8>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Binary {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[async_trait::async_trait]
impl<'a> FromRequest<'a> for Binary {
  type Error = ReadBodyError;

  async fn from_request(_req: &'a Request, body: &mut RequestBody) -> Result<Self, Self::Error> {
    Ok(Self(body.take()?.into_vec().await?))
  }
}

impl IntoResponse for Binary {
  fn into_response(self) -> Response {
    Response::builder()
      .header(header::CONTENT_TYPE, "arraybuffer")
      .body(self.0)
  }
}
