

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = không tìm thấy máy chủ nào có tên { $host }

folders-far-not-all-arrived = chưa gửi đủ nên vẫn để nguyên chỗ cũ

folders-far-not-followed = là liên kết, không được theo dõi nên không gửi

folders-far-not-sent = không phải tệp hay thư mục nên không gửi

folders-far-not-told = máy chủ không cho biết đó là gì nên đã bỏ qua

folders-far-ftp-cannot-refuse = Máy chủ này có thể cho hai tệp cùng một tên.

folders-far-cannot-do = Không thể làm việc đó với tệp trên máy chủ

folders-far-not-looked-at = không thể xem được

folders-far-not-opened = Không thể mở được.

folders-far-inside-itself = { $name } (thư mục không thể nằm trong chính nó)

folders-far-taken-by-folder = { $name } (đã có thư mục cùng tên)

folders-far-taken-by-file = { $name } (đã có tệp cùng tên)

folders-far-address-unknown = Không biết địa chỉ của máy chủ.

folders-far-copy-wrong-length = bản đã gửi không đúng độ dài cần có

folders-far-only-server-file = Chỉ tệp trên máy chủ mới được đổi tên kiểu này.

folders-far-aside-both = bản cũ vẫn còn nguyên tại { $old }, bản mới tại { $new }: { $why }

folders-far-aside-kept = không thể xóa bản cũ, bản cũ vẫn còn nguyên tại { $old }

folders-far-aside-kept-why = không thể xóa bản cũ, bản cũ vẫn còn nguyên tại { $old }: { $why }

folders-far-not-empty = { $name } không trống.

folders-far-unsafe-name = Đã bỏ qua: máy chủ gửi một tên không an toàn.

folders-far-unsafe-names =
    { $n ->
        [1] Đã bỏ qua: máy chủ gửi một tên không an toàn: { $names }.
       *[other] Đã bỏ qua { $n } mục: máy chủ gửi tên không an toàn: { $names }.
    }

folders-far-fetching = Đang tải { $name }…

folders-far-cannot-fetch = Không thể tải { $name }: { $why }.

folders-far-opened-copy = Đã mở một bản sao của { $name }. Khi lưu, các thay đổi sẽ được gửi lại.

folders-far-saved = Đã lưu { $name } trên máy chủ.

folders-far-not-saved = Không thể lưu { $name } trên máy chủ: { $why }. Bản sao ở đây vẫn giữ các thay đổi.

folders-far-renamed = Đã đổi tên { $was } thành { $now }.

folders-far-connect-title = Kết nối tới máy chủ

folders-far-connect-close = Đóng

folders-far-connect-address = Địa chỉ, ví dụ sftp://jo@example.com

folders-far-connect-password = Mật khẩu, nếu cần

folders-far-connect-key = Tệp khóa, nếu bạn chọn

folders-far-connect-pick-key = Chọn tệp khóa

folders-far-connect-choose = Chọn

folders-far-connect-ssh-setup = Dùng thiết lập ssh của chính bạn: cấu hình, agent, khóa và các máy chủ đã biết.

folders-far-connect-new-host = Chưa từng truy cập { $host } trước đây nên Trindle không thể biết đó có đúng máy chủ không. Chỉ tin cậy nếu bạn đang định kết nối tới nó.

folders-far-connect-host-changed = { $host } không còn giữ danh tính như trước. Điều này có thể có nghĩa là có người đang chen vào giữa, nên sẽ không mở. Nếu chủ sở hữu đã thay đổi, hãy xóa mục cũ khỏi danh sách máy chủ đã biết trước.

folders-far-connect-needs-password = Máy chủ này cần mật khẩu. Hãy nhập mật khẩu ở trên rồi thử lại.

folders-far-connect-needs-key = Máy chủ này cần khóa hoặc mật khẩu. Hãy nêu tên tệp khóa ở trên hoặc thêm khóa vào ssh agent, rồi thử lại.

folders-far-connect-cert-new = Không thể kiểm tra chứng chỉ của { $host }: { $why }. Nếu có thể, hãy so sánh dấu vân tay của nó với dấu vân tay do chủ máy chủ cung cấp, và chỉ tin cậy khi hai bên khớp nhau.

folders-far-connect-cert-changed = Chứng chỉ của { $host } không phải chứng chỉ bạn đã tin cậy trước đây. Chủ sở hữu có thể đã gia hạn, hoặc có người đang chen vào giữa. Chỉ tin cậy chứng chỉ mới nếu chủ sở hữu xác nhận dấu vân tay này.

folders-far-connect-connecting = Đang kết nối

folders-far-connect-trust = Tin cậy và kết nối

folders-far-connect-go = Kết nối

folders-far-phones = Điện thoại sẽ tự hiện ở thanh bên khi được cắm vào và mở khóa.
