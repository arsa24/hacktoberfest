// ...existing code...
pub fn get_provider_name(url: &str) -> Result<String, String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;
    let domain = parsed_url.domain().ok_or("No domain found")?;
    let parts: Vec<&str> = domain.split('.').collect();

    // Heuristic: return the second-level domain when available (e.g. "www.pixeldrain.com" -> "pixeldrain")
    let provider = if parts.len() >= 2 {
        parts[parts.len() - 2]
    } else {
        parts[0]
    };

    Ok(provider.to_lowercase())
}
// ...existing code...# 📝 Panduan Kontribusi

Terima kasih sudah tertarik untuk berkontribusi pada **Random Tools Collection**! 🎉 

Repository ini dibuat khusus untuk **Hacktoberfest 2025** dan kami sangat menyambut kontribusi dari developer dengan berbagai tingkat keahlian.

## 📋 Daftar Isi

- [Code of Conduct](#-code-of-conduct)
- [Jenis Kontribusi](#-jenis-kontribusi)
- [Cara Berkontribusi](#-cara-berkontribusi)
- [Standar Coding](#-standar-coding)
- [Template Tool](#-template-tool)
- [Review Process](#-review-process)
- [Tips untuk Kontributor Baru](#-tips-untuk-kontributor-baru)

## 📜 Code of Conduct

Dengan berkontribusi pada proyek ini, kamu setuju untuk mengikuti [Code of Conduct](CODE_OF_CONDUCT.md) kami. Singkatnya:
- Bersikap sopan dan menghormati sesama kontributor
- Memberikan feedback yang konstruktif
- Fokus pada apa yang terbaik untuk komunitas

## 🎯 Jenis Kontribusi

### ✨ Menambah Tool Baru
- **Beginner Friendly**: Calculator, Todo List, Random Quote Generator
- **Intermediate**: URL Shortener, File Converter, API Wrapper
- **Advanced**: System Monitor, Data Analytics Tool, Machine Learning Utility

### 🐛 Memperbaiki Bug
- Cek [Issues](https://github.com/antidonasi/hacktoberfest/issues) dengan label `bug`
- Reproduksi bug dan berikan langkah-langkah yang jelas
- Jelaskan solusi yang kamu implementasikan

### 📚 Dokumentasi
- Memperbaiki typo atau kesalahan bahasa
- Menambah contoh penggunaan
- Membuat tutorial atau panduan
- Menerjemahkan ke bahasa lain

### 🔧 Optimasi & Refactoring
- Memperbaiki performance
- Menambah error handling
- Memperbaiki struktur kode
- Menambah unit tests

## 🚀 Cara Berkontribusi

### 1. Fork & Clone
```bash
# Fork repository di GitHub, lalu clone
git clone https://github.com/antidonasi/hacktoberfest.git
cd hacktoberfest

# Tambah remote upstream
git remote add upstream https://github.com/antidonasi/hacktoberfest.git
```

### 2. Buat Branch Baru
```bash
# Update main branch
git checkout main
git pull upstream main

# Buat branch baru
git checkout -b feature/nama-tool-kamu
# atau
git checkout -b fix/deskripsi-bug
```

### 3. Buat Tool atau Perbaikan

#### Untuk Tool Baru:
1. Buat folder di kategori yang sesuai: `tools/kategori/nama-tool/`
2. Ikuti [Template Tool](#-template-tool) di bawah
3. Pastikan tool berfungsi dengan baik
4. Tambah dokumentasi yang jelas

#### Untuk Bug Fix:
1. Identifikasi root cause masalah
2. Buat fix yang minimal dan tepat sasaran
3. Test fix kamu thoroughly
4. Update dokumentasi jika diperlukan

### 4. Commit & Push
```bash
# Add dan commit perubahan
git add .
git commit -m "feat: add password generator tool"
# atau
git commit -m "fix: resolve CSV parsing issue in data converter"

# Push ke branch kamu
git push origin nama-branch-kamu
```

### 5. Buat Pull Request
1. Pergi ke repository asli di GitHub
2. Klik "New Pull Request"
3. Pilih branch kamu dari fork
4. Isi title dan deskripsi yang jelas
5. Centang checklist yang ada
6. Submit PR

## 📏 Standar Coding

### General Guidelines
- **Bahasa kode**: Boleh bahasa apa saja, tapi komentar dalam bahasa Inggris atau Indonesia
- **Naming**: Gunakan nama yang deskriptif (`passwordGenerator` bukan `pg`)
- **Comments**: Jelaskan bagian kode yang kompleks
- **Error Handling**: Selalu handle error dengan baik

### Struktur File Tool
```
tools/kategori/nama-tool/
├── README.md           # Dokumentasi tool
├── src/               # Source code utama
├── examples/          # Contoh penggunaan
├── tests/             # Unit tests (opsional)
└── requirements/      # Dependencies (package.json, requirements.txt, dll)
```

### Template README.md untuk Tool
```markdown
# Nama Tool

Deskripsi singkat tool ini.

## Features
- Feature 1
- Feature 2

## Prerequisites
- Bahasa/runtime yang dibutuhkan
- Dependencies lainnya

## Installation
```bash
# Langkah instalasi
```

## Usage
```language
# Use case/contoh penggunaan
```

## Examples
[Contoh penggunaan lengkap]

## Contributing
Lihat [CONTRIBUTING.md](../../CONTRIBUTING.md)

## License
MIT License
```

## 🔍 Review Process

### Kriteria Pull Request yang Baik:
- ✅ **Functionality**: Tool berfungsi sesuai deskripsi
- ✅ **Documentation**: README jelas dan lengkap  
- ✅ **Code Quality**: Kode clean dan readable
- ✅ **Testing**: Tool sudah ditest dan tidak error
- ✅ **Originality**: Tool tidak duplikat dengan yang sudah ada

### Timeline Review:
- **Initial Response**: 1-2 hari
- **Full Review**: 3-5 hari
- **Merge**: Setelah semua feedback diaddress

### Jika PR Ditolak:
- Jangan berkecil hati! 💪
- Baca feedback dengan seksama
- Perbaiki sesuai saran
- Push update ke branch yang sama
- PR akan otomatis terupdate

## 💡 Tips untuk Kontributor Baru

### Pilih Kontribusi Pertama yang Mudah:
- 🟢 **Easy**: Perbaiki typo, tambah contoh, buat simple calculator
- 🟡 **Medium**: File converter, API client, automation script
- 🔴 **Hard**: Complex algorithm, system integration, advanced analytics

### Best Practices:
1. **Start Small**: Mulai dari tool sederhana dulu
2. **Read Issues**: Cek issues dengan label `good first issue` atau `hacktoberfest`
3. **Ask Questions**: Jangan ragu bertanya di Issues atau Discussions
4. **Test Thoroughly**: Pastikan tool kamu benar-benar berfungsi
5. **Follow Updates**: Watch repository untuk notifikasi

### Ide Tool yang Dibutuhkan:
- 🔧 **System**: File organizer, log cleaner, backup utility
- 🌐 **Web**: URL validator, HTML minifier, CSS optimizer
- 📊 **Data**: JSON formatter, data validator, report generator
- 🎨 **Creative**: ASCII art, color picker, random generator
- 📱 **Utility**: QR code, barcode, hash generator

## ❓ Butuh Bantuan?

- 💬 **GitHub Discussions**: Untuk pertanyaan umum
- 🐛 **GitHub Issues**: Untuk melaporkan bug atau request feature
- 📧 **Email**: [maintainer-email@example.com]
- 💭 **Discord**: [Link Discord Community]

## 🏆 Recognition

Kontributor akan:
- 📝 Namanya dicantumkan di README utama
- 🏆 Badge khusus untuk kontributor aktif
- 🎁 Mention di social media project
- ⭐ Recommendation letter untuk kontributor outstanding

---

**Selamat berkontribusi dan Happy Hacktoberfest! 🎃**

*Last updated: September 2025*