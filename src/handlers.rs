use actix_web::{
    body::BoxBody,
    error::ErrorInternalServerError,
    get,
    http::header::ContentType,
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use colored_json::ToColoredJson;
use log::info;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};

use crate::{alert::get_alerts, Record, Transaction, TxnType};

#[derive(Deserialize, Serialize)]
pub(crate) struct ReqPayload {
    pub r#type: TxnType,
    pub amount: String,
    pub user_id: u32,
    pub time: u32,
}

#[derive(Serialize)]
struct Response {
    alert: bool,
    alert_codes: Vec<u32>,
    user_id: u32,
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(
        self,
        _req: &HttpRequest,
    ) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/event")]
pub async fn event(
    json: Json<ReqPayload>,
    db: Data<Surreal<Db>>,
) -> Result<impl Responder, actix_web::Error> {
    let _: Option<Record> = db
        .create("transactions")
        .content(Transaction {
            user_id: json.user_id,
            amount: json.amount.parse::<f64>().unwrap(),
            r#type: TxnType::Deposit,
            time: json.time,
        })
        .await
        .map_err(ErrorInternalServerError)?;

    info!(
        "New transaction: {}\n",
        serde_json::to_string_pretty(&json)?
            .to_colored_json_auto()?
    );

    /* Since, we are not tracking user's account balance, we are simply adding the transaction to
    * the list of transactions. However, with a user account model, we could deligate the request
    * to withdrawal and deposit handlers as such:
    *
        match json.r#type {
            PayloadType::Deposit => {
                handle_deposit(
                    json.amount.parse::<f64>().unwrap(),
                    json.user_id,
                )
                .await?
            }
            PayloadType::Withdraw => {
                handle_withdraw(
                    json.amount.parse::<f64>().unwrap(),
                    json.user_id,
                )
                .await?
            }
        };
    */

    let query = format!(
        "SELECT * FROM type::table($table) WHERE user_id = {} ORDER BY time DESC;", 
            json.user_id);

    let mut result = db
        .query(query)
        .bind(("table", "transactions"))
        .await
        .map_err(ErrorInternalServerError)?;

    let transactions: Vec<Transaction> =
        result.take(0).map_err(ErrorInternalServerError)?;

    let alerts = get_alerts(&transactions).await;

    Ok(Response {
        alert: !alerts.is_empty(),
        alert_codes: alerts.into_iter().map(|x| x as u32).collect(),
        user_id: json.user_id,
    })
}

#[get("/transactions")]
async fn get_transactions(
    data: Data<Surreal<Db>>,
) -> Result<impl Responder, actix_web::Error> {
    let txns: Vec<Transaction> = data
        .select("transactions")
        .await
        .map_err(ErrorInternalServerError)?;

    dbg!(&txns);

    Ok(HttpResponse::Ok().json(txns))
}
