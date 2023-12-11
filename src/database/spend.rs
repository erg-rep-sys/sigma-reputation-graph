use std::fmt::Debug;
use std::io::Error;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
struct ReputationProof {
    proof_id: Option<String>,
    amount: i64
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Deserialize)]
struct CountRecord {
    #[allow(dead_code)]
    count: i16,
}

const DB_ERROR_MSG: &str = "Invalid response or error connection from the database";
const NAMESPACE: &str = "local";
const DATABASE: &str = "graph";
const ENDPOINT: &str = "127.0.0.1:8000";
const RESOURCE: &str = "reputation_proof";


#[tokio::main]
async fn get_proof_db_id(id: &str) -> Result<String, Error> {
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);

    let local_id = RESOURCE.to_owned() + ":" + id;
        
    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    let sql = "SELECT count() FROM $id";
    let mut response = db.query(sql)
        .bind(("id", &local_id))
        .await.expect(DB_ERROR_MSG);


    let response: Option<CountRecord> = response.take(0).expect(DB_ERROR_MSG);

    match response {
        Some(record) => match record.count {
                1_i16..=i16::MAX => {
                    Ok(local_id)
                },
                0 => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Amount cannot be zero"),
                )),
                i16::MIN..=-1_i16 => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Amount cannot negative"),
                ))
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to retrieve amount from database"),
        )),
    }
}


/**
 * Based on:
 *  https://stackoverflow.com/a/62536772/11370826
 * 
 */

#[tokio::main]
pub async fn store_on_db(proof_id: Option<String>, amount: i64) 
    -> Result<String, std::io::Error> 
{
    let db = Surreal::new::<Ws>(ENDPOINT)
        .await.expect(DB_ERROR_MSG);
        
    db.use_ns(NAMESPACE).use_db(DATABASE).await.expect(DB_ERROR_MSG);

    let id_result: Result<Option<String>, Error> = match proof_id {
        None => Ok(None),
        Some(id) => {
            let response = tokio::task::spawn_blocking(move || {
                get_proof_db_id(id.as_str())
            }).await.expect("Blocking task panicked");
            match response {
                Ok(id) => Ok(Some(id)),
                Err(error) => Err(error)
            }
        }
    };

    match id_result {
        Ok(proof_id) => {
            // Create a new person with a random id
            let created: Vec<Record> = db
                .create(RESOURCE)
                .content(ReputationProof {
                    proof_id,
                    amount
                })
                .await
                .expect(DB_ERROR_MSG);

            let raw_id = created.first().unwrap().id.to_string();
            Ok(raw_id)
        },
        Err(error) => Err(error)
    }

}