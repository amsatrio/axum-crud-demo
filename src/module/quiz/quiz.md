https://news.detik.com/berita/d-8293108/nadiem-minta-dibebaskan-dari-dakwaan-kasus-korupsi-chromebook-rp-2-1-t
c
Nadiem Makarim terjerat kasus dugaan korupsi pengadaan...
a. Chromium
b. Google Chrome
c. Chromebook

https://www.bbc.com/indonesia/articles/cn475k3gwk3o
a
Siapa yang dilindas kendaraan taktis Brimob dalam unjuk rasa 28 Agustus 2025 lalu?
a. Affan Kurniawan 
b. Affan Hermawan
c. Affan Sulaiman

https://en.wikipedia.org/wiki/Prabowo_Subianto
c
Berapa presiden Indonesia saat ini?
a. Prabowo
b. Jokowi
c. 1

https://www.detik.com/sulsel/berita/d-7517017/40-contoh-soal-twk-cpns-2024-terbaru-lengkap-kunci-jawaban-pembahasannya
b
Salah satu ancaman terhadap keutuhan NKRI adalah...
a. Semangat gotong royong
b. Radikalisme
c. Demokrasi

https://www.detik.com/sulsel/berita/d-7517017/40-contoh-soal-twk-cpns-2024-terbaru-lengkap-kunci-jawaban-pembahasannya
a
Siapa proklamator kemerdekaan Indonesia?
a. Ir. Soekarno
b. Nadiem Makarim
c. Prabowo Subianto

https://tirto.id/berapa-gaji-sopir-mbg-yuk-cek-besaran-dan-tugasnya-apa-saja-hjte
a
Kisaran berapa gaji supir MBG di Indonesia?
a. 80k - 100k / hari
b. 60k - 70k / hari
c. 160k - 200k / hari

https://www.detik.com/sulsel/berita/d-7517017/40-contoh-soal-twk-cpns-2024-terbaru-lengkap-kunci-jawaban-pembahasannya
b
Prinsip demokrasi yang paling mendasar adalah...
a. Kekuasaan mutlak berada di tangan presiden
b. Kedaulatan berada di tangan rakyat
c. Adanya satu partai politik yang berkuasa

https://www.detik.com/sulsel/berita/d-7517017/40-contoh-soal-twk-cpns-2024-terbaru-lengkap-kunci-jawaban-pembahasannya
b
Fungsi utama lembaga legislatif adalah...
a. Melaksanakan undang-undang
b. Membuat undang-undang
c. Menegakkan hukum

https://www.detik.com/sulsel/berita/d-7517017/40-contoh-soal-twk-cpns-2024-terbaru-lengkap-kunci-jawaban-pembahasannya
c
Lembaga negara yang bertugas membuat undang-undang adalah...
a. Presiden
b. Mahkamah Konstitusi
c. Dewan Perwakilan Rakyat

https://www.youtube.com/watch?v=1rumaHS0ghI
c
Nama lagu kebangsaan Indonesia adalah...
a. Garuda Pancasila
b. Indonesia Pusaka
c. Indonesia Raya






```sql

SELECT 
    u.username,
    COUNT(a.id) AS total_correct,
    (MAX(umc.created_on) - MIN(umc.created_on)) AS duration_seconds,
    MAX(umc.created_on),
    MIN(umc.created_on)
FROM 
    m_user u
JOIN 
    m_user_multiple_choice umc ON u.id = umc.m_user_id
LEFT JOIN 
    m_answer a ON umc.m_question_id = a.m_question_id 
               AND umc.m_multiple_choice_id = a.multiple_choice_id
GROUP BY 
    u.id, u.username
ORDER BY 
    total_correct DESC,
    umc.created_on ASC,
    duration_seconds ASC;

```