# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

common-refused-why-cancelled = 没有提供管理员密码
common-refused-why-wrong = 管理员密码未被接受
common-refused-why-not-allowed = 无论输入什么密码，此登录账户都不被允许这样做
common-refused-why-unasked = 没有任何东西来询问管理员密码
common-refused-why-unclear = 管理员密码未被接受，或者此登录账户不被允许这样做
common-refused-sentence-cancelled = 没有提供管理员密码
common-refused-sentence-wrong = 管理员密码未被接受
common-refused-sentence-not-allowed = 无论输入什么密码，此登录账户都不被允许这样做
common-refused-sentence-unasked = 没有任何东西来询问管理员密码
common-refused-sentence-unclear = 管理员密码未被接受，或者此登录账户不被允许这样做
common-refused-not-done-cancelled = 未执行：没有提供管理员密码。
common-refused-not-done-wrong = 未执行：管理员密码未被接受。
common-refused-not-done-not-allowed = 未执行：无论输入什么密码，此登录账户都不被允许这样做。请注销后重新登录，再试一次。
common-refused-not-done-unasked = 未执行：没有任何东西来询问管理员密码。请注销后重新登录，再试一次。
common-refused-not-done-unclear = 未执行：管理员密码未被接受，或者此登录账户不被允许这样做。如果没有任何提示要求输入密码，请注销后重新登录，再试一次。
common-refused-no-reason = 未执行：该步骤失败了，也没有说明原因。
common-keyring-locked = 密钥环已锁定。请注销后重新登录，或将其解锁，然后再试一次。
common-keyring-missing = 这台电脑上没有可用来保存它的密钥环。
common-keyring-silent = 密钥环没有及时响应。
common-keyring-never = 密钥环始终没有响应。
common-keyring-turn-failed = 无法与密钥环交互（{ $why }），所以没有向它提出任何请求。
common-keyring-waiting =
    { $action ->
        [removing] 密钥环尚未响应，所以{ $what }暂时没有被移除。如果密钥环最终移除了它，这一行会说明。
       *[other] 密钥环尚未响应，所以{ $what }暂时没有被保存。如果密钥环最终保存了它，这一行会说明。
    }
common-keyring-late-done =
    { $action ->
        [removing] 密钥环响应得晚了：{ $what }已被移除。
       *[other] 密钥环响应得晚了：{ $what }已被保存。
    }
common-keyring-late-never =
    { $action ->
        [removing] 密钥环始终没有响应，所以{ $what }没有被移除。
       *[other] 密钥环始终没有响应，所以{ $what }没有被保存。
    }
common-keyring-late-refused =
    { $action ->
        [removing] 密钥环响应得晚了，并且没有移除{ $what }：{ $why }
       *[other] 密钥环响应得晚了，并且没有保存{ $what }：{ $why }
    }
common-more = 更多
common-more-spoken = 了解更多，将打开帮助
folders-volumes-no-password = 没有提供密码，所以没有任何改动。
folders-volumes-nothing-changed = { $reason }，所以没有任何改动。
folders-volumes-busy = 仍有东西在使用它。请关闭所有显示其文件的窗口或程序，然后再试。
folders-volumes-no-room = 按该大小，空间不足以容纳其上的内容。
folders-prompt-password = 密码
folders-prompt-unlock = 解锁
folders-prompt-cancel = 取消
folders-prompt-type-first = 请先输入密码
folders-prompt-checking = 正在检查…
folders-wallpaper-larger-4k = 大于 4K
folders-wallpaper-larger-screen = 大于您的屏幕
folders-wallpaper-over-20-or-200 = 超过 20 MB（动态的超过 200 MB）
folders-wallpaper-over-20 = 超过 20 MB
folders-wallpaper-over-200 = 超过 200 MB
folders-wallpaper-reasons-or = { $first }或{ $second }
