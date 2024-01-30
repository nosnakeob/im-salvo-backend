#[catch(404)]
pub async fn not_found() -> &'static str { "404" }

#[catch(401)]
pub async fn unauthorized() -> &'static str { "401" }
