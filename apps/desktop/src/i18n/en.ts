export const en = {
  "app.tagline": "A webpage, as a desktop app. Not a browser.",
  "app.settings": "Settings",
  "app.blocking.ready": "{count} rules",
  "app.blocking.loading": "Loading filters…",
  "app.blocking.status": "Ad blocking status",
  "app.search": "Search your apps",
  "app.empty.title": "No apps yet",
  "app.empty.body":
    "Paste a URL above and ZWebWrap turns it into a desktop app with its own window and icon.",
  "app.noMatch": "Nothing matches “{query}”",
  "app.clearSearch": "Clear search",
  "app.library": "Your apps",

  "media.title": "Video will not play on this system",
  "media.body":
    "GStreamer is missing {missing}. Every wrapped site will show its player but nothing will ever load — this affects all WebKit apps, not just ZWebWrap.",
  "media.installHint": "Install the missing plugins, then check again:",
  "media.recheck": "Check again",
  "media.found": "Media plugins found — reopen your apps and video will work",
  "media.stillMissing": "Still missing: {missing}",

  "add.placeholder": "Paste a URL — youtube.com, claude.ai…",
  "add.submit": "Create app",
  "add.working": "Creating…",
  "add.busy": "Fetching the icon and creating your shortcut…",
  "add.preview.suffix": "adds a shortcut to your app menu",
  "add.hint":
    "You get a desktop shortcut that opens this site in its own window. No tabs, no address bar.",

  "card.open": "Open {name}",
  "card.settings": "Settings for {name}",
  "card.blocking": "Blocking",
  "card.blockingOff": "Off",
  "card.blockingOn.title": "Ad blocking is on for this app",
  "card.blockingOff.title": "Ad blocking is off for this app",

  "detail.general": "General",
  "detail.behaviour": "Behaviour",
  "detail.appearance": "Appearance",
  "detail.advanced": "Advanced",
  "detail.sections": "App settings sections",

  "detail.name": "Name",
  "detail.name.hint": "Shown on the shortcut and in the window title.",
  "detail.address": "Address",
  "detail.icon.change": "Change",
  "detail.icon.saving": "Saving…",
  "detail.icon.reset": "Reset",
  "detail.icon.reset.title": "Fetch the site's own icon again",

  "detail.adblock": "Block ads and trackers",
  "detail.adblock.full": "Full",
  "detail.adblock.networkOnly": "Network only",
  "detail.adblock.off": "Off",
  "detail.adblock.hint.networkOnly":
    "Requests are still filtered, but nothing is injected into the page. Use this when hiding ad slots breaks a site.",
  "detail.adblock.hint.off": "Nothing is filtered or hidden for this app.",
  "detail.adblock.hint":
    "Filters requests before they leave and hides leftover ad slots.",
  "detail.shortcut": "Desktop shortcut",
  "detail.shortcut.checking": "Checking…",
  "detail.shortcut.hint":
    "This app always has an entry in your app menu. This is the copy on your desktop.",
  "detail.fullscreen": "Fullscreen fills this window",
  "detail.fullscreen.hint":
    "Clicking fullscreen on a video fills this window instead of taking over the whole screen. Turn off for a normal fullscreen.",
  "detail.frame": "Window frame",
  "detail.frame.hint":
    "No border asks your desktop to drop the resize edge — many themes already have none, so it may look identical. No titlebar always looks different: the hover toolbar carries move, maximise and close instead.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "No border",
  "detail.frame.frameless": "No titlebar",

  "detail.offsite": "When a link leaves {host}",
  "detail.offsite.aria": "When a link leaves this site",
  "detail.offsite.hint":
    "This window is locked to one site. A link pointing anywhere else either opens in your normal browser, or is followed here. Sign-ins stay here either way.",
  "detail.offsite.browser": "Send to my browser",
  "detail.offsite.inWindow": "Follow it here",
  "detail.scope": "Also part of this app",
  "detail.scope.hint":
    "Extra domains that should open in-window instead of the browser. Comma separated.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Send this app's traffic through a proxy — host:port, or socks5://host:port. Setting one moves this app to its own session, because both webview engines attach a proxy to the login session rather than to the window.",
  "detail.auth": "Website login",
  "detail.auth.username": "Username",
  "detail.auth.password": "Password",
  "detail.auth.hint":
    "For a site behind an HTTP password box, the kind an intranet dashboard uses. Answered on the first prompt only — if it is rejected, the site's own box appears rather than locking the account out. Stored in credentials.json in your data folder, readable by anything running as you.",

  "detail.zoom": "Zoom",
  "detail.window": "Window",
  "detail.window.hint": "Remembered when you close the window.",
  "detail.userAgent": "Browser identity",
  "detail.userAgent.placeholder": "Default — WebKit's own user agent",
  "detail.userAgent.hint":
    "What this app tells websites it is. The default is the most honest and what bot checks expect; some sites reject it and need Chrome.",

  "detail.delete": "Delete",
  "detail.delete.confirm": "Delete this app and its shortcut?",
  "detail.cancel": "Cancel",
  "detail.save": "Save",
  "detail.saving": "Saving…",

  "settings.title": "Settings",
  "settings.subtitle": "Preferences, ad blocking and storage",
  "settings.sections": "Settings sections",
  "settings.general": "General",
  "settings.blocking": "Ad blocking",
  "settings.storage": "Storage",
  "settings.autosaved": "Changes save as you make them",
  "settings.done": "Done",

  "settings.startClose": "Starting and closing",
  "settings.autostart": "Start with the system",
  "settings.autostart.hint":
    "Opens quietly in the tray when you log in — no window.",
  "settings.tray": "Close to tray",
  "settings.tray.hint":
    "Closing a wrapped app hides it instead of quitting, so it reopens instantly and keeps playing where it was. Quit from the tray menu.",
  "settings.tray.note":
    "ZWebWrap keeps a tray icon while either of these is on. With both off it leaves no process behind.",

  "settings.windows": "Windows",
  "settings.fullscreen": "Fullscreen fills the window",
  "settings.fullscreen.hint":
    "Clicking fullscreen on a video fills the wrapped window instead of taking over the whole screen. Any app can override this in its own settings.",

  "settings.newApps": "New apps",
  "settings.desktopShortcut": "Put new shortcuts on the desktop",
  "settings.desktopShortcut.hint":
    "They always appear in your app menu. This adds a copy to your desktop as well. Each app can override it in its own settings.",
  "settings.adblockDefault": "Block ads in new apps",
  "settings.adblockDefault.hint":
    "Apps you already made keep their own setting.",

  "settings.language": "Language",
  "settings.language.hint": "Follows your system unless you pick one here.",
  "settings.language.system": "Follow the system",
  "settings.theme": "Theme",
  "settings.theme.hint": "Follows your desktop unless you pick one here.",
  "settings.theme.light": "Light",
  "settings.theme.dark": "Dark",

  "settings.metric.rules": "network rules",
  "settings.metric.lists": "filter lists",
  "settings.metric.updated": "last updated",
  "settings.notReady":
    "The filter engine is still loading. Windows opened right now will not block ads.",
  "settings.seedOnly":
    "Only the small built-in list is active — the full filter lists have not downloaded yet. It blocks ad requests but cannot hide leftover ad slots.",
  "settings.lists": "Filter lists",
  "settings.lists.hint":
    "Downloaded on first run, then refreshed every 4 days on their own.",
  "settings.update": "Update now",
  "settings.updating": "Updating…",
  "settings.updated": "Filters updated — {count} rules",
  "settings.blocked": "What gets blocked",
  "settings.blocked.body":
    "Requests are filtered before they leave. Leftover ad slots are hidden using both site-specific and generic rules, and YouTube video ads are stripped from the player response itself.",
  "settings.dataFolder": "Data folder",
  "settings.dataFolder.hint":
    "Apps, icons, filter lists and the login session shared by every wrapped app.",
  "settings.open": "Open",

  "app.blocking.preparing": "Preparing filters…",
  "app.blocking.basic": "Basic blocking",
  "menu.open": "Open",
  "menu.settings": "Settings…",
  "menu.copyAddress": "Copy address",
  "menu.adblockOff": "Turn off ad blocking",
  "menu.adblockOn": "Turn on ad blocking",
  "menu.delete": "Delete…",
  "toast.created": "Created {name}",
  "toast.saved": "Saved {name}",
  "toast.deleted": "Deleted {name}",
  "toast.copied": "Address copied",
  "toast.copyFailed": "Could not copy the address",
  "confirm.delete.title": "Delete {name}?",
  "confirm.delete.body":
    "Its desktop shortcut and icon are removed too. The site itself is untouched, and you can add it again at any time.",
  "ui.dismiss": "Dismiss",
  "ui.close": "Close",
  "ui.cancel": "Cancel",
  "detail.permissions": "Permissions",
  "detail.perm.notifications": "Notifications",
  "detail.perm.microphone": "Microphone",
  "detail.perm.camera": "Camera",
  "detail.perm.location": "Location",
  "detail.perm.hint":
    "What this app may ask your system for. Ask means you are prompted once, then the answer is remembered.",
  "rule.allow": "Allow",
  "rule.deny": "Block",
  "rule.ask": "Ask",
  "detail.badge": "Show unread count in the title",
  "detail.badge.hint":
    "Mirrors a count the page puts in its own title — (3) Discord, Inbox (12) — onto this window, so your taskbar shows it.",
  "detail.userCss": "Your CSS",
  "detail.userCss.hint":
    "Injected into this app before it paints. Useful for hiding something the site will not let you hide.",
  "detail.userJs": "Your JavaScript",
  "detail.userJs.hint":
    "Runs at document start with the page's own privileges. It cannot reach ZWebWrap itself.",
  "detail.hotkey": "Global shortcut",
  "detail.hotkey.hint":
    "Press the keys you want, anywhere in the system, to bring this app forward. Press again while it is in front to send it away. Click the field and type a combination.",
  "detail.hotkey.record": "Click and press keys",
  "detail.hotkey.clear": "Clear",
  "detail.hotkey.taken": "That combination is not accepted. Try another.",
  "import.title": "Import from your browser",
  "import.subtitle": "Bookmarks you already keep, as desktop apps",
  "import.reading": "Reading your browsers…",
  "import.none": "No bookmarks found that aren't already apps.",
  "import.search": "Filter bookmarks",
  "import.pick": "Pick the ones you want",
  "import.chosen": "{n} selected",
  "import.submit": "Import {n}",
  "import.working": "Importing…",
  "import.done": "Imported {n} apps",
  "app.import": "Import from browser",
  "settings.platform": "This system",
  "settings.platform.fine":
    "Everything ZWebWrap does is available on this system.",
  "settings.platform.session": "Session",
  "time.never": "never",
  "time.justNow": "just now",
  "time.minutes": "{n}m ago",
  "time.hours": "{n}h ago",
  "time.days": "{n}d ago",
  "dl.title": "Downloads",
  "dl.empty": "Nothing downloaded yet.",
  "dl.clear": "Clear",
  "dl.open": "Open",
  "dl.reveal": "Show in folder",
  "dl.failed": "Failed",
  "dl.running": "{size} so far",
  "detail.session": "Login session",
  "detail.session.shared": "Shared",
  "detail.session.isolated": "Separate",
  "detail.session.hint":
    "Shares one login with your other apps, so signing in to Google once covers YouTube, Gmail and Drive.",
  "detail.session.hint.isolated":
    "This app signs in on its own, so you can use a second account. It starts logged out. Takes effect next time you open it.",
  "settings.downloads": "Downloads",
  "settings.askWhereToSave": "Ask where to save each file",
  "settings.askWhereToSave.hint":
    "A wrapped window has no download bar, so a file that saves itself is easy to miss. Turn this off to send everything straight to your Downloads folder.",
  "dl.toast.done": "Downloaded {name}",
  "dl.toast.failed": "Could not download {name}",
  "settings.metric.hidden": "ads hidden",

  "settings.tour": "Guided tour",
  "settings.tour.hint":
    "Shown once on a fresh install. Run it again whenever you like.",
  "settings.tour.button": "Show the tour",
  "tour.title": "A quick tour",
  "tour.step": "Step {n} of {total}",
  "tour.skip": "Skip the tour",
  "tour.back": "Back",
  "tour.next": "Next",
  "tour.done": "Finish",
  "tour.welcome.title": "Welcome to ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap turns a website into a desktop app: its own window, its own icon in your taskbar, and no tabs or address bar. Two minutes and you will know the whole thing.",
  "tour.add.title": "Add an app",
  "tour.add.body":
    "Paste an address here and press Add. ZWebWrap fetches the site's own icon, gives it a window and — unless you say otherwise — a shortcut in your app menu.",
  "tour.import.title": "Or bring your bookmarks",
  "tour.import.body":
    "Import reads the bookmarks already in Chrome, Edge, Brave, Vivaldi, Opera or Firefox, and lets you pick which ones become apps. Nothing is sent anywhere; the files are read from your own disk.",
  "tour.library.title": "Your apps live here",
  "tour.library.body":
    "Click a tile to open it. Right-click one for its settings, to copy its address, to switch ad blocking off for that site, or to delete it. Give apps a group and they are shown together, here and in the tray.",
  "tour.toolbar.title": "The hidden toolbar",
  "tour.toolbar.body":
    "A wrapped window has no chrome — so move the pointer to the very top edge of one and a small toolbar appears: back, forward, reload, find on the page, zoom, keep on top, downloads, and open this page in your real browser.",
  "tour.blocking.title": "Ads are blocked by default",
  "tour.blocking.body":
    "This button shows how many filter rules are loaded, and opens the settings that manage them. Blocking is per app: full, network only for a site that cosmetic filtering breaks, or off.",
  "tour.settings.title": "Everything else is in settings",
  "tour.settings.body":
    "Language, theme, where downloads go, starting with the system, the tray, filter updates — and the optional browser extension, which adds a “Wrap this page” button to Chrome, Edge or Firefox. You can run this tour again from there.",

  "settings.about": "About",
  "about.version": "Version {version}",
  "about.description":
    "Turns a website into a desktop app: its own window, its own icon, and a navigation lock that keeps it to one site. Built on the webview your system already ships rather than a bundled browser engine, which is why it installs in a few megabytes.",
  "about.author": "Author",
  "about.licence": "Licence",
  "about.licence.hint":
    "Free software. You may use, study, change and share it, and anything you distribute built from it must stay under the same licence.",
  "about.links": "Links",
  "about.link.downloads": "Download page",
  "about.link.source": "Source code",
  "about.link.projects": "More projects",
  "about.link.author": "The author on GitHub",
  "about.downloads.hint":
    "Ready-built versions are only on zsync.eu — never on GitHub. The GitHub repository holds the source code alone, so anything offering a prebuilt ZWebWrap there did not come from us.",
  "detail.group": "Group",
  "detail.group.placeholder": "Work, Personal…",
  "detail.group.hint":
    "Apps with the same group are shown together in your library and in the tray. Leave empty for none.",
} as const;

export type Key = keyof typeof en;

export type Dictionary = Record<Key, string>;
