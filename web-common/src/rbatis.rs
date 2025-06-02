use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, Error};
use rbs::Value;

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
