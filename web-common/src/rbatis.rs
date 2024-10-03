use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error, RBatis};
use rbdc_pg::PgDriver;
use rbs::Value;
use rocket::fairing::AdHoc;

use crate::core::utils::config::get_config;


#[derive(Debug)]
struct InsertReturnIdPlugin {}

#[async_trait]
impl Intercept for InsertReturnIdPlugin {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if sql.contains("insert into") {
            let new_sql = format!("{} {}", sql, "returning id");

            if let ResultType::Exec(exec_r) = result {
                // !重复执行
                let id = rb.query(&new_sql, args.clone()).await?;
                let exec = ExecResult {
                    rows_affected: id.len() as u64,
                    last_insert_id: id.into_array().unwrap().last().unwrap()["id"].clone(),
                };

                *exec_r = Ok(exec);
            }
        }

        Ok(Some(true))
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init sql", |rocket| async {
        let sql_addr = get_config("database.postgres.url").unwrap().into_string().unwrap();

        let rb = RBatis::new();
        rb.link(PgDriver {}, &sql_addr).await.unwrap();

        // rb.intercepts.insert(0, Arc::new(InsertReturnIdPlugin {}));

        rocket.manage(rb)
    })
}