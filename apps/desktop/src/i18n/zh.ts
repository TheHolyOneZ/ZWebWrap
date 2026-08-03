import type { Dictionary } from "./en";

export const zh: Dictionary = {
  "app.tagline": "把网页变成桌面应用。这不是浏览器。",
  "app.settings": "设置",
  "app.blocking.ready": "{count} 条规则",
  "app.blocking.loading": "正在加载过滤规则…",
  "app.blocking.status": "广告拦截状态",
  "app.search": "搜索你的应用",
  "app.empty.title": "还没有应用",
  "app.empty.body":
    "在上方粘贴一个网址，ZWebWrap 会把它变成一个有独立窗口和图标的桌面应用。",
  "app.noMatch": "没有匹配“{query}”的结果",
  "app.clearSearch": "清除搜索",
  "app.library": "你的应用",

  "media.title": "此系统上无法播放视频",
  "media.body":
    "GStreamer 缺少 {missing}。每个网站都会显示播放器，但什么也加载不出来——这影响所有基于 WebKit 的应用，不只是 ZWebWrap。",
  "media.installHint": "安装缺少的插件后再检查一次：",
  "media.recheck": "再检查一次",
  "media.found": "已找到媒体插件 — 重新打开应用后视频即可播放",
  "media.stillMissing": "仍然缺少：{missing}",

  "add.placeholder": "粘贴网址 — youtube.com、claude.ai…",
  "add.submit": "创建应用",
  "add.working": "正在创建…",
  "add.busy": "正在获取图标并创建快捷方式…",
  "add.preview.suffix": "会在应用菜单中添加快捷方式",
  "add.hint":
    "你会得到一个快捷方式，在独立窗口中打开这个网站。没有标签页，没有地址栏。",

  "card.open": "打开 {name}",
  "card.settings": "{name} 的设置",
  "card.blocking": "拦截中",
  "card.blockingOff": "已关闭",
  "card.blockingOn.title": "此应用已启用广告拦截",
  "card.blockingOff.title": "此应用已关闭广告拦截",

  "detail.general": "常规",
  "detail.behaviour": "行为",
  "detail.appearance": "外观",
  "detail.advanced": "高级",
  "detail.sections": "应用设置分区",

  "detail.name": "名称",
  "detail.name.hint": "显示在快捷方式和窗口标题上。",
  "detail.address": "网址",
  "detail.icon.change": "更换",
  "detail.icon.saving": "正在保存…",
  "detail.icon.reset": "重置",
  "detail.icon.reset.title": "重新获取网站自己的图标",

  "detail.adblock": "拦截广告和追踪器",
  "detail.adblock.full": "完全",
  "detail.adblock.networkOnly": "仅网络",
  "detail.adblock.off": "关闭",
  "detail.adblock.hint.networkOnly":
    "请求仍会被过滤，但不会向页面注入任何内容。当隐藏广告位会破坏网站时使用。",
  "detail.adblock.hint.off": "此应用不过滤也不隐藏任何内容。",
  "detail.adblock.hint": "在请求发出前过滤，并隐藏残留的广告位。",
  "detail.shortcut": "桌面快捷方式",
  "detail.shortcut.checking": "检查中…",
  "detail.shortcut.hint":
    "这个应用始终在你的应用菜单里。这里指的是桌面上的那份副本。",
  "detail.fullscreen": "全屏充满此窗口",
  "detail.fullscreen.hint":
    "点击视频全屏时充满这个窗口，而不是占据整个屏幕。想要普通全屏就关掉它。",
  "detail.frame": "窗口边框",
  "detail.frame.hint":
    "“无边框”请求桌面去掉用于调整大小的边缘——很多主题本来就没有，所以可能看起来一样。“无标题栏”一定看得出区别：移动、最大化和关闭改由悬浮工具栏承担。",
  "detail.frame.normal": "正常",
  "detail.frame.noBorder": "无边框",
  "detail.frame.frameless": "无标题栏",

  "detail.offsite": "当链接离开 {host} 时",
  "detail.offsite.aria": "当链接离开本站时",
  "detail.offsite.hint":
    "这个窗口锁定在一个网站上。指向别处的链接要么在你平时的浏览器中打开，要么在这里继续。无论哪种，登录都留在这里。",
  "detail.offsite.browser": "交给我的浏览器",
  "detail.offsite.inWindow": "在这里打开",
  "detail.scope": "也属于这个应用",
  "detail.scope.hint": "应在窗口内而不是浏览器中打开的其他域名。用逗号分隔。",

  "detail.proxy": "代理",
  "detail.proxy.hint":
    "让这个应用的流量走代理——host:端口，或 socks5://host:端口。设置代理后该应用会切换到独立会话，因为两种 webview 引擎都把代理挂在会话上，而不是窗口上。",
  "detail.auth": "网站登录",
  "detail.auth.username": "用户名",
  "detail.auth.password": "密码",
  "detail.auth.hint":
    "用于内网仪表板那类带 HTTP 密码框的站点。只在第一次询问时应答；若被拒绝，就显示站点自己的密码框，而不是把账号锁掉。保存在数据文件夹的 credentials.json 中，以你的身份运行的任何程序都能读取。",
  "detail.zoom": "缩放",
  "detail.window": "窗口",
  "detail.window.hint": "关闭窗口时记住。",
  "detail.userAgent": "浏览器身份",
  "detail.userAgent.placeholder": "默认 — WebKit 自己的 User-Agent",
  "detail.userAgent.hint":
    "这个应用向网站自称的身份。默认值最诚实，也是机器人检测所预期的；少数网站会拒绝它并要求 Chrome。",

  "detail.delete": "删除",
  "detail.delete.confirm": "删除这个应用及其快捷方式？",
  "detail.cancel": "取消",
  "detail.save": "保存",
  "detail.saving": "正在保存…",

  "settings.title": "设置",
  "settings.subtitle": "偏好、广告拦截与存储",
  "settings.sections": "设置分区",
  "settings.general": "常规",
  "settings.blocking": "广告拦截",
  "settings.storage": "存储",
  "settings.autosaved": "更改会自动保存",
  "settings.done": "完成",

  "settings.startClose": "启动与关闭",
  "settings.autostart": "随系统启动",
  "settings.autostart.hint": "登录时静静地出现在托盘里——不打开窗口。",
  "settings.tray": "关闭时收进托盘",
  "settings.tray.hint":
    "关闭应用只是把它隐藏而不是退出，所以能立刻回来，并从原处继续播放。要退出请用托盘菜单。",
  "settings.tray.note":
    "只要其中一项开启，ZWebWrap 就会在托盘中保留一个图标。两项都关闭时不会留下任何进程。",

  "settings.windows": "窗口",
  "settings.fullscreen": "全屏充满窗口",
  "settings.fullscreen.hint":
    "点击视频全屏时充满窗口，而不是占据整个屏幕。每个应用都可以在自己的设置里另行决定。",

  "settings.newApps": "新应用",
  "settings.desktopShortcut": "把新快捷方式放到桌面",
  "settings.desktopShortcut.hint":
    "它们总会出现在应用菜单里。这项会另外在桌面放一份副本。每个应用都可以另行决定。",
  "settings.adblockDefault": "在新应用中拦截广告",
  "settings.adblockDefault.hint": "已经创建的应用保留各自的设置。",

  "settings.language": "语言",
  "settings.language.hint": "除非在这里选择，否则跟随系统。",
  "settings.language.system": "跟随系统",
  "settings.theme": "主题",
  "settings.theme.hint": "除非在这里选择，否则跟随系统。",
  "settings.theme.light": "浅色",
  "settings.theme.dark": "深色",

  "settings.metric.rules": "网络规则",
  "settings.metric.lists": "过滤列表",
  "settings.metric.updated": "上次更新",
  "settings.notReady": "过滤引擎仍在加载。此刻打开的窗口不会拦截广告。",
  "settings.seedOnly":
    "目前只有内置的小规则表在生效——完整的过滤列表尚未下载。它能拦截请求，但无法隐藏残留的广告位。",
  "settings.lists": "过滤列表",
  "settings.lists.hint": "首次运行时下载，之后每 4 天自动刷新。",
  "settings.update": "立即更新",
  "settings.updating": "正在更新…",
  "settings.updated": "过滤规则已更新 — {count} 条规则",
  "settings.blocked": "拦截了什么",
  "settings.blocked.body":
    "请求在发出前就被过滤。残留的广告位会用站点专用规则和通用规则一起隐藏，YouTube 的视频广告则直接从播放器的响应里剔除。",
  "settings.dataFolder": "数据文件夹",
  "settings.dataFolder.hint":
    "应用、图标、过滤列表，以及所有应用共享的登录会话。",
  "settings.open": "打开",

  "app.blocking.preparing": "正在准备过滤规则…",
  "app.blocking.basic": "基础拦截",
  "menu.open": "打开",
  "menu.settings": "设置…",
  "menu.copyAddress": "复制网址",
  "menu.adblockOff": "关闭广告拦截",
  "menu.adblockOn": "开启广告拦截",
  "menu.delete": "删除…",
  "toast.created": "已创建 {name}",
  "toast.saved": "已保存 {name}",
  "toast.deleted": "已删除 {name}",
  "toast.copied": "网址已复制",
  "toast.copyFailed": "无法复制网址",
  "confirm.delete.title": "删除 {name}？",
  "confirm.delete.body":
    "它的桌面快捷方式和图标也会一并删除。网站本身不受影响，你随时可以再添加回来。",
  "ui.dismiss": "关闭",
  "ui.close": "关闭",
  "ui.cancel": "取消",

  "detail.permissions": "权限",
  "detail.perm.notifications": "通知",
  "detail.perm.microphone": "麦克风",
  "detail.perm.camera": "摄像头",
  "detail.perm.location": "位置",
  "detail.perm.hint":
    "这个应用可以向系统请求什么。“询问”表示只问你一次，然后记住答案。",
  "rule.allow": "允许",
  "rule.deny": "拦截",
  "rule.ask": "询问",

  "detail.badge": "在标题中显示未读数",
  "detail.badge.hint":
    "把网页写在自己标题里的数字——(3) Discord、收件箱 (12)——映射到这个窗口，好让任务栏显示出来。",
  "detail.userCss": "你的 CSS",
  "detail.userCss.hint":
    "在页面绘制之前注入。适合用来隐藏网站不让你隐藏的东西。",
  "detail.userJs": "你的 JavaScript",
  "detail.userJs.hint":
    "在文档开始时以页面自身的权限运行。它接触不到 ZWebWrap 本身。",

  "detail.hotkey": "全局快捷键",
  "detail.hotkey.hint":
    "在系统任何地方按下这些键，就能把这个应用调到前面。它已在前面时再按一次就收起来。点击输入框并按下组合键。",
  "detail.hotkey.record": "点击并按键",
  "detail.hotkey.clear": "清除",
  "detail.hotkey.taken": "该组合不被接受。请换一个。",
  "import.title": "从浏览器导入",
  "import.subtitle": "把你已有的书签变成桌面应用",
  "import.reading": "正在读取你的浏览器…",
  "import.none": "没有找到尚未成为应用的书签。",
  "import.search": "筛选书签",
  "import.pick": "选择你想要的",
  "import.chosen": "已选 {n} 个",
  "import.submit": "导入 {n} 个",
  "import.working": "正在导入…",
  "import.done": "已导入 {n} 个应用",
  "app.import": "从浏览器导入",
  "settings.platform": "本系统",
  "settings.platform.fine": "ZWebWrap 的所有功能在本系统上都可用。",
  "settings.platform.session": "会话",
  "time.never": "从未",
  "time.justNow": "刚刚",
  "time.minutes": "{n} 分钟前",
  "time.hours": "{n} 小时前",
  "time.days": "{n} 天前",
  "dl.title": "下载",
  "dl.empty": "还没有下载任何内容。",
  "dl.clear": "清除",
  "dl.open": "打开",
  "dl.reveal": "在文件夹中显示",
  "dl.failed": "失败",
  "dl.running": "已下载 {size}",
  "detail.session": "登录会话",
  "detail.session.shared": "共享",
  "detail.session.isolated": "独立",
  "detail.session.hint":
    "与你的其他应用共用一次登录：登录一次 Google 即可用于 YouTube、Gmail 和云端硬盘。",
  "detail.session.hint.isolated":
    "此应用单独登录，因此可以使用第二个账号。启动时为未登录状态。下次打开时生效。",
  "settings.downloads": "下载",
  "settings.askWhereToSave": "每个文件都询问保存位置",
  "settings.askWhereToSave.hint":
    "应用窗口没有下载栏，静默保存的文件很容易被忽略。关闭后所有文件将直接保存到下载文件夹。",
  "dl.toast.done": "已下载 {name}",
  "dl.toast.failed": "无法下载 {name}",
  "settings.metric.hidden": "已隐藏广告",
  "settings.tour": "向导",
  "settings.tour.hint": "全新安装时显示一次。你可以随时再看一遍。",
  "settings.tour.button": "查看向导",
  "tour.title": "快速向导",
  "tour.step": "第 {n} 步，共 {total} 步",
  "tour.skip": "跳过向导",
  "tour.back": "上一步",
  "tour.next": "下一步",
  "tour.done": "完成",
  "tour.welcome.title": "欢迎使用 ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap 把网站变成桌面应用：独立的窗口、任务栏里独立的图标，没有标签页也没有地址栏。两分钟就能了解全部。",
  "tour.add.title": "添加一个应用",
  "tour.add.body":
    "在这里粘贴网址并按“添加”。ZWebWrap 会取回站点自己的图标，给它一个窗口，并在你不另作说明时，在应用菜单里创建快捷方式。",
  "tour.import.title": "或者导入书签",
  "tour.import.body":
    "导入会读取 Chrome、Edge、Brave、Vivaldi、Opera 或 Firefox 里已有的书签，由你挑选哪些变成应用。不会向任何地方发送数据；文件只从你自己的磁盘读取。",
  "tour.library.title": "你的应用都在这里",
  "tour.library.body":
    "点击磁贴即可打开。右键可以打开它的设置、复制网址、为该站点关闭广告拦截，或者删除它。同一分组的应用会显示在一起，这里和托盘里都是。",
  "tour.toolbar.title": "隐藏的工具栏",
  "tour.toolbar.body":
    "封装窗口没有任何控件——把指针移到窗口最上边缘，一条小工具栏就会出现：后退、前进、重新加载、页内查找、缩放、置顶、下载，以及在真正的浏览器中打开本页。",
  "tour.blocking.title": "默认拦截广告",
  "tour.blocking.body":
    "这个按钮显示已加载多少条过滤规则，并打开管理它们的设置。拦截按应用设置：完整、仅网络（用于会被外观过滤破坏的站点），或者关闭。",
  "tour.settings.title": "其余一切都在设置里",
  "tour.settings.body":
    "语言、主题、下载位置、开机启动、托盘、过滤规则更新——以及可选的浏览器扩展，它会给 Chrome、Edge 或 Firefox 添加一个“封装此页面”按钮。你也可以在那里重新运行本向导。",
  "settings.about": "关于",
  "about.version": "版本 {version}",
  "about.description":
    "把网站变成桌面应用：独立的窗口、独立的图标，以及把它锁在单一站点上的导航限制。它使用系统本就自带的 webview，而不是打包一个浏览器内核，因此只有几兆字节。",
  "about.author": "作者",
  "about.licence": "许可证",
  "about.licence.hint":
    "自由软件。你可以使用、研究、修改和分享它；凡是基于它构建并分发的内容，都必须保持相同的许可证。",
  "about.links": "链接",
  "about.link.downloads": "下载页面",
  "about.link.source": "源代码",
  "about.link.projects": "更多项目",
  "about.link.author": "作者的 GitHub",
  "about.downloads.hint":
    "预编译版本只在 zsync.eu 提供，绝不会放在 GitHub。GitHub 仓库只存放源代码，所以那里出现的任何预编译 ZWebWrap 都不是我们发布的。",
  "detail.group": "分组",
  "detail.group.placeholder": "工作、个人…",
  "detail.group.hint":
    "同一分组的应用会在你的库和托盘中显示在一起。留空表示不分组。",
};
