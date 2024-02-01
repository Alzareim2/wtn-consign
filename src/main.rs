use reqwest::header::{HeaderMap, HeaderValue, HOST, PRAGMA, ACCEPT, AUTHORIZATION, X_XSS_PROTECTION, ACCEPT_LANGUAGE, ACCEPT_ENCODING, CACHE_CONTROL, ORIGIN, USER_AGENT, REFERER};
use reqwest::Client;
use reqwest::header::HeaderName;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let client = Client::new();
    let mut seen_sizes: HashMap<String, HashSet<String>> = HashMap::new();

    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static("api-sell.wethenew.com"));
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6ImFsemFyZWltMTIzNEBnbWFpbC5jb20iLCJmaXJzdG5hbWUiOiJJc2FiZWxsZSIsImxhc3RuYW1lIjoiUmVnaW5hYyAiLCJpYXQiOjE2ODkyNDIwODYsImV4cCI6MTY5NDQyNjA4Nn0.ZVF8DOG6a1QJOTbNm07SznkJahGtqNEn2Pez3TmeQwE"));
    headers.insert(X_XSS_PROTECTION, HeaderValue::from_static("1;mode=block"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr-FR,fr;q=0.9"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(HeaderName::from_static("feature-policy"), HeaderValue::from_static("microphone 'none'; geolocation 'none'; camera 'none'; payment 'none'; battery 'none'; gyroscope 'none'; accelerometer 'none';"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://sell.wethenew.com"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148"));
    headers.insert(REFERER, HeaderValue::from_static("https://sell.wethenew.com/"));

    let headers_map: HashMap<_, _> = headers.iter().map(|(k, v)| {
        (k.to_string(), v.to_str().unwrap_or("").to_string())
    }).collect();

        loop {
            let payload = json!({
                "headers": headers_map.clone(),
                "requestUrl": "https://api-sell.wethenew.com/consignment-slots?productBrands%5B%5D=Nike&productBrands%5B%5D=Adidas&productBrands%5B%5D=Air%20Jordan&productBrands%5B%5D=New%20Balance&productBrands%5B%5D=Swatch&skip=0&take=100",
                "requestMethod": "GET",
                // "proxyUrl": proxy_url,
            });
    
            let res = client.post("http://localhost:8080/api/forward")
                .header("x-api-key", "my-auth-key-1")
                .json(&payload)
                .send()
                .await?;
    
            let res_text = res.text().await?;
    
            let value: Value = serde_json::from_str(&res_text)?;
    
            if let Some(body) = value.get("body") {
                match serde_json::from_str::<Value>(body.as_str().unwrap()) {
                    Ok(json_body) => {
                        if let Some(results) = json_body.get("results") {
                            for result in results.as_array().unwrap() {
                                if let Some(id) = result.get("id") {
                                    let id = id.to_string();
                                    if let Some(sizes) = result.get("sizes") {
                                        let current_sizes_str: HashSet<String> = sizes.as_array().unwrap()
                                            .iter()
                                            .map(|size| format!(" {} - consign\n", size.as_str().unwrap()))
                                            .collect::<HashSet<String>>();
                                        let shoe_sizes = seen_sizes.entry(id.clone()).or_default();
                            
                                        
                                        let new_sizes: HashSet<_> = current_sizes_str.difference(shoe_sizes).cloned().collect();
                            
                                        if !new_sizes.is_empty() {
                                            
                                            shoe_sizes.extend(new_sizes.iter().cloned());
                            
                                            let sizes_str = new_sizes.into_iter().collect::<Vec<_>>().join("");
                                            let webhook_content = json!({
                                                "username": "Wethenew Consign",
                                                "avatar_url": "https://image.noelshack.com/fichiers/2023/35/2/1693324495-image-2.png",
                                                "embeds": [{
                                                    "color": 16711680,
                                                    "description": format!("**[{}]({})**",
                                                        format!("{} {}", result.get("brand").unwrap().as_str().unwrap(), result.get("name").unwrap().as_str().unwrap()),
                                                        format!("{}{}", "https://sell.wethenew.com/consignment/product/", id)),
                                                    "fields": [
                                                        {
                                                            "name": "Brand",
                                                            "value": result.get("brand").unwrap().as_str().unwrap(),
                                                            "inline": true
                                                        },
                                                       
                                                        {
                                                        "name": "Sizes",
                                                        "value": sizes_str,
                                                        "inline": false
                                                        }
                                                    ],
                                                    "thumbnail": {
                                                        "url": result.get("image").unwrap().as_str().unwrap()
                                                    },
                                                    "author": {
                                                        "name": "WTN Consign",
                                                        "icon_url": "https://cdn.discordapp.com/attachments/1087460706447786107/1146113467196117082/aa.png"
                                                    },
                                                    "footer": {
                                                        "text": "Fluxy Custom = Wethenew Consign",
                                                        "icon_url": "https://image.noelshack.com/fichiers/2023/35/2/1693324495-image-2.png"
                                                    }
                                                }]
                                            });
            
                                      
                                            let webhook_url = "https://discord.com/api/webhooks/1142108063030059049/3QGO7M5LzXCpvezHmBMdSkhTqAidfA2ZafwjUfUkuG1SSyLUGN462tnmVpwFdKFfff1I";
                                            let client = reqwest::Client::new();
                                            let res = client.post(webhook_url)
                                                .json(&webhook_content)
                                                .send()
                                                .await;
                                             
                                            match res {
                                                Ok(_) => println!("Webhook sent successfully!"),
                                                Err(e) => eprintln!("Failed to send webhook: {}", e),
                                            }

                                            sleep(Duration::from_secs(1)).await;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => println!("Failed to parse body as JSON: {}", e),
                }
            } else {
                println!("No 'body' hehe");
            }
            
                    
            
            sleep(Duration::from_secs(60)).await;
        }
}

