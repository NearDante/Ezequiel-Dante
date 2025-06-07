

use databento::{
        dbn::Schema, 
        historical::batch::SubmitJobParams,
        HistoricalClient,
    };
use time::macros::datetime;
use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    //DATABENTO_API_KEY="db-d5S9enKHu8pdAsfEVJaXyphbSxTGh"
    let api_key = "db-d5S9enKHu8pdAsfEVJaXyphbSxTGh";

    let mut client =
        HistoricalClient::builder().key(api_key)?.build()?;
    let job = client
        .batch()
        .submit_job(
            &SubmitJobParams::builder()
                .dataset("XNAS.ITCH")
                .date_time_range((
                    datetime!(2022-05-05 12:00 UTC),
                    datetime!(2022-05-06 00:00 UTC),
                ))
                .symbols("TSLA")
                .schema(Schema::Mbo)
                .build(),
        )
        .await?;
    println!("{job:#?}");

    let url = "https://hist.databento.com/v0/batch.list_jobs";

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .basic_auth(api_key, Some("")) // usuario=api_key, password vacío
        .query(&[
            ("states", "done"),
            ("since", "20250606T00:00:00"),
        ])
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

/*
    let files = client
        .batch()
        .list_files("XNAS-20250108-VVS57U5PD8")
        .await?;
    println!("{files:#?}");
*/

    println!("Hello, world!");
    Ok(())
}
