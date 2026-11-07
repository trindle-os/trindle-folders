

folders-far-failed-why = { $name }（{ $why }）

folders-far-no-server = 找不到名为 { $host } 的服务器

folders-far-not-all-arrived = 没有全部送达，所以仍留在原处

folders-far-not-followed = 这是链接，不会跟随，所以没有发送

folders-far-not-sent = 不是文件或文件夹，所以没有发送

folders-far-not-told = 服务器不说明它是什么，所以已跳过

folders-far-ftp-cannot-refuse = 此服务器可能让两个文件使用同一个名称。

folders-far-cannot-do = 无法对服务器上的文件执行此操作

folders-far-not-looked-at = 无法查看

folders-far-not-opened = 无法打开。

folders-far-inside-itself = { $name }（文件夹不能放进它自己里面）

folders-far-taken-by-folder = { $name }（那里已有同名文件夹）

folders-far-taken-by-file = { $name }（那里已有同名文件）

folders-far-address-unknown = 不知道服务器的地址。

folders-far-copy-wrong-length = 发送的副本长度不对

folders-far-only-server-file = 只有服务器上的文件才能这样改名。

folders-far-aside-both = 旧文件完整保存在 { $old }，新文件在 { $new }：{ $why }

folders-far-aside-kept = 无法删除旧文件，它完整保存在 { $old }

folders-far-aside-kept-why = 无法删除旧文件，它完整保存在 { $old }：{ $why }

folders-far-not-empty = { $name } 不是空的。

folders-far-unsafe-name = 已跳过：服务器发来的名称不安全。

folders-far-unsafe-names =
    { $n ->
       *[other] 已跳过 { $n } 项：服务器发来的名称不安全：{ $names }。
    }

folders-far-fetching = 正在获取 { $name }…

folders-far-cannot-fetch = 无法获取 { $name }：{ $why }。

folders-far-opened-copy = 已打开 { $name } 的副本。保存时会把更改送回服务器。

folders-far-saved = 已将 { $name } 保存到服务器。

folders-far-not-saved = 无法将 { $name } 保存到服务器：{ $why }。此处的副本保留了更改。

folders-far-renamed = 已将 { $was } 改名为 { $now }。

folders-far-connect-title = 连接到服务器

folders-far-connect-close = 关闭

folders-far-connect-address = 地址，例如 sftp://jo@example.com

folders-far-connect-password = 密码（如果需要）

folders-far-connect-key = 密钥文件（如果你选择）

folders-far-connect-pick-key = 选择密钥文件

folders-far-connect-choose = 选择

folders-far-connect-ssh-setup = 使用你自己的 ssh 设置：其配置、代理、密钥和已知服务器。

folders-far-connect-new-host = 以前没有访问过 { $host }，所以 Trindle 无法确定它是否正确。如果你确实想连接它，请信任它。

folders-far-connect-host-changed = { $host } 的身份与以前不同。这可能表示有人在中途拦截，所以没有打开。如果是其所有者更改了它，请先从你的已知主机中删除旧的记录。

folders-far-connect-needs-password = 此服务器需要密码。请在上方输入，然后再试一次。

folders-far-connect-needs-key = 此服务器需要密钥或密码。请在上方填写你的密钥文件，或将其添加到你的 ssh 代理，然后再试一次。

folders-far-connect-cert-new = 无法检查 { $host } 的证书：{ $why }。如果可以，请把它的指纹与服务器所有者给出的指纹对比，只有两者一致才信任它。

folders-far-connect-cert-changed = { $host } 的证书与你以前信任的不同。可能是其所有者续期了证书，也可能有人在中途拦截。只有在所有者确认此指纹后，才信任新证书。

folders-far-connect-connecting = 正在连接

folders-far-connect-trust = 信任并连接

folders-far-connect-go = 连接

folders-far-phones = 手机插入并解锁后，会自动显示在侧边栏中。

