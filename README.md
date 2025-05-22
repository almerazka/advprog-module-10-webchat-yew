**Tutorial 10 Pemrograman Lanjut (Advanced Programming) 2024/2025 Genap**
* Nama    : Muhammad Almerazka Yocendra
* NPM     : 2306241745
* Kelas   : Pemrograman Lanjut - A

## 🍊 Original Code

![image](https://github.com/user-attachments/assets/7c5d250d-7fda-4aa2-9d14-4d7ca44b7a36)

![image](https://github.com/user-attachments/assets/68185e62-fac4-4d74-a860-73cd472981f2)

Berdasarkan gambar yang ditampilkan, telah berhasil dilakukan implementasi _webchat_ menggunakan **Yew** _framework_ dengan mengkloning dua repository : **YewChat** dari branch `websockets-part2` dan **SimpleWebsocketServer**. Aplikasi _webchat_ ini memiliki _interface_ yang _user-friendly_ dengan halaman login yang meminta _username_ sebelum memasuki _chat room_, kemudian menampilkan daftar _users_ yang aktif di _sidebar_ kiri dan area _chat_ di sebelah kanan. Setelah menjalankan `npm install` dan `npm start` pada keduanya yaitu **YewChat** dan **SimpleWebSocketServer**, aplikasi dapat berfungsi dengan baik dimana users seperti **"Almer"** dan **"Azka"** dapat saling berinteraksi dalam _real-time_ chat.

🍋 Be Creative!

![image](https://github.com/user-attachments/assets/babcffe2-bdc7-43c3-976b-ac9eaa303a21)

![image](https://github.com/user-attachments/assets/fa2e5415-4a27-4984-80e0-162c89a6253d)

Saya menambahkan tiga perubahan 
- **dark mode toggle** dengan menambah `field dark_mode: bool` di _struct_ **Chat** dan **Msg::ToggleDarkMode** untuk _switch_ tema
- **avatar hewan** dengan mengganti URL avatar dari **Dicebear** ke **Robohash** menggunakan `"https://robohash.org/{}?set=set4&size=200x200"` untuk mendapat avatar kucing/hewan yang unik per _username_
- **message bubbles** yang berbeda dimana pesan sendiri menggunakan _gradient_ biru-ungu `(bg-gradient-to-r from-blue-500 to-purple-500)` dengan posisi kanan dan _rounded_ _corner_ khusus, sedangkan pesan orang lain pakai warna `abu-abu/putih` dengan posisi kiri.
- Saya juga menambahkan _user identification_ dengan badge **"You"** untuk _user_ sendiri, _online indicator_ hijau untuk semua _user_, dan _enhanced styling_ dengan _gradients_ dan _shadows_ yang membuat tampilan lebih _modern_ dan _user-friendly_
