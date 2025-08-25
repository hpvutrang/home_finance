use std::time::Duration;

use futures::{StreamExt, stream};
use tokio_postgres::AsyncMessage;
use tokio_postgres::{
    Config, Socket,
    tls::{MakeTlsConnect, TlsConnect},
};

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

    pub async fn attach<H>(self, handler: H, channel: &str)
    where
        H: NotificationHandler + Sync + Send + 'static,
    {
        let (client, mut connection) = self.pg_config.connect(self.tls).await.unwrap();

        // Notification listen
        tokio::spawn(async move {
            let mut stream = stream::poll_fn(|cx| connection.poll_message(cx));
            while let Some(Ok(message)) = stream.next().await {
                if let AsyncMessage::Notification(notification) = message {
                    handler
                        .on_notification_received(notification.channel(), notification.payload());
                }
            }
        });

        let listen_query = format!("LISTEN {}", channel);
        client.simple_query(&listen_query).await.unwrap();

        // Heartbeat query to keep the connection alive
        tokio::spawn(async move {
            loop {
                client.simple_query("SELECT 1").await.unwrap();
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        });
    }
}
