use super::models::StripeUpdate;
use crate::prelude::*;

pub async fn persist_update_op(
    db: impl PgExecutor<'_>,
    update: &StripeUpdate,
) -> Aresult<()> {
    query!(
        "update users set subscription_type_id = $1
        where stripe_customer_id = $2",
        update.subscription_type.as_int(),
        update.stripe_customer_id
    )
    .execute(db)
    .await?;
    Ok(())
}
