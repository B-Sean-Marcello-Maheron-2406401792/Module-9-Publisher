a. Berapa banyak data yang akan dikirim oleh program publisher dalam satu kali jalan?

Berdasarkan kode program yang diimplementasikan pada fungsi main, publisher akan mengirimkan sebanyak lima buah pesan atau event ke message broker dalam satu kali eksekusi . Setiap pesan tersebut berisi data UserCreatedEventMessage dengan rincian user_id dan user_name yang berbeda-beda, mulai dari data untuk Amir, Budi, Cica, Dira, hingga Emir . Pengiriman ini dilakukan secara berurutan menggunakan fungsi publish_event yang diarahkan ke antrean dengan nama user_created.

b. Apa arti dari URL amqp://guest:guest@localhost:5672 yang sama dengan program subscriber?

Kesamaan URL tersebut menunjukkan bahwa kedua program, baik publisher maupun subscriber, terhubung ke infrastruktur message broker yang sama agar dapat saling berkomunikasi. Dalam arsitektur event-driven, publisher memerlukan alamat tersebut untuk mengetahui ke mana harus mengirimkan pesan, sementara subscriber menggunakannya untuk mengetahui dari mana harus mengambil atau mendengarkan pesan . Karena keduanya menggunakan alamat localhost dengan port 5672, hal ini menandakan bahwa kedua program tersebut beroperasi sebagai satu ekosistem yang terhubung melalui perantara atau broker yang sama yang berjalan di mesin lokal. 
