# 🧮 Unit Converter (CLI)

Program berbasis **Command Line Interface (CLI)** untuk melakukan konversi berbagai satuan seperti **panjang** dan **berat**.  
Dibuat agar mudah digunakan, sederhana, dan cocok dijadikan proyek pembelajaran atau kontribusi awal di folder `tools/console/`.

---

## ✨ Fitur Utama
- Konversi **panjang**: `m`, `cm`, `mm`, `km`, `inch`, `ft`, `yd`, `mile`
- Konversi **berat**: `kg`, `g`, `mg`, `lb`, `oz`, `ton`
- Tersedia mode **interaktif** langsung di terminal
- Validasi input dengan pesan error yang mudah dipahami
- Tidak memerlukan dependensi eksternal (dibuat dengan **pure Python**)

---

## 🧾 Prasyarat
- **Python 3.x** sudah terinstal di sistem Anda

---

## 🚀 Instalasi & Cara Menjalankan

Clone repositori berikut (atau tambahkan folder `unit-converter` ke dalam repo utama Anda):
```bash
git clone https://github.com/antidonasi/hacktoberfest.git
cd hacktoberfest/tools/console/unit-converter
```
Jalankan program dengan perintah:
```bash
python src/onverter.py
```
> Jika sistem Anda menggunakan `python3` sebagai alias, gunakan:
> python3 src/converter.py

---

## 💻 Contoh Penggunaan (Mode Interaktif)
```bash
$ python converter.py
Pilih mode (length/weight): length  
Masukkan nilai: 10  
Dari unit: cm  
Ke unit: inch  

hasil 10 cm = 3.9370 inch  
```
---

## 🔧 Contoh Penggunaan (Non-Interaktif / Scriptable)

Anda juga dapat memanggil fungsi konversi langsung dari kode Python lain, misalnya:
```py
from src.unit_converter import convert_length

print(convert_length(10, "cm", "inch"))
```
---

## 🧩 Daftar Unit yang Didukung

**Panjang (Length):**  
`m`, `cm`, `mm`, `km`, `inch`, `ft`, `yd`, `mile`  

**Berat (Weight):**  
`kg`, `g`, `mg`, `lb`, `oz`, `ton`

---

## ❗ Penanganan Error

Jika pengguna memasukkan satuan yang tidak dikenali atau nilai yang tidak valid, program akan menampilkan pesan kesalahan yang jelas untuk membantu memperbaiki input.

---

## 🧠 Rencana Pengembangan (Kontribusi Terbuka)

Beberapa ide pengembangan yang dapat Anda bantu kembangkan:
- Menambahkan konversi **suhu** (`C`, `F`, `K`)
- Menambahkan kategori **volume** (`liter`, `ml`, `gallon`)
- Menambahkan mode CLI **non-interaktif** (`--value`, `--from`, `--to`, `--mode`)
- Menambahkan **alias unit** (contoh: `feet` → `ft`, `kilogram` → `kg`)
- Menambahkan **unit test** dengan `pytest` dan integrasi CI melalui GitHub Actions

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah **MIT License**.  
Lihat berkas [LICENSE](../../../LICENSE) untuk informasi selengkapnya.

---

**Dikembangkan oleh:** [ravaachmad](https://github.com/ravaachmad)  
**Kategori:** Console Tools  
**Bahasa:** Python
