# 🌐 Mini Manager

Aplikasi web sederhana untuk manajemen data berbasis PHP dengan interface yang user-friendly.

## ✨ Features
- 👥 User management (CRUD)
- 📊 Dashboard dengan statistics
- 🔐 Login system yang aman
- 📝 Data management
- 📱 Responsive design
- 💾 Database integration dengan MySQL
- 🔍 Search dan filter data
- 📄 Export data ke Excel/PDF

## 📋 Prerequisites
- PHP 7.4+ atau 8.x
- MySQL 5.7+ atau MariaDB
- Apache/Nginx web server
- Composer (untuk dependency management)

## 🚀 Instalasi

### 1. Clone Repository
```bash
git clone https://github.com/antidonasi/hacktoberfest.git
cd hacktoberfest/tools/web/mini-manager-by-kira
```

### 2. Setup Web Server

#### Menggunakan XAMPP/WAMP:
1. Copy folder ke `htdocs/mini-manager`
2. Start Apache dan MySQL
3. Akses via `http://localhost/mini-manager`

#### Menggunakan PHP Built-in Server:
```bash
php -S localhost:8000
```

### 3. Setup Database
1. **Buat database**:
   ```sql
   CREATE DATABASE mini_manager;
   ```

2. **Import schema**:
   ```bash
   mysql -u root -p mini_manager < database/schema.sql
   ```

3. **Konfigurasi koneksi** di `koneksi.php`:
   ```php
   <?php
   $host = "localhost";
   $username = "root"; 
   $password = "";
   $database = "mini_manager";
   
   $conn = new mysqli($host, $username, $password, $database);
   ?>
   ```

## 💻 Penggunaan

### 1. Akses Login
```
URL: http://localhost:8000/
Default Login:
- Username: admin
- Password: admin123
```

### 2. Dashboard
- 📊 Overview statistik
- 📈 Grafik data
- 🔔 Notifikasi sistem
- 📋 Quick actions

### 3. User Management
```php
// Contoh endpoint API
GET  /api/users        - List all users
POST /api/users        - Create new user  
PUT  /api/users/{id}   - Update user
DELETE /api/users/{id} - Delete user
```

## 🗂️ Struktur File

```
mini-manager-by-kira/
├── index.php           # Main entry point
├── koneksi.php         # Database connection
├── README.md           # Documentation
├── assets/
│   ├── css/           # Stylesheets
│   ├── js/            # JavaScript files
│   └── images/        # Images dan icons
├── includes/
│   ├── header.php     # Common header
│   ├── footer.php     # Common footer
│   └── functions.php  # Utility functions
├── pages/
│   ├── dashboard.php  # Dashboard page
│   ├── users.php      # User management
│   └── settings.php   # Settings page
└── database/
    └── schema.sql     # Database schema
```

## 🎨 UI Components

### Dashboard Cards:
```html
<div class="card">
    <div class="card-header">
        <h3>📊 Total Users</h3>
    </div>
    <div class="card-body">
        <h2 class="counter">1,234</h2>
        <p class="text-success">↗️ +12% dari bulan lalu</p>
    </div>
</div>
```

### Data Table:
```html
<table class="data-table">
    <thead>
        <tr>
            <th>ID</th>
            <th>Nama</th>
            <th>Email</th>
            <th>Actions</th>
        </tr>
    </thead>
    <tbody id="user-data">
        <!-- Data loaded via AJAX -->
    </tbody>
</table>
```

## 🔐 Security Features

- ✅ SQL Injection prevention
- ✅ XSS protection 
- ✅ CSRF token validation
- ✅ Password hashing
- ✅ Session management
- ✅ Input validation

## 📱 Responsive Design

- 📱 Mobile-first approach
- 💻 Desktop optimization
- 🎨 Modern CSS Grid/Flexbox
- 🌙 Dark/Light theme toggle

## 🚀 Performance

- ⚡ Lazy loading untuk images
- 🗜️ CSS/JS minification
- 📦 Gzip compression
- 🔄 AJAX untuk dynamic content
- 💾 Browser caching

## 🔧 Configuration

Edit `config.php`:
```php
<?php
define('APP_NAME', 'Mini Manager');
define('APP_VERSION', '1.0.0');
define('DEBUG_MODE', false);
define('ITEMS_PER_PAGE', 10);
?>
```

## 🛠️ Pengembangan

Tool ini dapat dikembangkan lebih lanjut dengan:
- 🔗 REST API endpoints
- 📧 Email notifications
- 📊 Advanced reporting
- 🔄 Real-time updates
- 🌐 Multi-language support
- 📱 Mobile app companion

## 🤝 Kontribusi

Kontribusi sangat diterima! Lihat [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk panduan.

## 📄 Lisensi

MIT License - Lihat [LICENSE](../../../LICENSE) untuk detail lengkap.

---
**Dibuat oleh**: kira  
**Kategori**: Web Tools  
**Bahasa**: PHP
