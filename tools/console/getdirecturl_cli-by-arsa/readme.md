<h1 align="center">GetDirectUrl</h1>
Sebuah tools cli yang memungkinkan untuk mendapatkan direct url dari website cloud storage.

## Fitur
| Fitur | Status |
|-------|--------|
|Mendapatkan direct url| Berfungfi |
|Download file | WIP |

## Provider
Beberapa provider dari cloud storage yang didukung:
- Pixeldrain
- Mega Nz
- Kraken file

## Persyaratan

Sebelum membangun proyek ini, pastikan sudah menginstal:

- [Rust](https://www.rust-lang.org/tools/install) (termasuk `cargo`)
- Koneksi internet untuk mengunduh dependensi crate saat build

### Cek instalasi
Pastikan Rust dan Cargo sudah terpasang dengan benar:
```sh
rustc --version
```
```sh
cargo --version
```

## Installasi

### 1. Clone repository
```sh
git clone https://github.com/AntiDonasi/hacktoberfest.git
```
### 2. Masuk ke repository
```sh
cd hacktoberfest/tools/console/getdirecturl_cli-by-arsa
```
### 3. Build
```sh
cargo build --release
```

### 4. Jalankan langsung
#### 1. Linux
```sh
./target/release/getdirecturl --help
```
#### 2. Windows
```sh
target/release/getdirecturl.exe --help
```
#### 3. Macos
```sh
./target/release/getdirecturl --help
```

### 5. Pindahkan binary ke system
Agar cli bisa dijalankan langsung maka kita perlu membuat variable atau menambahkannya ke direktori system
#### 1. Linux
```sh
sudo cp target/release/getdirecturl /usr/local/bin
```
#### 2. Windows
m-m-maaf ya kalau yang ini kurang tahu >///<
#### 3. Macos
 ```sh
sudo cp target/release/getdirecturl /usr/local/bin
```