use rocket::serde::Deserialize;

#[derive(Deserialize)]
struct IkeaResponse {
    availabilities: Vec<Availability>,
}

#[derive(Deserialize)]
struct Availability {
    #[serde(rename = "buyingOption")]
    buying_option: BuyingOption,
    #[serde(rename = "classUnitKey")]
    class_unit_key: ClassUnitKey,
}

#[derive(Deserialize)]
struct BuyingOption {
    #[serde(rename = "cashCarry")]
    cash_carry: Option<CashCarry>,
}

#[derive(Deserialize)]
struct CashCarry {
    availability: Option<CashCarryAvailability>,
}

#[derive(Deserialize)]
struct CashCarryAvailability {
    quantity: i32,
}

#[derive(Deserialize)]
struct ClassUnitKey {
    #[serde(rename = "classUnitCode")]
    class_unit_code: String,
}

#[get("/shark")]
pub async fn shark() -> String {
    let url =
        "https://api.salesitem.ingka.com/availabilities/ru/de?itemNos=30373588&expand=StoresList";
    let client = reqwest::Client::new();

    let response = match client
        .get(url)
        .header("X-Client-ID", "ef382663-a2a5-40d4-8afe-f0634821c0ed")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return "Error fetching data from Ikea".to_string(),
    };

    let data = match response.json::<IkeaResponse>().await {
        Ok(json) => json,
        Err(_) => return "Error parsing Ikea response".to_string(),
    };

    if let Some(store) = data
        .availabilities
        .iter()
        .find(|a| a.class_unit_key.class_unit_code == "147")
    {
        let quantity = store
            .buying_option
            .cash_carry
            .as_ref()
            .and_then(|cc| cc.availability.as_ref())
            .map(|a| a.quantity)
            .unwrap_or(0);
        format!("Ikea currently has {} BLÅHAJ in stock", quantity)
    } else {
        "Store 147 not found in Ikea response".to_string()
    }
}
