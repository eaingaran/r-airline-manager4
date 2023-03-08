use crate::co2;
use crate::departure;
use crate::fuel;
use crate::maintenance;
use crate::marketing;
use crate::purchase;
use crate::routes;

pub(crate) async fn perform_routine_operations(cookies: &str) {
    // check and buy fuel
    fuel::purchase(cookies).await;

    // check and buy co2
    co2::purchase(cookies).await;

    // check the campaign status and start new campaigns if required
    marketing::validate_campaigns(cookies, true, true, true).await;

    // depart planes (if reputation is above a reasonable level)
    // departure::depart_planes(cookies);
    // this is a temporary function. this needs to be revisited.
    departure::temp_depart_planes(cookies).await;

    // check and buy fuel
    fuel::purchase(cookies).await;

    // check and buy co2
    co2::purchase(cookies).await;

    // perform A-check
    maintenance::maintain_planes(cookies).await;

    // // route parked pax planes
    // routes::route_a388(cookies).await;

    // // route parked cargo planes
    // routes::route_a388f(cookies).await;

    // // buy pax planes (if there are empty spots in the hanger)
    // purchase::buy_a388(cookies).await;

    // // buy cargo planes (if there are empty spots in the hanger)
    // purchase::buy_a388f(cookies).await;
}
