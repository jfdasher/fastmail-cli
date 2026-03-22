//! GraphQL query resolvers

use async_graphql::{Context, Object, Result};

use super::types::*;

pub struct QueryRoot;

#[Object]
#[allow(clippy::too_many_arguments)]
impl QueryRoot {
    /// List all mailboxes (folders) with unread counts. Start here to discover available folders.
    async fn mailboxes(&self, ctx: &Context<'_>) -> Result<Vec<GqlMailbox>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let mut mailboxes = client.list_mailboxes().await?;
        mailboxes.sort_by(|a, b| match (&a.role, &b.role) {
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });
        Ok(mailboxes.into_iter().map(GqlMailbox::from).collect())
    }

    /// List emails in a specific mailbox/folder.
    async fn emails(
        &self,
        ctx: &Context<'_>,
        #[graphql(
            desc = "Mailbox name (e.g., 'INBOX', 'Sent') or role (e.g., 'inbox', 'sent', 'drafts')"
        )]
        mailbox: String,
        #[graphql(desc = "Maximum number of emails to return (default 25, max 100)")] limit: Option<
            u32,
        >,
    ) -> Result<Vec<GqlEmailSummary>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let limit = limit.unwrap_or(25).min(100);
        let mb = client.find_mailbox(&mailbox).await?;
        let emails = client.list_emails(&mb.id, limit).await?;
        Ok(emails.into_iter().map(Into::into).collect())
    }

    /// Get full content of a specific email by ID.
    async fn email(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The email ID (from emails or searchEmails queries)")] id: String,
    ) -> Result<Option<GqlEmail>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        match client.get_email(&id).await {
            Ok(email) => Ok(Some(GqlEmail(email))),
            Err(crate::error::Error::EmailNotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all emails in a thread/conversation. Returns emails sorted oldest-first.
    async fn thread(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Any email ID in the thread")] email_id: String,
    ) -> Result<GqlThread> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let mut emails = client.get_thread(&email_id).await?;
        emails.sort_by(|a, b| a.received_at.cmp(&b.received_at));
        let total = emails.len();
        Ok(GqlThread {
            emails: emails.into_iter().map(Into::into).collect(),
            total,
        })
    }

    /// Search emails with flexible filters.
    async fn search_emails(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "General search — searches subject, body, from, and to fields")]
        query: Option<String>,
        #[graphql(desc = "Search sender address/name")] from: Option<String>,
        #[graphql(desc = "Search recipient address/name")] to: Option<String>,
        #[graphql(desc = "Search CC recipients")] cc: Option<String>,
        #[graphql(desc = "Search subject line only")] subject: Option<String>,
        #[graphql(desc = "Search email body only")] body: Option<String>,
        #[graphql(desc = "Limit search to a specific mailbox/folder")] mailbox: Option<String>,
        #[graphql(desc = "Only emails with attachments")] has_attachment: Option<bool>,
        #[graphql(desc = "Emails before this date (YYYY-MM-DD or ISO 8601)")] before: Option<
            String,
        >,
        #[graphql(desc = "Emails after this date (YYYY-MM-DD or ISO 8601)")] after: Option<String>,
        #[graphql(desc = "Only unread emails")] unread: Option<bool>,
        #[graphql(desc = "Only flagged/starred emails")] flagged: Option<bool>,
        #[graphql(desc = "Maximum number of results (default 25, max 100)")] limit: Option<u32>,
    ) -> Result<Vec<GqlEmailSummary>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let limit = limit.unwrap_or(25).min(100);

        let filter = crate::commands::SearchFilter {
            text: query,
            from,
            to,
            cc,
            bcc: None,
            subject,
            body,
            mailbox: None,
            has_attachment: has_attachment.unwrap_or(false),
            min_size: None,
            max_size: None,
            before,
            after,
            unread: unread.unwrap_or(false),
            flagged: flagged.unwrap_or(false),
        };

        let mailbox_id = if let Some(ref name) = mailbox {
            client.find_mailbox(name).await.ok().map(|m| m.id)
        } else {
            None
        };

        let emails = client
            .search_emails_filtered(&filter, mailbox_id.as_deref(), limit)
            .await?;
        Ok(emails.into_iter().map(Into::into).collect())
    }

    /// List all sender identities on the account.
    async fn identities(&self, ctx: &Context<'_>) -> Result<Vec<GqlIdentity>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let identities = client.list_identities().await?;
        Ok(identities.into_iter().map(GqlIdentity::from).collect())
    }

    /// List attachments on an email.
    async fn attachments(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The email ID")] email_id: String,
    ) -> Result<Vec<GqlAttachmentInfo>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let email = client.get_email(&email_id).await?;
        Ok(email
            .attachments
            .as_ref()
            .map(|atts| {
                atts.iter()
                    .filter(|a| a.blob_id.is_some())
                    .map(GqlAttachmentInfo::from)
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Download an attachment. Images are resized and base64-encoded. Documents have text extracted.
    async fn attachment(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The email ID the attachment belongs to")] email_id: String,
        #[graphql(desc = "The blob ID of the attachment (from attachments query)")] blob_id: String,
    ) -> Result<GqlAttachmentContent> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;

        let email = client.get_email(&email_id).await?;
        let attachment = email
            .attachments
            .as_ref()
            .and_then(|atts| atts.iter().find(|a| a.blob_id.as_deref() == Some(&blob_id)));

        let attachment = attachment
            .ok_or_else(|| async_graphql::Error::new(format!("Attachment not found: {blob_id}")))?;

        let content_type = attachment
            .content_type
            .as_deref()
            .unwrap_or("application/octet-stream");
        let name = attachment.name.as_deref().unwrap_or("attachment");

        let data = client.download_blob(&blob_id).await?;

        let mime = if crate::util::is_image(content_type, name) {
            crate::util::infer_image_mime(name).unwrap_or(content_type)
        } else {
            content_type
        };

        // Images
        if crate::util::is_image(mime, name) {
            return match crate::util::resize_image(&data, mime, crate::util::MCP_IMAGE_MAX_BYTES) {
                Ok((processed_data, _mime_type)) => {
                    let base64_data = base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        &processed_data,
                    );
                    Ok(GqlAttachmentContent {
                        blob_id,
                        name: name.to_string(),
                        content_type: mime.to_string(),
                        size: processed_data.len(),
                        base64_content: Some(base64_data),
                        text_content: None,
                        info: None,
                    })
                }
                Err(e) => Err(async_graphql::Error::new(format!(
                    "Failed to process image: {e}"
                ))),
            };
        }

        // Documents — extract text
        match crate::util::extract_text(&data, name).await {
            Ok(Some(text)) => {
                return Ok(GqlAttachmentContent {
                    blob_id,
                    name: name.to_string(),
                    content_type: content_type.to_string(),
                    size: data.len(),
                    base64_content: None,
                    text_content: Some(text),
                    info: None,
                });
            }
            Ok(None) => {}
            Err(e) => {
                return Err(async_graphql::Error::new(format!(
                    "Failed to extract text: {e}"
                )));
            }
        }

        // Binary fallback
        Ok(GqlAttachmentContent {
            blob_id,
            name: name.to_string(),
            content_type: content_type.to_string(),
            size: data.len(),
            base64_content: None,
            text_content: None,
            info: Some("Binary attachment — cannot be displayed directly.".to_string()),
        })
    }

    /// List all masked email addresses.
    async fn masked_emails(&self, ctx: &Context<'_>) -> Result<Vec<GqlMaskedEmail>> {
        let client = ctx.data::<tokio::sync::Mutex<crate::jmap::JmapClient>>()?;
        let client = client.lock().await;
        let mut masked = client.list_masked_emails().await?;
        masked.sort_by(|a, b| {
            let a_enabled = a.state.as_deref() == Some("enabled");
            let b_enabled = b.state.as_deref() == Some("enabled");
            match (a_enabled, b_enabled) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.email.cmp(&b.email),
            }
        });
        Ok(masked.into_iter().map(GqlMaskedEmail::from).collect())
    }

    /// Search contacts by name, email, or organization. Requires FASTMAIL_APP_PASSWORD.
    async fn contacts(
        &self,
        #[graphql(desc = "Search query — matches name, email, or organization")] query: String,
    ) -> Result<Vec<GqlContact>> {
        let config = crate::config::Config::load()?;
        let username = config.get_username().map_err(|_| {
            async_graphql::Error::new("Username not configured. Set FASTMAIL_USERNAME env var.")
        })?;
        let app_password = config.get_app_password().map_err(|_| {
            async_graphql::Error::new(
                "App password not configured. Set FASTMAIL_APP_PASSWORD env var (API tokens don't work for CardDAV).",
            )
        })?;

        let client = crate::carddav::CardDavClient::new(username, app_password);
        let contacts = client.search_contacts(&query).await?;
        Ok(contacts.into_iter().map(GqlContact::from).collect())
    }
}
