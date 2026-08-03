import type { Dictionary } from "./en";

export const tr: Dictionary = {
  "app.tagline": "Bir web sayfası, masaüstü uygulaması olarak. Tarayıcı değil.",
  "app.settings": "Ayarlar",
  "app.blocking.ready": "{count} kural",
  "app.blocking.loading": "Filtreler yükleniyor…",
  "app.blocking.status": "Reklam engelleme durumu",
  "app.search": "Uygulamalarında ara",
  "app.empty.title": "Henüz uygulama yok",
  "app.empty.body":
    "Yukarıya bir adres yapıştır; ZWebWrap onu kendi penceresi ve simgesi olan bir masaüstü uygulamasına dönüştürsün.",
  "app.noMatch": "“{query}” ile eşleşen bir şey yok",
  "app.clearSearch": "Aramayı temizle",
  "app.library": "Uygulamalarınız",

  "media.title": "Bu sistemde video oynatılmayacak",
  "media.body":
    "GStreamer'da {missing} eksik. Her site oynatıcısını gösterir ama hiçbir şey yüklenmez — bu yalnızca ZWebWrap'i değil, tüm WebKit uygulamalarını etkiler.",
  "media.installHint": "Eksik eklentileri kur, sonra yeniden denetle:",
  "media.recheck": "Yeniden denetle",
  "media.found":
    "Medya eklentileri bulundu — uygulamalarınızı yeniden açın, video çalışacak",
  "media.stillMissing": "Hâlâ eksik: {missing}",

  "add.placeholder": "Bir adres yapıştır — youtube.com, claude.ai…",
  "add.submit": "Uygulama oluştur",
  "add.working": "Oluşturuluyor…",
  "add.busy": "Simge alınıyor ve kısayol oluşturuluyor…",
  "add.preview.suffix": "uygulama menünüze bir kısayol ekler",
  "add.hint":
    "Bu siteyi kendi penceresinde açan bir kısayol elde edersin. Sekme yok, adres çubuğu yok.",

  "card.open": "{name} uygulamasını aç",
  "card.settings": "{name} ayarları",
  "card.blocking": "Engelliyor",
  "card.blockingOff": "Kapalı",
  "card.blockingOn.title": "Bu uygulamada reklam engelleme açık",
  "card.blockingOff.title": "Bu uygulamada reklam engelleme kapalı",

  "detail.general": "Genel",
  "detail.behaviour": "Davranış",
  "detail.appearance": "Görünüm",
  "detail.advanced": "Gelişmiş",
  "detail.sections": "Uygulama ayarı bölümleri",

  "detail.name": "Ad",
  "detail.name.hint": "Kısayolda ve pencere başlığında görünür.",
  "detail.address": "Adres",
  "detail.icon.change": "Değiştir",
  "detail.icon.saving": "Kaydediliyor…",
  "detail.icon.reset": "Sıfırla",
  "detail.icon.reset.title": "Sitenin kendi simgesini yeniden getir",

  "detail.adblock": "Reklamları ve izleyicileri engelle",
  "detail.adblock.full": "Tam",
  "detail.adblock.networkOnly": "Yalnızca ağ",
  "detail.adblock.off": "Kapalı",
  "detail.adblock.hint.networkOnly":
    "İstekler yine filtrelenir, ancak sayfaya hiçbir şey eklenmez. Reklam alanlarını gizlemek bir siteyi bozduğunda kullanın.",
  "detail.adblock.hint.off":
    "Bu uygulama için hiçbir şey filtrelenmez veya gizlenmez.",
  "detail.adblock.hint":
    "İstekleri çıkmadan önce süzer ve geriye kalan reklam alanlarını gizler.",
  "detail.shortcut": "Masaüstü kısayolu",
  "detail.shortcut.checking": "Denetleniyor…",
  "detail.shortcut.hint":
    "Bu uygulama her zaman uygulama menünde bulunur. Bu, masaüstündeki kopyasıdır.",
  "detail.fullscreen": "Tam ekran bu pencereyi doldurur",
  "detail.fullscreen.hint":
    "Bir videoyu tam ekran yapmak, tüm ekranı kaplamak yerine bu pencereyi doldurur. Normal tam ekran için kapat.",
  "detail.frame": "Pencere çerçevesi",
  "detail.frame.hint":
    "«Kenarlıksız», masaüstünden yeniden boyutlandırma kenarını bırakmasını ister — birçok temada zaten yoktur, bu yüzden aynı görünebilir. «Başlık çubuğu yok» ise her zaman farklı görünür: taşıma, büyütme ve kapatma açılır çubuğa geçer.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "Kenarlıksız",
  "detail.frame.frameless": "Başlık çubuğu yok",

  "detail.offsite": "Bir bağlantı {host} dışına çıktığında",
  "detail.offsite.aria": "Bir bağlantı bu siteden çıktığında",
  "detail.offsite.hint":
    "Bu pencere tek bir siteye kilitlidir. Başka bir yere giden bağlantı ya normal tarayıcında açılır ya da burada izlenir. Oturum açmalar her durumda burada kalır.",
  "detail.offsite.browser": "Tarayıcıma gönder",
  "detail.offsite.inWindow": "Burada aç",
  "detail.scope": "Bu uygulamanın parçası olan diğer adresler",
  "detail.scope.hint":
    "Tarayıcı yerine pencerede açılması gereken ek alan adları. Virgülle ayrılır.",

  "detail.proxy": "Vekil sunucu",
  "detail.proxy.hint":
    "Bu uygulamanın trafiğini bir vekil sunucu üzerinden gönderir — host:port ya da socks5://host:port. Bir tane ayarlamak uygulamayı kendi oturumuna taşır; her iki webview motoru da vekil sunucuyu pencereye değil oturuma bağlar.",
  "detail.auth": "Site girişi",
  "detail.auth.username": "Kullanıcı adı",
  "detail.auth.password": "Parola",
  "detail.auth.hint":
    "HTTP parola kutusunun arkasındaki siteler için — intranet panolarının kullandığı türden. Yalnızca ilk istekte yanıtlanır; reddedilirse hesabı kilitlemek yerine sitenin kendi kutusu açılır. Veri klasöründeki credentials.json dosyasında saklanır ve senin hesabınla çalışan her şey tarafından okunabilir.",
  "detail.zoom": "Yakınlaştırma",
  "detail.window": "Pencere",
  "detail.window.hint": "Pencereyi kapattığında hatırlanır.",
  "detail.userAgent": "Tarayıcı kimliği",
  "detail.userAgent.placeholder":
    "Varsayılan — WebKit'in kendi kullanıcı aracısı",
  "detail.userAgent.hint":
    "Bu uygulamanın sitelere kendini nasıl tanıttığı. Varsayılan en dürüst olanıdır ve bot denetimlerinin beklediği budur; bazı siteler onu reddeder ve Chrome ister.",

  "detail.delete": "Sil",
  "detail.delete.confirm": "Bu uygulama ve kısayolu silinsin mi?",
  "detail.cancel": "Vazgeç",
  "detail.save": "Kaydet",
  "detail.saving": "Kaydediliyor…",

  "settings.title": "Ayarlar",
  "settings.subtitle": "Tercihler, reklam engelleme ve depolama",
  "settings.sections": "Ayar bölümleri",
  "settings.general": "Genel",
  "settings.blocking": "Reklam engelleme",
  "settings.storage": "Depolama",
  "settings.autosaved": "Değişiklikler kendiliğinden kaydedilir",
  "settings.done": "Tamam",

  "settings.startClose": "Başlatma ve kapatma",
  "settings.autostart": "Sistemle birlikte başlat",
  "settings.autostart.hint":
    "Oturum açtığında sessizce sistem tepsisinde açılır — pencere olmadan.",
  "settings.tray": "Tepsiye kapat",
  "settings.tray.hint":
    "Bir uygulamayı kapatmak onu çıkmak yerine gizler; anında geri gelir ve kaldığı yerden çalmayı sürdürür. Çıkmak için tepsi menüsünü kullan.",
  "settings.tray.note":
    "Bunlardan biri açık olduğu sürece ZWebWrap tepside bir simge tutar. İkisi de kapalıysa geride hiçbir süreç bırakmaz.",

  "settings.windows": "Pencereler",
  "settings.fullscreen": "Tam ekran pencereyi doldurur",
  "settings.fullscreen.hint":
    "Bir videoyu tam ekran yapmak, tüm ekranı kaplamak yerine pencereyi doldurur. Her uygulama bunu kendi ayarlarında değiştirebilir.",

  "settings.newApps": "Yeni uygulamalar",
  "settings.desktopShortcut": "Yeni kısayolları masaüstüne koy",
  "settings.desktopShortcut.hint":
    "Her zaman uygulama menünde görünürler. Bu, ayrıca masaüstüne bir kopya ekler. Her uygulama bunu kendi ayarlarında değiştirebilir.",
  "settings.adblockDefault": "Yeni uygulamalarda reklamları engelle",
  "settings.adblockDefault.hint":
    "Daha önce oluşturduğun uygulamalar kendi ayarlarını korur.",

  "settings.language": "Dil",
  "settings.language.hint": "Burada bir şey seçmezsen sistemi izler.",
  "settings.language.system": "Sistemi izle",
  "settings.theme": "Tema",
  "settings.theme.hint": "Burada bir seçim yapmadıkça masaüstünüzü izler.",
  "settings.theme.light": "Açık",
  "settings.theme.dark": "Koyu",

  "settings.metric.rules": "ağ kuralı",
  "settings.metric.lists": "filtre listesi",
  "settings.metric.updated": "son güncelleme",
  "settings.notReady":
    "Filtre motoru hâlâ yükleniyor. Şimdi açılan pencereler reklamları engellemez.",
  "settings.seedOnly":
    "Yalnızca küçük yerleşik liste etkin — tam filtre listeleri henüz indirilmedi. İstekleri engeller ama geriye kalan reklam alanlarını gizleyemez.",
  "settings.lists": "Filtre listeleri",
  "settings.lists.hint":
    "İlk çalıştırmada indirilir, sonra 4 günde bir kendiliğinden yenilenir.",
  "settings.update": "Şimdi güncelle",
  "settings.updating": "Güncelleniyor…",
  "settings.updated": "Filtreler güncellendi — {count} kural",
  "settings.blocked": "Neler engelleniyor",
  "settings.blocked.body":
    "İstekler çıkmadan önce süzülür. Geriye kalan reklam alanları hem siteye özgü hem genel kurallarla gizlenir ve YouTube video reklamları oynatıcının yanıtından çıkarılır.",
  "settings.dataFolder": "Veri klasörü",
  "settings.dataFolder.hint":
    "Uygulamalar, simgeler, filtre listeleri ve tüm uygulamaların paylaştığı oturum.",
  "settings.open": "Aç",

  "app.blocking.preparing": "Filtreler hazırlanıyor…",
  "app.blocking.basic": "Temel engelleme",
  "menu.open": "Aç",
  "menu.settings": "Ayarlar…",
  "menu.copyAddress": "Adresi kopyala",
  "menu.adblockOff": "Reklam engellemeyi kapat",
  "menu.adblockOn": "Reklam engellemeyi aç",
  "menu.delete": "Sil…",
  "toast.created": "{name} oluşturuldu",
  "toast.saved": "{name} kaydedildi",
  "toast.deleted": "{name} silindi",
  "toast.copied": "Adres kopyalandı",
  "toast.copyFailed": "Adres kopyalanamadı",
  "confirm.delete.title": "{name} silinsin mi?",
  "confirm.delete.body":
    "Masaüstü kısayolu ve simgesi de kaldırılır. Sitenin kendisine dokunulmaz ve istediğin zaman yeniden ekleyebilirsin.",
  "ui.dismiss": "Kapat",
  "ui.close": "Kapat",
  "ui.cancel": "Vazgeç",

  "detail.permissions": "İzinler",
  "detail.perm.notifications": "Bildirimler",
  "detail.perm.microphone": "Mikrofon",
  "detail.perm.camera": "Kamera",
  "detail.perm.location": "Konum",
  "detail.perm.hint":
    "Bu uygulamanın sisteminden neler isteyebileceği. «Sor», bir kez sorulup yanıtın hatırlanması demektir.",
  "rule.allow": "İzin ver",
  "rule.deny": "Engelle",
  "rule.ask": "Sor",

  "detail.badge": "Okunmamış sayısını başlıkta göster",
  "detail.badge.hint":
    "Sayfanın kendi başlığına koyduğu sayıyı — (3) Discord, Gelen Kutusu (12) — bu pencereye yansıtır, böylece görev çubuğunda görünür.",
  "detail.userCss": "Senin CSS'in",
  "detail.userCss.hint":
    "Sayfa çizilmeden önce eklenir. Sitenin gizlemene izin vermediği bir şeyi gizlemek için kullanışlıdır.",
  "detail.userJs": "Senin JavaScript'in",
  "detail.userJs.hint":
    "Belge başlarken sayfanın yetkileriyle çalışır. ZWebWrap'in kendisine erişemez.",

  "detail.hotkey": "Genel kısayol",
  "detail.hotkey.hint":
    "Bu uygulamayı öne getirmek için sistemin herhangi bir yerinde bu tuşlara bas. Öndeyken yeniden basmak onu gizler. Alana tıkla ve bir kombinasyon yaz.",
  "detail.hotkey.record": "Tıkla ve tuşlara bas",
  "detail.hotkey.clear": "Temizle",
  "detail.hotkey.taken": "Bu kombinasyon kabul edilmiyor. Başka birini dene.",
  "import.title": "Tarayıcından içe aktar",
  "import.subtitle": "Zaten sakladığın yer imleri, uygulama olarak",
  "import.reading": "Tarayıcıların okunuyor…",
  "import.none": "Zaten uygulama olmayan yer imi bulunamadı.",
  "import.search": "Yer imlerini süz",
  "import.pick": "İstediklerini seç",
  "import.chosen": "{n} seçildi",
  "import.submit": "{n} tanesini aktar",
  "import.working": "Aktarılıyor…",
  "import.done": "{n} uygulama aktarıldı",
  "app.import": "Tarayıcıdan aktar",
  "settings.platform": "Bu sistem",
  "settings.platform.fine":
    "ZWebWrap'in yaptığı her şey bu sistemde kullanılabilir.",
  "settings.platform.session": "Oturum",
  "time.never": "hiç",
  "time.justNow": "az önce",
  "time.minutes": "{n} dk önce",
  "time.hours": "{n} sa önce",
  "time.days": "{n} gün önce",
  "dl.title": "İndirilenler",
  "dl.empty": "Henüz hiçbir şey indirilmedi.",
  "dl.clear": "Temizle",
  "dl.open": "Aç",
  "dl.reveal": "Klasörde göster",
  "dl.failed": "Başarısız",
  "dl.running": "şu ana kadar {size}",
  "detail.session": "Oturum",
  "detail.session.shared": "Ortak",
  "detail.session.isolated": "Ayrı",
  "detail.session.hint":
    "Girişi diğer uygulamalarınla paylaşır: Google'a bir kez giriş yapmak YouTube, Gmail ve Drive için geçerlidir.",
  "detail.session.hint.isolated":
    "Bu uygulama kendi başına giriş yapar, böylece ikinci bir hesap kullanabilirsin. Oturum açılmamış başlar. Bir sonraki açılışta geçerli olur.",
  "settings.downloads": "İndirilenler",
  "settings.askWhereToSave": "Her dosya için kaydetme yeri sorulsun",
  "settings.askWhereToSave.hint":
    "Uygulama penceresinde indirme çubuğu yoktur, bu yüzden sessizce kaydedilen bir dosya gözden kaçar. Her şeyin doğrudan İndirilenler klasörüne gitmesi için kapatın.",
  "dl.toast.done": "{name} indirildi",
  "dl.toast.failed": "{name} indirilemedi",
  "settings.metric.hidden": "gizlenen reklam",
  "settings.tour": "Rehberli tur",
  "settings.tour.hint":
    "Yeni bir kurulumda bir kez gösterilir. İstediğin zaman yeniden çalıştırabilirsin.",
  "settings.tour.button": "Turu göster",
  "tour.title": "Kısa bir tur",
  "tour.step": "Adım {n} / {total}",
  "tour.skip": "Turu atla",
  "tour.back": "Geri",
  "tour.next": "İleri",
  "tour.done": "Bitir",
  "tour.welcome.title": "ZWebWrap'e hoş geldin",
  "tour.welcome.body":
    "ZWebWrap bir web sitesini masaüstü uygulamasına çevirir: kendi penceresi, görev çubuğunda kendi simgesi, sekme ve adres çubuğu yok. İki dakikada hepsini öğrenirsin.",
  "tour.add.title": "Uygulama ekle",
  "tour.add.body":
    "Buraya bir adres yapıştır ve Ekle'ye bas. ZWebWrap sitenin kendi simgesini indirir, ona bir pencere verir ve — aksini söylemezsen — uygulama menünde bir kısayol oluşturur.",
  "tour.import.title": "Ya da yer imlerini getir",
  "tour.import.body":
    "İçe aktarma, Chrome, Edge, Brave, Vivaldi, Opera veya Firefox'ta zaten duran yer imlerini okur ve hangilerinin uygulamaya dönüşeceğini sana seçtirir. Hiçbir şey hiçbir yere gönderilmez; dosyalar kendi diskinden okunur.",
  "tour.library.title": "Uygulamaların burada durur",
  "tour.library.body":
    "Açmak için bir karta tıkla. Sağ tıklamak ayarlarını açar, adresini kopyalar, o site için reklam engellemeyi kapatır ya da uygulamayı siler. Aynı gruptaki uygulamalar burada ve sistem tepsisinde birlikte görünür.",
  "tour.toolbar.title": "Gizli araç çubuğu",
  "tour.toolbar.body":
    "Sarmalanmış bir pencerede hiç düğme yoktur — imleci en üst kenara götür, küçük bir çubuk belirir: geri, ileri, yeniden yükle, sayfada bul, yakınlaştırma, üstte tut, indirmeler ve bu sayfayı gerçek tarayıcında aç.",
  "tour.blocking.title": "Reklamlar varsayılan olarak engellenir",
  "tour.blocking.body":
    "Bu düğme kaç filtre kuralının yüklü olduğunu gösterir ve onları yöneten ayarları açar. Engelleme uygulama başına seçilir: tam, kozmetik filtrelemenin bozduğu siteler için yalnızca ağ, ya da kapalı.",
  "tour.settings.title": "Geri kalan her şey ayarlarda",
  "tour.settings.body":
    "Dil, tema, indirmelerin nereye gideceği, sistemle başlatma, sistem tepsisi, filtre güncellemeleri — ve Chrome, Edge veya Firefox'a «Bu sayfayı sarmala» düğmesi ekleyen isteğe bağlı tarayıcı eklentisi. Bu turu oradan yeniden başlatabilirsin.",
  "settings.about": "Hakkında",
  "about.version": "Sürüm {version}",
  "about.description":
    "Bir web sitesini masaüstü uygulamasına çevirir: kendi penceresi, kendi simgesi ve onu tek bir sitede tutan bir gezinme kilidi. Paketlenmiş bir tarayıcı motoru yerine sisteminde zaten bulunan webview üzerine kurulur; birkaç megabayt olmasının sebebi budur.",
  "about.author": "Yazar",
  "about.licence": "Lisans",
  "about.licence.hint":
    "Özgür yazılım. Kullanabilir, inceleyebilir, değiştirebilir ve paylaşabilirsin; ondan türetip dağıttığın her şey aynı lisansta kalmalıdır.",
  "about.links": "Bağlantılar",
  "about.link.downloads": "İndirme sayfası",
  "about.link.source": "Kaynak kodu",
  "about.link.projects": "Diğer projeler",
  "about.link.author": "Yazarın GitHub sayfası",
  "about.downloads.hint":
    "Hazır sürümler yalnızca zsync.eu'da bulunur, GitHub'da asla. GitHub deposunda sadece kaynak kod vardır; orada sunulan hazır bir ZWebWrap bizden gelmemiştir.",
  "detail.group": "Grup",
  "detail.group.placeholder": "İş, Kişisel…",
  "detail.group.hint":
    "Aynı gruptaki uygulamalar kitaplığınızda ve sistem tepsisinde birlikte gösterilir. Grupsuz için boş bırakın.",
};
