use crate::jmap::authenticated_client;
use crate::models::Output;

pub async fn list_masked_emails() -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let masked_emails = client.list_masked_emails().await?;

    Output::success(masked_emails).print();
    Ok(())
}

pub async fn create_masked_email(
    for_domain: Option<&str>,
    description: Option<&str>,
    prefix: Option<&str>,
) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let masked_email = client
        .create_masked_email(for_domain, description, prefix)
        .await?;

    Output::success(masked_email).print();
    Ok(())
}

pub async fn enable_masked_email(id: &str) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    client
        .update_masked_email(id, Some("enabled"), None, None)
        .await?;

    Output::<()>::success_msg(format!("Masked email {} enabled", id)).print();
    Ok(())
}

pub async fn disable_masked_email(id: &str) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    client
        .update_masked_email(id, Some("disabled"), None, None)
        .await?;

    Output::<()>::success_msg(format!("Masked email {} disabled", id)).print();
    Ok(())
}

pub async fn delete_masked_email(id: &str) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    client
        .update_masked_email(id, Some("deleted"), None, None)
        .await?;

    Output::<()>::success_msg(format!("Masked email {} deleted", id)).print();
    Ok(())
}
