# 🔍 Port Scanner

Tool port scanner sederhana yang dibuat dengan Go untuk scanning port jaringan dan analisis keamanan.

## ✨ Features
- 🌐 Scan port pada target IP atau hostname
- ⚡ Multi-threading untuk scanning cepat
- 📊 Deteksi port terbuka dan tertutup
- 🕒 Timeout configuration
- 📋 Output hasil scan yang terstruktur

## 📋 Prerequisites
- Go 1.19+ 
- Akses jaringan ke target yang akan discan
- Permission untuk network scanning (beberapa OS memerlukan admin/root)

## 🚀 Instalasi

1. **Clone repository utama**:
   ```bash
   git clone https://github.com/antidonasi/hacktoberfest.git
   cd hacktoberfest/tools/cybersec/port-scanner-by-kachina
   ```

2. **Initialize Go module**:
   ```bash
   go mod init port-scanner
   go mod tidy
   ```

3. **Build executable**:
   ```bash
   go build -o port-scanner index.go
   ```

## 💻 Penggunaan

### Basic Usage:
```bash
# Scan port populer pada localhost
./port-scanner -host localhost

# Scan range port spesifik
./port-scanner -host 192.168.1.1 -start 80 -end 443

# Scan dengan timeout custom
./port-scanner -host example.com -timeout 3s
```

### Contoh Output:
```bash
🔍 Port Scanner by Kachina
==========================
Target: 192.168.1.1
Port Range: 1-1000
Timeout: 2s

Scanning...
[OPEN]  Port 22/tcp   - SSH
[OPEN]  Port 80/tcp   - HTTP  
[OPEN]  Port 443/tcp  - HTTPS
[CLOSED] Port 21/tcp  - FTP
[CLOSED] Port 23/tcp  - Telnet

Scan completed: 3 open ports found
```

## ⚙️ Command Line Options

| Flag | Deskripsi | Default |
|------|-----------|---------|
| `-host` | Target hostname atau IP | localhost |
| `-start` | Port mulai scan | 1 |
| `-end` | Port akhir scan | 1000 |  
| `-timeout` | Timeout per port | 2s |
| `-threads` | Jumlah concurrent threads | 100 |

## ⚠️ Peringatan Keamanan

- **Legal Use Only**: Hanya gunakan pada sistem yang Anda miliki atau dengan izin eksplisit
- **Responsible Disclosure**: Jika menemukan vulnerability, laporkan dengan cara yang bertanggung jawab
- **Rate Limiting**: Beberapa firewall dapat memblokir scanning yang agresif

## 🛠️ Pengembangan

Tool ini dapat dikembangkan lebih lanjut dengan:
- Service detection dan banner grabbing
- Export hasil ke JSON/CSV
- Integration dengan vulnerability databases
- GUI interface
- Stealth scanning techniques

## 🤝 Kontribusi

Kontribusi sangat diterima! Lihat [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk panduan.

## 📄 Lisensi

MIT License - Lihat [LICENSE](../../../LICENSE) untuk detail lengkap.

---
**Dibuat oleh**: kachina  
**Kategori**: Cybersecurity Tools  
**Bahasa**: Go
