a. Berapa banyak data yang akan dikirim oleh program publisher dalam satu kali jalan?

Berdasarkan kode program yang diimplementasikan pada fungsi main, publisher akan mengirimkan sebanyak lima buah pesan atau event ke message broker dalam satu kali eksekusi . Setiap pesan tersebut berisi data UserCreatedEventMessage dengan rincian user_id dan user_name yang berbeda-beda, mulai dari data untuk Amir, Budi, Cica, Dira, hingga Emir . Pengiriman ini dilakukan secara berurutan menggunakan fungsi publish_event yang diarahkan ke antrean dengan nama user_created.

b. Apa arti dari URL amqp://guest:guest@localhost:5672 yang sama dengan program subscriber?

Kesamaan URL tersebut menunjukkan bahwa kedua program, baik publisher maupun subscriber, terhubung ke infrastruktur message broker yang sama agar dapat saling berkomunikasi. Dalam arsitektur event-driven, publisher memerlukan alamat tersebut untuk mengetahui ke mana harus mengirimkan pesan, sementara subscriber menggunakannya untuk mengetahui dari mana harus mengambil atau mendengarkan pesan . Karena keduanya menggunakan alamat localhost dengan port 5672, hal ini menandakan bahwa kedua program tersebut beroperasi sebagai satu ekosistem yang terhubung melalui perantara atau broker yang sama yang berjalan di mesin lokal. 

### Tampilan running RabbitMQ

![img.png](img.png)

### Tampilan pengiriman event ke message broker
![img_2.png](img_2.png)

Pada tangkapan layar di atas, terjadi proses **Event-Driven Communication** sebagai berikut:
1. **Publisher** dijalankan dan berhasil membuat koneksi ke RabbitMQ server di `localhost:5672`.
2. Publisher mengirimkan (**Publish**) 5 buah event `UserCreatedEventMessage` ke antrean (*queue*) bernama `user_created`.
3. Data yang dikirim diserialisasi menggunakan format **Borsh** untuk memastikan efisiensi payload.
4. **Subscriber** yang sedang dalam posisi *listening*, secara otomatis mendeteksi adanya pesan baru di dalam queue.
5. Subscriber mengonsumsi (**Consume**) pesan tersebut, melakukan deserialisasi, dan mencetak datanya ke konsol dengan **Sean's Computer [2406401792]**.
