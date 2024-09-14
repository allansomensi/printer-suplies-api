pub mod tests {
    use reqwest::StatusCode;
    use std::env::var;

    pub fn setup() {
        dotenvy::dotenv().ok();
    }

    #[tokio::test]
    pub async fn status_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/status", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // Supplies/Toner

    #[tokio::test]
    pub async fn toner_count_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/supplies/toner-count",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn toner_search_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/supplies/toner/4340c4a2-eac5-4b51-9baa-40b498605a8c",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_toners_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/supplies/toners", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_toner_endpoint() {
        setup();
        let body = r#"{
        "name": ""
    }"#;

        let endpoint = format!("http://{}/api/v1/supplies/toners", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_toner_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b398605a8c",
        "name": "TEST Toner",
        "color": "black"
    }"#;

        let endpoint = format!("http://{}/api/v1/supplies/toners", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_toner_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
    }"#;

        let endpoint = format!("http://{}/api/v1/supplies/toners", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Supplies/Drum

    #[tokio::test]
    pub async fn drum_count_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/supplies/drum-count", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn drum_search_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/supplies/drum/4340c4a2-eac5-4b51-9baa-40b498605a8c",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_drums_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/supplies/drums", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_drum_endpoint() {
        setup();
        let body = r#"{
            "name": ""
        }"#;

        let endpoint = format!("http://{}/api/v1/supplies/drums", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_drum_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b398605a8c",
        "name": "TEST Drum"
    }"#;

        let endpoint = format!("http://{}/api/v1/supplies/drums", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_drum_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
    }"#;

        let endpoint = format!("http://{}/api/v1/supplies/drums", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Brand

    #[tokio::test]
    pub async fn brand_count_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/brand-count", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn brand_search_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/brand/4340c4a2-eac5-4b51-9baa-40b498605a8c",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_brands_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/brands", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_brand_endpoint() {
        setup();
        let body = r#"{
            "name": ""
        }"#;

        let endpoint = format!("http://{}/api/v1/brands", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_brand_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b398605a8c",
        "name": "TEST brand"
    }"#;

        let endpoint = format!("http://{}/api/v1/brands", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_brand_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
    }"#;

        let endpoint = format!("http://{}/api/v1/brands", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Printer

    #[tokio::test]
    pub async fn printer_count_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/printer-count", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn printer_search_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/printer/4340c4a2-eac5-4b51-9baa-40b498605a8c",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn show_printers_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/printers", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_printer_endpoint() {
        setup();
        let body = r#"{
        "name": "",
        "model": "TEST model",
        "brand": "wrong_id",
        "toner": "wrong_id",
        "drum": "wrong_id",
    }"#;

        let endpoint = format!("http://{}/api/v1/printers", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn update_printer_endpoint() {
        setup();
        let body = r#"{
            "id": "4340c4a2-eac5-4b51-9baa-40b398605a8c",
            "name": "TEST printer",
            "model": "TEST model",
            "brand": "4340c4a2-eac5-4b51-9baa-40b498605a8c",
            "toner": "4340c4a2-eac5-4b51-9baa-40b498605a8c",
            "drum": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
        }"#;

        let endpoint = format!("http://{}/api/v1/printers", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .put(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_printer_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
    }"#;

        let endpoint = format!("http://{}/api/v1/printers", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // Movement

    // Count

    #[tokio::test]
    pub async fn count_all_movements_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/movement-count", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn count_toner_movements_endpoint() {
        setup();
        let endpoint = format!(
            "http://{}/api/v1/movement-count/toner",
            var("HOST").unwrap()
        );
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn count_drum_movements_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/movement-count/drum", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // Show

    #[tokio::test]
    pub async fn show_all_movements_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/movements", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn show_toner_movements_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/movements/toner", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn show_drum_movements_endpoint() {
        setup();
        let endpoint = format!("http://{}/api/v1/movements/drum", var("HOST").unwrap());
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_movement_endpoint() {
        setup();
        let body = r#"{
        "id": "4340c4a2-eac5-4b51-9baa-40b498605a8c"
    }"#;

        let endpoint = format!("http://{}/api/v1/movements", var("HOST").unwrap());

        let client = reqwest::Client::new();

        let response = client
            .delete(endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
