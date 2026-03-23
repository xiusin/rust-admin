use super::common::periodic;
use crate::common::error::Result;

pub async fn clear_periodic_worker() -> Result<()> {
    periodic::destroy_all().await
}
 

pub async fn periodic_worker<Args>(
    cron_str: &str,
    name: &str,
    queue: &str,
    args: Args,
    class_name: String,
) -> Result<()>
where
    Args: Sync + Send + for<'de> serde::Deserialize<'de> + serde::Serialize + 'static,
{
    let d = periodic::builder(cron_str)?
        .name(name)
        .queue(queue)
        .args(args)?
        .into_periodic_job(class_name)?;

    let payload = serde_json::to_string(&d)?;
    d.update(&payload).await?;
    
    Ok(())
}
