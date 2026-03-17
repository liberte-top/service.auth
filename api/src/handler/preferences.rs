use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cookie::Cookie;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::state::AppState;

const LANGUAGE_COOKIE: &str = "liberte_language";
const THEME_COOKIE: &str = "liberte_theme";

#[derive(Clone, Copy)]
enum PreferenceLanguage {
    En,
    ZhCn,
}

#[derive(Clone, Copy)]
enum PreferenceTheme {
    System,
    Light,
    Dark,
}

#[derive(Serialize, ToSchema)]
pub struct PreferenceOption {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, ToSchema)]
pub struct PreferencesResponse {
    pub language: String,
    pub theme: String,
}

#[derive(Serialize, ToSchema)]
pub struct PreferenceOptionsResponse {
    pub languages: Vec<PreferenceOption>,
    pub themes: Vec<PreferenceOption>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePreferencesRequest {
    pub language: Option<String>,
    pub theme: Option<String>,
}

impl PreferenceLanguage {
    fn parse(value: Option<&str>) -> Self {
        match value.unwrap_or("en").trim().to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" => Self::ZhCn,
            _ => Self::En,
        }
    }

    fn value(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::ZhCn => "zh-CN",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::ZhCn => "简体中文",
        }
    }

    fn option(self) -> PreferenceOption {
        PreferenceOption {
            label: self.label().to_owned(),
            value: self.value().to_owned(),
        }
    }
}

impl PreferenceTheme {
    fn parse(value: Option<&str>) -> Self {
        match value.unwrap_or("system").trim().to_ascii_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }

    fn value(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    fn option(self) -> PreferenceOption {
        PreferenceOption {
            label: self.label().to_owned(),
            value: self.value().to_owned(),
        }
    }
}

fn normalize_language(value: Option<&str>) -> &'static str {
    PreferenceLanguage::parse(value).value()
}

fn normalize_theme(value: Option<&str>) -> &'static str {
    PreferenceTheme::parse(value).value()
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| {
            raw.split(';').find_map(|item| {
                let mut parts = item.trim().splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) if key.trim() == name => Some(value.trim().to_owned()),
                    _ => None,
                }
            })
        })
}

fn preferences_from_headers(headers: &HeaderMap) -> PreferencesResponse {
    PreferencesResponse {
        language: normalize_language(cookie_value(headers, LANGUAGE_COOKIE).as_deref()).to_owned(),
        theme: normalize_theme(cookie_value(headers, THEME_COOKIE).as_deref()).to_owned(),
    }
}

fn preference_options() -> PreferenceOptionsResponse {
    PreferenceOptionsResponse {
        languages: vec![PreferenceLanguage::En.option(), PreferenceLanguage::ZhCn.option()],
        themes: vec![
            PreferenceTheme::System.option(),
            PreferenceTheme::Light.option(),
            PreferenceTheme::Dark.option(),
        ],
    }
}

fn preference_cookie(state: &AppState, name: &str, value: &str) -> String {
    let mut cookie = Cookie::build((name, value.to_owned()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::days(365));

    if let Some(domain) = state.config().forwardauth_session_cookie_domain() {
        cookie = cookie.domain(domain.to_owned());
    }

    cookie.build().to_string()
}

#[utoipa::path(
    get,
    path = "/api/v1/preferences/options",
    responses((status = 200, description = "Supported cross-app preference options", body = PreferenceOptionsResponse)),
    tag = "preferences"
)]
pub async fn get_preference_options() -> impl IntoResponse {
    Json(preference_options())
}

#[utoipa::path(
    get,
    path = "/api/v1/preferences",
    responses((status = 200, description = "Current cross-app preferences", body = PreferencesResponse)),
    tag = "preferences"
)]
pub async fn get_preferences(headers: HeaderMap) -> impl IntoResponse {
    Json(preferences_from_headers(&headers))
}

#[utoipa::path(
    post,
    path = "/api/v1/preferences",
    request_body = UpdatePreferencesRequest,
    responses((status = 200, description = "Updated cross-app preferences", body = PreferencesResponse)),
    tag = "preferences"
)]
pub async fn update_preferences(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdatePreferencesRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let language = normalize_language(payload.language.as_deref()).to_owned();
    let theme = normalize_theme(payload.theme.as_deref()).to_owned();

    let mut response_headers = HeaderMap::new();
    response_headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&preference_cookie(&state, LANGUAGE_COOKIE, &language))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    response_headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&preference_cookie(&state, THEME_COOKIE, &theme))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((
        response_headers,
        Json(PreferencesResponse {
            language,
            theme,
        }),
    ))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/preferences/options", get(get_preference_options))
        .route(
            "/api/v1/preferences",
            get(get_preferences).post(update_preferences),
        )
        .with_state(state)
}
