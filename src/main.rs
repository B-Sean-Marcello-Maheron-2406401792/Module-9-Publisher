use borsh::{BorshSerialize};
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};

#[derive(Debug, Clone, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    // 1. Koneksi ke RabbitMQ
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    let channel = conn.create_channel().await.unwrap();

    // 2. Deklarasi Queue (Pastikan namanya sama dengan subscriber)
    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions {
                durable: false,
                auto_delete: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap();

    // 3. Daftar pesan yang akan dikirim
    let messages = vec![
        ("1", "2406401792-Amir"),
        ("2", "2406401792-Budi"),
        ("3", "2406401792-Cica"),
        ("4", "2406401792-Dira"),
        ("5", "2406401792-Emir"),
    ];

    for (id, name) in messages {
        let event = UserCreatedEventMessage {
            user_id: id.to_owned(),
            user_name: name.to_owned(),
        };

        // Serialisasi data menggunakan Borsh ke format Vec<u8>
        let payload = event.try_to_vec().unwrap();

        // 4. Publish pesan ke queue
        channel
            .basic_publish(
                "",             // exchange
                "user_created", // routing key / queue name
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .unwrap();

        println!(" [x] Sent: {:?}", event);
    }

    // Menutup koneksi
    conn.close(0, "").await.unwrap();
}