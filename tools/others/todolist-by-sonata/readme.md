# ✅ Todo List App

Aplikasi todo list sederhana berbasis console yang dibuat dengan Python untuk manajemen task harian.

## ✨ Features
- ➕ Tambah task baru
- ✅ Mark task sebagai completed
- 🗑️ Hapus task
- 📝 Edit task yang ada
- 👀 Lihat semua task
- 💾 Simpan data secara persisten
- 🔍 Filter task berdasarkan status

## 📋 Prerequisites
- Python 3.x
- Modul `json` (built-in Python)
- Modul `datetime` (built-in Python)

## 🚀 Instalasi

1. **Clone repository utama**:
   ```bash
   git clone https://github.com/antidonasi/hacktoberfest.git
   cd hacktoberfest/tools/others/todolist-by-sonata
   ```

2. **Install dependencies** (jika ada):
   ```bash
   pip install -r requirements.txt
   ```

3. **Jalankan aplikasi**:
   ```bash
   python index.py
   ```

## 💻 Penggunaan

### Menu Utama:
```bash
✅ Todo List App by Sonata
=========================
1. 📝 Lihat semua task
2. ➕ Tambah task baru  
3. ✅ Mark task completed
4. ✏️ Edit task
5. 🗑️ Hapus task
6. 🔍 Filter task
7. 🚪 Keluar

Pilih menu (1-7): 
```

### Contoh Penggunaan:

#### 1. Menambah Task:
```bash
➕ Tambah Task Baru
Masukkan task: Belajar Python
Prioritas (high/medium/low): high
Deadline (YYYY-MM-DD) atau kosong: 2025-10-10

✅ Task berhasil ditambahkan!
```

#### 2. Melihat Task:
```bash
📝 Daftar Task
==============
[1] ⏳ Belajar Python (Priority: HIGH) - Deadline: 2025-10-10
[2] ✅ Review code (COMPLETED) - Added: 2025-10-04
[3] ⏳ Meeting client (Priority: MEDIUM) - Deadline: 2025-10-05
```

## 💾 Data Storage

Data disimpan dalam file `tasks.json` dengan format:
```json
{
  "tasks": [
    {
      "id": 1,
      "title": "Belajar Python", 
      "completed": false,
      "priority": "high",
      "deadline": "2025-10-10",
      "created_at": "2025-10-04T10:30:00"
    }
  ]
}
```

## 🎯 Fitur Tambahan

### Filter Task:
- Lihat hanya completed tasks
- Lihat hanya pending tasks  
- Filter berdasarkan prioritas
- Filter berdasarkan deadline

### Prioritas Task:
- 🔴 **HIGH**: Task urgent
- 🟡 **MEDIUM**: Task normal  
- 🟢 **LOW**: Task tidak urgent

## 🛠️ Pengembangan

Tool ini dapat dikembangkan lebih lanjut dengan:
- 🌐 Web interface dengan Flask/Django
- 📱 Mobile app version
- 🔔 Reminder dan notification
- 👥 Multi-user support
- 📊 Statistics dan analytics
- 🔄 Sync dengan cloud storage

## 🤝 Kontribusi

Silakan berkontribusi untuk meningkatkan aplikasi ini! Lihat [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk panduan.

## 📄 Lisensi

MIT License - Lihat [LICENSE](../../../LICENSE) untuk detail lengkap.

---
**Dibuat oleh**: sonata  
**Kategori**: Others  
**Bahasa**: Python