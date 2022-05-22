use std::env;

pub(crate) fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let imap_server = env::var("IMAP_SERVER").unwrap();
    let imap_username = env::var("IMAP_USERNAME").unwrap();
    let imap_password = env::var("IMAP_PASSWORD").unwrap();

    let client = imap::ClientBuilder::new(imap_server, 993).native_tls()?;

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(imap_username, imap_password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    let body = Ok(Some(body));
    let res: String = fetch_inbox_top().unwrap().unwrap();
    println!("{}", res);

    body
}
