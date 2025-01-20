use aws_config::{defaults, Region};

use aws_sdk_sesv2::{
    config::{BehaviorVersion, Credentials},
    error::SdkError,
    operation::send_email::SendEmailError,
    types::{Body, Content, Destination, EmailContent, Message},
    Client,
};

pub async fn send_email(
    email: String,
    subject: Content,
    body: Content,
) -> Result<String, SdkError<SendEmailError>> {
    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(
            option_env!("AWS_REGION").unwrap_or("ap-northeast-2"),
        ))
        .credentials_provider(Credentials::new(
            option_env!("AWS_ACCESS_KEY_ID")
                .expect("AWS_ACCESS_KEY_ID is required")
                .to_string(),
            option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required")
                .to_string(),
            None,
            None,
            "democrasee",
        ))
        .load()
        .await;

    let sms_client = Client::new(&config);

    let dest = Destination::builder()
        .set_to_addresses(Some(vec![email]))
        .build();

    let body = Body::builder().text(body).build();

    let msg = Message::builder().subject(subject).body(body).build();

    let content = EmailContent::builder().simple(msg).build();

    match sms_client
        .send_email()
        .from_email_address("hi@biyard.co")
        .destination(dest)
        .content(content)
        .send()
        .await
    {
        Ok(v) => Ok(v.message_id.expect("Wrong Message Id")),
        Err(e) => Err(e),
    }
}

pub fn build_content(data: String) -> Content {
    Content::builder()
        .data(data)
        .build()
        .expect("Failed to build Content")
}
