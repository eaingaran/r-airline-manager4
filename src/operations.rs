use crate::departure;
use crate::marketing;

#[tokio::main]
pub(crate) async fn perform_routine_operations(cookies: &str) {
    // currently, we are not starting any campaigns.
    // below is the code for it
    let (airline_marketing_required, eco_marketing_required, cargo_marketing_required) =
        (false, false, false);

    if !!!(airline_marketing_required || eco_marketing_required || cargo_marketing_required) {
        return;
    }

    let active_campaigns: Vec<String> = marketing::get_active_campaigns(cookies);

    if !!!active_campaigns.contains(&"Eco friendly".to_string()) && eco_marketing_required {
        marketing::start_eco_campaign(cookies);
    }

    if !!!active_campaigns.contains(&"Airline reputation".to_string()) && airline_marketing_required
    {
        marketing::start_airline_campaign(cookies);
    }

    if !!!active_campaigns.contains(&"Cargo reputation".to_string()) && cargo_marketing_required {
        marketing::start_cargo_campaign(cookies);
    }

    // depart planes (if reputation is above a reasonable level)
    departure::depart_planes(cookies);

    // buy fuel (if price is reasonable)

    // buy co2 (if price is reasonable)

    // buy aircraft (if hanger space is available)
}
