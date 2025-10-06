# 📚 KBBI Scraper

Tool scraper untuk mengambil data dari Kamus Besar Bahasa Indonesia (KBBI) online yang dibuat dengan Node.js.

## ✨ Features
- 🔍 Search kata dalam KBBI
- 📖 Ekstrak definisi dan arti kata
- 🎯 Ambil contoh penggunaan kata
- 💾 Export hasil ke JSON/TXT
- ⚡ Async processing untuk performa optimal
- 🔄 Batch processing untuk multiple kata

## 📋 Prerequisites
- Node.js 16.x+ 
- npm atau yarn
- Koneksi internet untuk akses KBBI online

## 🚀 Instalasi

1. **Clone repository utama**:
   ```bash
   git clone https://github.com/antidonasi/hacktoberfest.git
   cd hacktoberfest/tools/scrapers/kbbi-scraper-by-kanata
   ```

2. **Install dependencies**:
   ```bash
   npm install
   # atau
   yarn install
   ```

3. **Jalankan scraper**:
   ```bash
   node index.js
   ```

## 💻 Penggunaan

### Basic Usage:
```bash
# Search single kata
node index.js --word "teknologi"

# Search multiple kata
node index.js --words "teknologi,inovasi,digital"

# Export ke file
node index.js --word "komputer" --output result.json
```

### Interactive Mode:
```bash
$ node index.js

📚 KBBI Scraper by Kanata
========================
Masukkan kata yang ingin dicari: teknologi

🔍 Mencari: teknologi
✅ Ditemukan!

📖 Definisi:
1. penerapan keilmuan; cara melakukan sesuatu dengan menggunakan alat dan keahlian
2. keseluruhan sarana untuk menyediakan barang-barang yang diperlukan bagi kelangsungan dan kenyamanan hidup manusia

💡 Contoh Penggunaan:
- teknologi informasi semakin berkembang pesat
- teknologi modern memudahkan kehidupan manusia

💾 Hasil disimpan ke: teknologi_result.json
```

## 📄 Output Format

### JSON Output:
```json
{
  "word": "teknologi",
  "found": true,
  "definitions": [
    {
      "meaning": "penerapan keilmuan; cara melakukan sesuatu dengan menggunakan alat dan keahlian",
      "type": "noun"
    }
  ],
  "examples": [
    "teknologi informasi semakin berkembang pesat"
  ],
  "scraped_at": "2025-10-04T10:30:00Z"
}
```

### TXT Output:
```
Kata: teknologi
================

Arti:
1. penerapan keilmuan; cara melakukan sesuatu dengan menggunakan alat dan keahlian
2. keseluruhan sarana untuk menyediakan barang-barang yang diperlukan

Contoh:
- teknologi informasi semakin berkembang pesat
- teknologi modern memudahkan kehidupan manusia
```

## ⚙️ Configuration

Edit `config.json` untuk menyesuaikan:
```json
{
  "baseUrl": "https://kbbi.kemdikbud.go.id",
  "timeout": 5000,
  "maxRetries": 3,
  "outputFormat": "json",
  "delay": 1000
}
```

## 🔧 Command Line Options

| Option | Deskripsi | Example |
|--------|-----------|---------|
| `--word` | Single kata untuk dicari | `--word "komputer"` |
| `--words` | Multiple kata (comma separated) | `--words "a,b,c"` |
| `--output` | Output file path | `--output result.json` |
| `--format` | Output format (json/txt) | `--format txt` |
| `--batch` | Batch file input | `--batch words.txt` |

## 🚫 Rate Limiting & Ethics

- ⏱️ Built-in delay antar request (1 detik)
- 🔄 Retry mechanism untuk failed requests
- 📊 Respectful scraping practices
- ⚖️ Mematuhi robots.txt dan terms of service

## ⚠️ Disclaimer

- Tool ini untuk tujuan **educational dan research**
- Pastikan mematuhi terms of service KBBI
- Gunakan secara bertanggung jawab
- Data hasil scraping adalah milik KBBI/Kemdikbud

## 🛠️ Pengembangan

Tool ini dapat dikembangkan lebih lanjut dengan:
- 🌐 Web interface dengan Express.js
- 🔍 Advanced search filters
- 📊 Analytics dan statistics
- 🗄️ Database integration
- 🔄 Real-time API
- 📱 Mobile app integration

## 🤝 Kontribusi

Kontribusi sangat diterima! Lihat [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk panduan.

## 📄 Lisensi

MIT License - Lihat [LICENSE](../../../LICENSE) untuk detail lengkap.

---
**Dibuat oleh**: kanata  
**Kategori**: Scrapers  
**Bahasa**: Node.js
