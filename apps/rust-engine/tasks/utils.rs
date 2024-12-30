use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::utils::{ApiResult, logo::get_logo_url};

// Priority institutions for sorting
const PRIORITY_INSTITUTIONS: &[&str] = &[
    "chase",           // Chase
    "wells_fargo",     // Wells Fargo
    "bank_of_america", // Bank Of America
    "pnc",            // PNC
    "credit_one",     // CreditOne
    "capital_one",    // CapitalOne
    "us_bank",        // US Bank
    "usaa",           // USAA
    "mercury",        // Mercury
    "citibank",       // Citibank
    "silicon_valley_bank", // Silicon Valley Bank
    "first_republic",  // First Republic
    "brex",           // Brex
    "amex",           // American Express
    "ins_133680",     // Angel List
    "morgan_stanley", // Morgan Stanley
    "truist",         // Truist
    "td_bank",        // TD Bank
    "ins_29",         // KeyBank
    "ins_19",         // Regions Bank
    "fifth_third",    // Fifth Third Bank
    "ins_111098",     // Citizens Bank
    "ins_100103",     // Comerica Bank
    "ins_21",         // Huntington Bank
];

pub fn get_popularity(id: &str) -> i32 {
    if let Some(pos) = PRIORITY_INSTITUTIONS.iter().position(|&x| x == id) {
        return 100 - pos as i32;
    }
    0
}

pub fn match_logo_url(id: &str) -> Option<String> {
    let logo_map: HashMap<&str, &str> = [
        ("ins_56", "chase"),
        ("ins_127991", "wells_fargo"),
        ("ins_116236", "ins_116236"),
        ("ins_133019", "wise"),
    ].iter().cloned().collect();

    logo_map.get(id)
        .map(|&mapped_id| get_logo_url(mapped_id))
        .or_else(|| Some(get_logo_url(id)))
}

pub async fn save_image_from_base64(base64_string: &str, file_path: &Path) -> ApiResult<()> {
    let bytes = base64::decode(base64_string)?;
    let mut file = File::create(file_path).await?;
    file.write_all(&bytes).await?;
    Ok(())
}

pub async fn save_image_from_url(url: &str, file_path: &Path) -> ApiResult<()> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(file_path).await?;
    file.write_all(&bytes).await?;
    Ok(())
}

pub async fn save_file(file_path: &Path, content: &str) -> ApiResult<()> {
    let mut file = File::create(file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

pub async fn retry<F, T, E>(f: F, retries: u32) -> Result<T, E>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>>>>,
{
    let mut attempts = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= retries {
                    return Err(e);
                }
                tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(attempts))).await;
            }
        }
    }
}

pub fn get_date_range(days: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let end = Utc::now();
    let start = end - chrono::Duration::days(days);
    (start, end)
}

#[derive(Debug, Deserialize)]
pub struct TellerResponse {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

pub async fn get_teller_data() -> ApiResult<Vec<TellerResponse>> {
    let client = reqwest::Client::new();
    let response = client.get("https://api.teller.io/institutions")
        .send()
        .await?;
    let data = response.json().await?;
    Ok(data)
}

pub async fn batch_promises<T, F, Fut>(tasks: Vec<F>) -> Vec<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let mut handles = Vec::with_capacity(tasks.len());
    for task in tasks {
        handles.push(tokio::spawn(task()));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    results
}
