use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;

  // car model structure
  struct Model {
    name: String,
    age: u32,
}

// method for calculating "engine weakening" in relation to the year of production.
fn calculate_fuel_multiplier_per_prod_year(prod_year: f64) -> f64 {
    let engine_weaking = prod_year / 2006.0_f64;
    return engine_weaking;
}

// endpoint for calculating fuel consumption
#[get("/calculateDisselUsageForDistance/{distance}/{yearOfProduction}/{fuelUsagePer100KM}")]
async fn index(info: web::Path<(u32, u32, u32)>) -> impl Responder {
  
    // state variables
    let distance = info.0;
    let year_of_production = info.1;
    let fuel_usage_per_100km = info.2;

    // change to float
    let distance_float = distance as f64;

    let fuel_usage_per_100km_float = fuel_usage_per_100km as f64;

    // in 2006 first model was produced
    let first_model: Model = Model {
        name: "PeopleCar PasWagon C6".to_string(),
        age: 2006,
    };

    // cant calculate for non existing models

    if year_of_production >= first_model.age {
        if year_of_production == 2006 {
            let _d = ((distance_float * fuel_usage_per_100km_float) / 100.0_f64)
                * calculate_fuel_multiplier_per_prod_year(year_of_production as f64);

            format!("{} Liters", _d)
        } else if year_of_production <= 2010 {
            let _d = ((distance_float * fuel_usage_per_100km_float) / 100.0_f64)
                * calculate_fuel_multiplier_per_prod_year(year_of_production as f64);
            format!("{} Liters", _d)
        } else if year_of_production <= 2015 {
            let _d = ((distance_float * fuel_usage_per_100km_float) / 100.0_f64)
                * calculate_fuel_multiplier_per_prod_year(year_of_production as f64);
            format!("{} Liters", _d)
        } else if year_of_production <= 2020 {
            let _d = ((distance_float * fuel_usage_per_100km_float) / 100.0_f64)
                * calculate_fuel_multiplier_per_prod_year(year_of_production as f64);
            format!("{} Liters", _d)
        } else {
            let _d = ((distance_float * fuel_usage_per_100km_float) / 100.0_f64)
                * calculate_fuel_multiplier_per_prod_year(year_of_production as f64);
            format!("{} Liters", _d)
        }
    } else {
        format!("not recognized {}", year_of_production)
    }
}
#[get("/probabilityOfUnitInjectorFail/{VIN}")]
async fn fac(info: web::Path<(String)>) -> impl Responder {
    let mut rng_num = rand::thread_rng();
    let random_number = rng_num.gen_range(0.0..100.0);
    let propability_of_injection = random_number / 100.0_f64;
    format!("{}% for VIN: {}", propability_of_injection, info)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/calculateDisselUsageForDistance",
                web::get().to(|| async { "calculation of Dissel usage for distance" }),
            )
            .service(index)
            .route(
                "/probabilityOfUnitInjectorFail",
                web::get().to(|| async { "probability of unit injector fail" }),
            )
            .service(fac)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
