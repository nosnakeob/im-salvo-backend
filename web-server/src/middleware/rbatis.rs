use anyhow::Result;
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use salvo::prelude::*;

#[handler]
pub async fn set_db(depot: &mut Depot) -> Result<()> {
    let rb = RBatis::new();
    rb.link(PgDriver {}, "postgres://postgres:135246@localhost/postgres")
        .await?;
    depot.inject(rb);
    Ok(())
}
