# 📝 IP Logger GUI (Rust)

A lightweight, cross-platform graphical IP logger built in Rust using `egui` and `eframe`.

این ابزار یک لاگر گرافیکی IP بسیار سبک و قابل‌اجرا روی ویندوز و لینوکس است که با زبان Rust و رابط کاربری `egui` ساخته شده است.

This tool monitors your public IP address, detects IP changes, and lists the active connected IPs on your system.
Designed with a simple, terminal-like GUI and ideal for network diagnostics, development labs, or red team monitoring.

ابزار، آدرس IP عمومی شما را بررسی کرده، تغییرات آن را تشخیص می‌دهد و همچنین لیستی از IPهای متصل به سیستم را نمایش می‌دهد. طراحی آن ساده، شبیه محیط ترمینال و مناسب برای آزمایشگاه‌های توسعه، مانیتورینگ امنیتی یا تیم‌های رد تیم است.

---

## ✨ Features | ویژگی‌ها

- 🟢 Real-time **public IP** detection using `ipify` | تشخیص لحظه‌ای **IP عمومی**
- 🔁 Detects and logs **IP changes** | شناسایی و ثبت تغییرات IP با زمان دقیق
- 📡 Lists **connected IPs** using `netstat` | نمایش **IPهای متصل** از طریق `netstat`
- 🖥️ Minimal, modern GUI with [`egui`](https://github.com/emilk/egui`) | رابط گرافیکی ساده و مدرن
- 💾 **Save logs** to file | قابلیت **ذخیره لاگ‌ها**
- 🔄 **Clear logs** anytime | دکمه برای **پاک‌سازی لاگ‌ها**
- ⚙️ Cross-platform: **Windows & Linux** | قابل اجرا روی ویندوز و لینوکس

---

### 🏗️ Build release binary | ساخت نسخه نهایی

```bash
cargo build --release
```

---

## 🖼️ Screenshot | تصویر محیط برنامه

> 

---

## 📄 License | لایسنس

This project is licensed under the [MIT License](LICENSE).  
این پروژه تحت لایسنس MIT منتشر شده است.

