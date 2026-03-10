use reqwest::StatusCode;
use serde::Deserialize;
use std::{env, time::Duration};
use tokio::time::sleep;

#[derive(Deserialize)]
struct HealthResponse {
    status: String,
}

#[derive(Deserialize)]
struct AccountResponse {
    uid: String,
    account_type: String,
}

#[tokio::test]
async fn smoke_health_and_accounts_crud() {
    dotenvy::dotenv().ok();

    let run_smoke = env::var("RUN_SMOKE_API")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if !run_smoke {
        eprintln!("skipping smoke_health_and_accounts_crud (set RUN_SMOKE_API=1 to enable)");
        return;
    }

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3333".to_string());
    let client = reqwest::Client::new();
    wait_for_health(&client, &base_url, 30, 300).await;

    let health = client
        .get(format!("{}/api/v1/health", base_url))
        .send()
        .await
        .expect("health request failed");
    assert_eq!(health.status(), StatusCode::OK);
    let health_body: HealthResponse = health.json().await.expect("health json parse failed");
    assert_eq!(health_body.status, "ok");

    let create = client
        .post(format!("{}/api/v1/admin/accounts", base_url))
        .json(&serde_json::json!({
            "account_type": "user",
            "username": "smoke-user",
            "email": "smoke-user@example.com"
        }))
        .send()
        .await
        .expect("create request failed");
    assert_eq!(create.status(), StatusCode::CREATED);
    let created: AccountResponse = create.json().await.expect("create json parse failed");
    assert_eq!(created.account_type, "user");

    let get = client
        .get(format!(
            "{}/api/v1/admin/accounts/{}",
            base_url, created.uid
        ))
        .send()
        .await
        .expect("get request failed");
    assert_eq!(get.status(), StatusCode::OK);

    let patch = client
        .patch(format!(
            "{}/api/v1/admin/accounts/{}",
            base_url, created.uid
        ))
        .json(&serde_json::json!({
            "username": "smoke-user-updated"
        }))
        .send()
        .await
        .expect("patch request failed");
    assert_eq!(patch.status(), StatusCode::OK);

    let delete = client
        .delete(format!(
            "{}/api/v1/admin/accounts/{}",
            base_url, created.uid
        ))
        .send()
        .await
        .expect("delete request failed");
    assert_eq!(delete.status(), StatusCode::NO_CONTENT);
}

async fn wait_for_health(client: &reqwest::Client, base_url: &str, retries: usize, delay_ms: u64) {
    let url = format!("{}/api/v1/health", base_url);
    for attempt in 0..retries {
        match client.get(&url).send().await {
            Ok(response) if response.status() == StatusCode::OK => return,
            _ => {
                if attempt + 1 >= retries {
                    panic!(
                        "service not ready after {} attempts (base_url={})",
                        retries, base_url
                    );
                }
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}
