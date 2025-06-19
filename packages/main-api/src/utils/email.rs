use aws_config::{Region, defaults};

use aws_sdk_sesv2::{
    Client,
    config::{BehaviorVersion, Credentials},
    error::SdkError,
    operation::send_email::SendEmailError,
    types::{Body, Content, Destination, EmailContent, Message},
};
pub async fn send_email(
    email: String,
    subject: Content,
    body: Content,
) -> Result<String, SdkError<SendEmailError>> {
    let conf = crate::config::get();
    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(conf.aws.region))
        .credentials_provider(Credentials::new(
            conf.aws.access_key_id,
            conf.aws.secret_access_key,
            None,
            None,
            "ratel",
        ));

    let config = config.load().await;
    let sms_client = Client::new(&config);

    let dest = Destination::builder()
        .set_to_addresses(Some(vec![email]))
        .build();

    let body = Body::builder().text(body).build();

    let msg = Message::builder().subject(subject).body(body).build();

    let content = EmailContent::builder().simple(msg).build();

    match sms_client
        .send_email()
        .from_email_address(conf.from_email)
        .destination(dest)
        .content(content)
        .send()
        .await
    {
        Ok(v) => Ok(v.message_id.expect("Wrong Message Id")),
        Err(e) => Err(e),
    }
}
