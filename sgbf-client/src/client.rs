use std::sync::Arc;
use reqwest::cookie;
use reqwest::cookie::CookieStore;
use serde::Serialize;
use crate::parsing;
use crate::model::{Day, RosterEntry};

pub struct Client {
    inner: reqwest::Client,
    cookie_provider: Arc<cookie::Jar>,
}

impl Client {
    pub async fn from_credentials(username: &str, password: &str) -> Self {
        let provider = Arc::new(cookie::Jar::default());
        let client = Self {
            inner: reqwest::Client::builder().cookie_provider(provider.clone()).build().unwrap(),
            cookie_provider: provider,
        };
        client.login(username, password).await;
        client
    }

    pub async fn from_token(token: &str) -> Self {
        let jar = cookie::Jar::default();
        jar.add_cookie_str(format!("PHPSESSID={}", token).as_str(), &reqwest::Url::parse(BASE_URL).unwrap());
        let provider = Arc::new(jar);
        let client = Self {
            inner: reqwest::Client::builder().cookie_provider(provider.clone()).build().unwrap(),
            cookie_provider: provider,
        };

        client
    }

    pub fn get_token(&self) -> String {
        let jar = self.cookie_provider.as_ref();
        let cookie = jar.cookies(&reqwest::Url::parse(BASE_URL).unwrap()).unwrap();
        let cookie = cookie.to_str().unwrap();
        let cookie = cookie.split(";").next().unwrap();
        let cookie = cookie.split("=").nth(1).unwrap();
        cookie.to_string()
    }

    async fn login(&self, username: &str, password: &str) {
        let url = format!("{}{}", BASE_URL, PATH_LOGIN);
        let body = LoginBody {
            username: username.to_string(),
            password: password.to_string(),
            save: "Login".to_string(),
        };
        let request = self.inner.post(url)
            .form(&body)
            .build()
            .unwrap();
        let _ = self.inner.execute(request).await.unwrap();
    }

    pub async fn get_calendar(&self) -> Vec<Day> {
        let url = format!("{}{}", BASE_URL, PATH_CALENDAR);
        let request = self.inner.get(url)
            .build()
            .unwrap();
        let response = self.inner.execute(request).await.unwrap();
        // body is html
        let body = response.text().await.unwrap();
        // parse
        parsing::parse_calendar(body)
    }

    pub async fn get_day(&self, date: chrono::NaiveDate) -> Vec<RosterEntry> {
        let url = format!("{}{}", BASE_URL, PATH_DAY);
        let request = self.inner.get(url)
            // fe_t=participant_sf&select_date=2023-06-04&fe_f=text
            .query(&[("fe_t", "participant_sf"), ("select_date", date.format("%Y-%m-%d").to_string().as_ref()), ("fe_f", "text")])
            .build()
            .unwrap();
        let response = self.inner.execute(request).await.unwrap();
        // body is html
        let body = response.text().await.unwrap();
        // parse
        parsing::parse_roster(body)
    }
}

const BASE_URL: &str = "https://schlepppiloten.ch";
const PATH_MENU: &str = "/menu.php";
const PATH_LOGIN: &str = "/edit/login_check.php";
const PATH_CALENDAR: &str = "/roster/list_roster_new.php";
const PATH_DAY: &str = "/roster/participant_edit.php";

#[derive(Debug, Clone, Serialize)]
struct LoginBody {
    #[serde(rename = "login_user")]
    username: String,
    #[serde(rename = "login_password")]
    password: String,
    save: String,
}

#[cfg(feature = "axum")]
mod axum {
    use axum::extract::FromRequestParts;
    use axum::async_trait;
    use axum::http::StatusCode;
    use axum::http::request::Parts;

    #[async_trait]
    impl <S> FromRequestParts<S> for super::Client {
        type Rejection = StatusCode;
        async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
            // get authorization header
            let header = parts.headers.get("Authorization");
            // check if header is present
            if let Some(header) = header {
                // check if header is valid
                if let Ok(header) = header.to_str() {
                    // check if header is valid
                    if header.starts_with("Bearer ") {
                        // get token
                        let token = header.strip_prefix("Bearer ").unwrap();
                        // create client
                        let client = super::Client::from_token(token).await;
                        // return client
                        return Ok(client);
                    }
                }
            }
            // return error
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}