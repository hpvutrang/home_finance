use std::time::Duration;

use futures::{StreamExt, stream};
use tokio_postgres::AsyncMessage;
use tokio_postgres::{
    Config, Socket,
    tls::{MakeTlsConnect, TlsConnect},
};
use tracing::{Level, instrument};

pub trait NotificationHandler {
    fn on_notification_received(&self, channel: &str, message: &str);
}

pub struct DatabaseListener<T>
where
    T: MakeTlsConnect<Socket> + Clone + Sync + Send + 'static,
    T::Stream: Sync + Send,
    T::TlsConnect: Sync + Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pg_config: Config,
    tls: T,
}

impl<T> DatabaseListener<T>
where
    T: MakeTlsConnect<Socket> + Clone + Sync + Send + 'static,
    T::Stream: Sync + Send,
    T::TlsConnect: Sync + Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn new(pg_config: Config, tls: T) -> Self {
        Self { pg_config, tls }
    }

    #[instrument(name = "db_listener", level = Level::DEBUG, skip(handler, channel, self))]
    pub async fn attach<H>(self, handler: H, channel: &str)
    where
        H: NotificationHandler + Sync + Send + 'static,
    {
        let (client, mut connection) = self.pg_config.connect(self.tls).await.unwrap();

        // Notification listen
        let notif_span = tracing::span!(Level::TRACE, "receiver");
        tokio::spawn(async move {
            let _guard = notif_span.enter();

            let mut stream = stream::poll_fn(|cx| connection.poll_message(cx));
            while let Some(Ok(message)) = stream.next().await {
                if let AsyncMessage::Notification(notification) = message {
                    handler
                        .on_notification_received(notification.channel(), notification.payload());
                    tracing::event!(Level::TRACE, "received: {:?}", notification);
                }
            }

            drop(_guard);
        });

        let listen_query = format!("LISTEN {}", channel);
        client.simple_query(&listen_query).await.unwrap();

        // Heartbeat query to keep the connection alive
        let hearbeat_span = tracing::span!(Level::TRACE, "heartbeat");
        tokio::spawn(async move {
            let _guard = hearbeat_span.enter();

            loop {
                client.simple_query("SELECT 1").await.unwrap();

                tracing::event!(Level::TRACE, "Heart beat 10 seconds");
                tokio::time::sleep(Duration::from_secs(10)).await;
            }

            #[allow(unreachable_code)]
            drop(_guard);
        });
    }
}
