use std::io::Error;
use surrealdb::sql::Thing;
use crate::database::generate::DatabaseAsync;
use crate::database::global::{*};

#[tokio::main]
pub async fn store_on_db(proof_id: Option<String>, amount: i64, pointer: Option<String>, database: DatabaseAsync)
    -> Result<String, Error>
{
    match database.await {
        Ok(db) => { 
            let _proof_id = proof_id.unwrap_or(String::from(""));
            let _pointer_id = pointer.unwrap_or(String::from(""));

            let result: Vec<RPBoxDBWithId> = db
                .query(
                    format!(
                        "SELECT * FROM {:?} WHERE pointer='{}' AND proof_id='{}'",
                        RESOURCE, &_pointer_id, &_proof_id
                    )
                )
                .await.expect(DB_ERROR_MSG)
                .take(1).expect(DB_ERROR_MSG);

            match &result[..] {

                [_s] => {  // Match if the query result has exactly one element.

                    let _updated: Option<Thing> = db
                        .update((RESOURCE, _s.id.as_str()))
                        .content(RPBoxDB {
                            proof_id: _s.proof_id.clone(),
                            pointer: _pointer_id.clone(),
                            amount: amount + _s.amount
                        })
                        .await.expect(DB_ERROR_MSG);

                    _s.id.clone()
                },
                _ => {
                    let created: Vec<Record> = db
                        .create(RESOURCE)
                        .content(RPBoxDB {
                            proof_id: _proof_id.clone(),
                            pointer: _pointer_id.clone(),
                            amount  // TODO could check that amount <= proof->amount
                        })
                        .await.expect(DB_ERROR_MSG);

                    created.first().unwrap().id.to_string()              
                }
            };

            Ok(String::from(""))
        },
        Err(err) => Err(err)
    }
}
