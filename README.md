a. Berapa banyak data yang akan dikirim oleh program publisher dalam satu kali jalan?

Berdasarkan kode program yang diimplementasikan pada fungsi main, publisher akan mengirimkan sebanyak lima buah pesan atau event ke message broker dalam satu kali eksekusi . Setiap pesan tersebut berisi data UserCreatedEventMessage dengan rincian user_id dan user_name yang berbeda-beda, mulai dari data untuk Amir, Budi, Cica, Dira, hingga Emir . Pengiriman ini dilakukan secara berurutan menggunakan fungsi publish_event yang diarahkan ke antrean dengan nama user_created.

b. Apa arti dari URL amqp://guest:guest@localhost:5672 yang sama dengan program subscriber?

Kesamaan URL tersebut menunjukkan bahwa kedua program, baik publisher maupun subscriber, terhubung ke infrastruktur message broker yang sama agar dapat saling berkomunikasi. Dalam arsitektur event-driven, publisher memerlukan alamat tersebut untuk mengetahui ke mana harus mengirimkan pesan, sementara subscriber menggunakannya untuk mengetahui dari mana harus mengambil atau mendengarkan pesan . Karena keduanya menggunakan alamat localhost dengan port 5672, hal ini menandakan bahwa kedua program tersebut beroperasi sebagai satu ekosistem yang terhubung melalui perantara atau broker yang sama yang berjalan di mesin lokal. 

### Running RabbitMQ as message broker.

![img.png](img.png)

### Sending and processing event.
![img_2.png](img_2.png)

Pada tangkapan layar di atas, terjadi proses **Event-Driven Communication** sebagai berikut:
1. **Publisher** dijalankan dan berhasil membuat koneksi ke RabbitMQ server di `localhost:5672`.
2. Publisher mengirimkan (**Publish**) 5 buah event `UserCreatedEventMessage` ke antrean (*queue*) bernama `user_created`.
3. Data yang dikirim diserialisasi menggunakan format **Borsh** untuk memastikan efisiensi payload.
4. **Subscriber** yang sedang dalam posisi *listening*, secara otomatis mendeteksi adanya pesan baru di dalam queue.
5. Subscriber mengonsumsi (**Consume**) pesan tersebut, melakukan deserialisasi, dan mencetak datanya ke konsol dengan **Sean's Computer [2406401792]**.

### Monitoring chart based on publisher

![img_1.png](img_1.png)

Lonjakan (spike) pada grafik di atas merepresentasikan aktivitas pengiriman pesan dari **Publisher** ke **Message Broker**.
* Setiap kali perintah `cargo run` dijalankan pada Publisher, 5 event dikirimkan secara instan ke RabbitMQ. Hal ini menyebabkan kenaikan mendadak pada laju pesan masuk (*incoming rate*).
* Jika Publisher dijalankan berulang kali dalam waktu singkat, grafik akan menunjukkan beberapa puncak (spike) yang berdekatan.
* Setelah pesan diterima oleh broker, Subscriber akan langsung mengambilnya (jika aktif), sehingga grafik akan turun kembali ke angka nol setelah semua pesan berhasil diproses.


# BONUS

### “Sending and processing event.”

![img_4.png](img_4.png)



![img_3.png](img_3.png)