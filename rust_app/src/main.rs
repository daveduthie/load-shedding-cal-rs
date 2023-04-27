use lambda_http::{
    aws_lambda_events::serde_json::json, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};

use anyhow::Result;
use interval::Interval;
use tracing::info;

mod ical;
mod interval;
mod scrape;
mod timetable;

async fn get_calendar(zone_id: usize) -> Result<String> {
    let events: Vec<_> = scrape::schedule()
        .await?
        .iter()
        .flat_map(|load_shed_time| {
            timetable::timetable_for_stage_and_zone(
                load_shed_time.stage,
                zone_id,
                load_shed_time.start,
            )
            .into_iter()
            .filter_map(|t| {
                info!("Do these intersect? {:#?}, {:#?}", load_shed_time, t);
                let Interval { start, end } = interval::intersection(
                    t,
                    interval::interval(load_shed_time.start, load_shed_time.end)?,
                )?;
                Some(ical::event(start, end, &load_shed_time.title()))
            })
            .collect::<Vec<_>>()
        })
        .collect();

    Ok(ical::ical(&events))
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    Ok(
        match event
            .query_string_parameters()
            .first("zone_id")
            .map(|zone_id| zone_id.parse())
        {
            Some(Ok(zone_id)) => Response::builder()
                .status(200)
                .header("content-type", "text/calendar")
                .body(get_calendar(zone_id).await?.into())
                .map_err(Box::new)?,
            _ => Response::builder()
                .status(400)
                .body(
                    json!({"message": "Missing or malformed zone_id"})
                        .to_string()
                        .into(),
                )
                .expect("failed to render response"),
        },
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
