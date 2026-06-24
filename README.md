# IBus Buffalo

**IBus Buffalo** là một bộ gõ tiếng Việt hiện đại, gọn nhẹ và hiệu năng cao dành cho hệ thống nhập liệu IBus trên hệ điều hành Linux. Dự án được phát triển hoàn toàn bằng **Rust** cho phần nhân dịch và **GTK4 / Relm4** cho giao diện cấu hình, mang lại trải nghiệm nhập liệu mượt mà, an toàn và giao diện trực quan, tinh gọn.

---

## Tính năng nổi bật

- **Hiệu năng vượt trội**: Phần nhân xử lý bộ gõ được viết bằng ngôn ngữ Rust, đảm bảo tốc độ phản hồi cực nhanh, an toàn bộ nhớ và tiêu thụ tài nguyên hệ thống ở mức tối thiểu.
- **Giao diện cấu hình hiện đại**: Ứng dụng cấu hình (`buffalo-ui`) được xây dựng bằng Relm4 và GTK4, mang phong cách thiết kế phẳng, trực quan và dễ sử dụng.
- **Hỗ trợ các kiểu gõ phổ biến**: Hỗ trợ kiểu gõ **Telex** và **VNI**.
- **Tương thích nhiều bảng mã**: Hỗ trợ gõ các bảng mã tiếng Việt thông dụng bao gồm **Unicode**, **TCVN3 (ABC)**, **VNI Windows**, và **VIQR**.
- **Tùy chọn cấu hình nâng cao hiển thị trực quan**:
  - _Tự động sửa lỗi chính tả_: Giúp sửa các lỗi gõ sai từ trong tiếng Việt một cách thông minh.
  - _Đặt dấu tự do (free tone marking)_: Cho phép gõ phím dấu ở bất kỳ vị trí nào trong từ.
  - _Đặt dấu kiểu mới_: Tự động đặt dấu chuẩn xác theo phong cách hiện đại (ví dụ: `hòa`, `khỏe` thay vì kiểu cũ `hoà`, `khoẻ`).

---

## Yêu cầu hệ thống (Prerequisites)

Để biên dịch bộ gõ từ mã nguồn, hệ thống của bạn cần cài đặt các thư viện phát triển sau (tên gói áp dụng cho Ubuntu/Debian):

```bash
sudo apt update
sudo apt install build-essential rustc cargo libibus-1.0-dev libgtk-4-dev libx11-dev libxtst-dev
```

---

## Hướng dẫn biên dịch và cài đặt từ mã nguồn

### 1. Biên dịch dự án

Biên dịch các thành phần bộ gõ ở chế độ tối ưu phát hành (release profile):

```bash
make build
```

### 2. Cài đặt vào hệ thống

Cài đặt các tệp tin thực thi, tệp tin cấu hình IBus Engine, biểu tượng và trình cấu hình desktop:

```bash
sudo make install
```

### 3. Áp dụng các thay đổi

Khởi động lại tiến trình IBus để hệ thống nhận diện bộ gõ mới:

```bash
ibus restart
```

### 4. Thêm bộ gõ vào hệ thống

1. Mở cài đặt hệ thống (Settings) -> **Keyboard** (hoặc **Region & Language**).
2. Tại mục **Input Sources**, nhấn nút **+** để thêm nguồn nhập liệu mới.
3. Chọn ngôn ngữ **Vietnamese**, sau đó tìm kiếm và thêm bộ gõ **Buffalo** hoặc **Buffalo (US layout)**.

---

## Hướng dẫn đóng gói và cài đặt bằng tệp `.deb` (cho Debian/Ubuntu)

Dự án tích hợp sẵn một kịch bản giúp bạn dễ dàng đóng gói toàn bộ ứng dụng thành tệp cài đặt `.deb` tiêu chuẩn cho kiến trúc `amd64`.

### 1. Tạo tệp cài đặt `.deb`

Chạy kịch bản đóng gói tự động trong thư mục gốc của dự án:

```bash
./package-deb.sh
```

Tệp tin cài đặt thành phẩm sẽ được tạo ra tại:
`target/debian/ibus-buffalo_0.1.0_amd64.deb`

### 2. Cài đặt gói `.deb`

Cài đặt gói dịch vụ trực tiếp vào hệ thống của bạn hoặc phân phối cho máy tính khác:

```bash
sudo dpkg -i target/debian/ibus-buffalo_0.1.0_amd64.deb
```

_(Nếu hệ thống báo thiếu thư viện phụ thuộc, hãy chạy lệnh `sudo apt-get install -f` để tự động sửa chữa)._

### 3. Khởi động lại IBus

```bash
ibus restart
```

---

## Hướng dẫn cấu hình bộ gõ

Để mở bảng điều khiển cấu hình trực quan của IBus Buffalo, bạn có thể thực hiện theo một trong các cách sau:

- Tìm kiếm ứng dụng **IBus Buffalo Setup** trong Menu ứng dụng (Application Launcher) của hệ điều hành.
- Chạy trực tiếp lệnh cấu hình từ Terminal:

  ```bash
  /usr/lib/ibus-buffalo/buffalo-ui
  ```

---

## Giấy phép (License)

Dự án này được phát hành dưới các điều khoản của giấy phép **GNU General Public License v3.0 (GPLv3)**. Xem tệp [LICENSE](LICENSE) để biết thêm chi tiết.
