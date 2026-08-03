import type { Dictionary } from "./en";

export const nl: Dictionary = {
  "app.tagline": "Een webpagina als bureaubladprogramma. Geen browser.",
  "app.settings": "Instellingen",
  "app.blocking.ready": "{count} regels",
  "app.blocking.loading": "Filters laden…",
  "app.blocking.status": "Status van de advertentieblokkering",
  "app.search": "Zoek in je apps",
  "app.empty.title": "Nog geen apps",
  "app.empty.body":
    "Plak hierboven een adres en ZWebWrap maakt er een bureaubladprogramma van met een eigen venster en pictogram.",
  "app.noMatch": "Niets komt overeen met “{query}”",
  "app.clearSearch": "Zoekopdracht wissen",
  "app.library": "Je apps",

  "media.title": "Op dit systeem wordt geen video afgespeeld",
  "media.body":
    "GStreamer mist {missing}. Elke site toont wel zijn speler maar laadt nooit iets — dit geldt voor alle WebKit-programma's, niet alleen ZWebWrap.",
  "media.installHint":
    "Installeer de ontbrekende plug-ins en controleer opnieuw:",
  "media.recheck": "Opnieuw controleren",
  "media.found": "Mediaplug-ins gevonden — open je apps opnieuw en video werkt",
  "media.stillMissing": "Ontbreekt nog steeds: {missing}",

  "add.placeholder": "Plak een adres — youtube.com, claude.ai…",
  "add.submit": "App maken",
  "add.working": "Bezig met maken…",
  "add.busy": "Het pictogram ophalen en je snelkoppeling maken…",
  "add.preview.suffix": "voegt een snelkoppeling toe aan je app-menu",
  "add.hint":
    "Je krijgt een snelkoppeling die deze site in een eigen venster opent. Geen tabbladen, geen adresbalk.",

  "card.open": "{name} openen",
  "card.settings": "Instellingen voor {name}",
  "card.blocking": "Blokkeert",
  "card.blockingOff": "Uit",
  "card.blockingOn.title": "Advertentieblokkering staat aan voor deze app",
  "card.blockingOff.title": "Advertentieblokkering staat uit voor deze app",

  "detail.general": "Algemeen",
  "detail.behaviour": "Gedrag",
  "detail.appearance": "Weergave",
  "detail.advanced": "Geavanceerd",
  "detail.sections": "Onderdelen van de app-instellingen",

  "detail.name": "Naam",
  "detail.name.hint": "Staat op de snelkoppeling en in de venstertitel.",
  "detail.address": "Adres",
  "detail.icon.change": "Wijzigen",
  "detail.icon.saving": "Opslaan…",
  "detail.icon.reset": "Herstellen",
  "detail.icon.reset.title": "Het pictogram van de site opnieuw ophalen",

  "detail.adblock": "Advertenties en trackers blokkeren",
  "detail.adblock.full": "Volledig",
  "detail.adblock.networkOnly": "Alleen netwerk",
  "detail.adblock.off": "Uit",
  "detail.adblock.hint.networkOnly":
    "Verzoeken worden nog steeds gefilterd, maar er wordt niets in de pagina geïnjecteerd. Gebruik dit wanneer het verbergen van advertentievakken een site kapotmaakt.",
  "detail.adblock.hint.off":
    "Voor deze app wordt niets gefilterd of verborgen.",
  "detail.adblock.hint":
    "Filtert verzoeken voordat ze vertrekken en verbergt overgebleven advertentievlakken.",
  "detail.shortcut": "Snelkoppeling op het bureaublad",
  "detail.shortcut.checking": "Controleren…",
  "detail.shortcut.hint":
    "Deze app staat altijd in je programmamenu. Dit is de kopie op je bureaublad.",
  "detail.fullscreen": "Volledig scherm vult dit venster",
  "detail.fullscreen.hint":
    "Een video op volledig scherm vult dit venster in plaats van het hele scherm over te nemen. Zet uit voor een gewoon volledig scherm.",
  "detail.frame": "Vensterrand",
  "detail.frame.hint":
    "„Zonder rand” vraagt je bureaublad de formaatrand weg te laten — veel thema's hebben er al geen, dus het kan er hetzelfde uitzien. „Zonder titelbalk” ziet er altijd anders uit: verplaatsen, maximaliseren en sluiten gaan dan via de zwevende balk.",
  "detail.frame.normal": "Normaal",
  "detail.frame.noBorder": "Zonder rand",
  "detail.frame.frameless": "Zonder titelbalk",

  "detail.offsite": "Wanneer een link {host} verlaat",
  "detail.offsite.aria": "Wanneer een link deze site verlaat",
  "detail.offsite.hint":
    "Dit venster is aan één site gebonden. Een link naar iets anders opent in je gewone browser, of wordt hier gevolgd. Aanmeldingen blijven hoe dan ook hier.",
  "detail.offsite.browser": "Naar mijn browser sturen",
  "detail.offsite.inWindow": "Hier openen",
  "detail.scope": "Hoort ook bij deze app",
  "detail.scope.hint":
    "Extra domeinen die in het venster moeten openen in plaats van in de browser. Gescheiden door komma's.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Stuurt het verkeer van deze app via een proxy — host:poort, of socks5://host:poort. Eén instellen geeft deze app een eigen sessie, want beide webview-engines hangen een proxy aan de sessie in plaats van aan het venster.",
  "detail.auth": "Website-aanmelding",
  "detail.auth.username": "Gebruikersnaam",
  "detail.auth.password": "Wachtwoord",
  "detail.auth.hint":
    "Voor een site achter een HTTP-wachtwoordvenster, zoals intranetdashboards gebruiken. Wordt alleen bij de eerste vraag ingevuld — wordt het geweigerd, dan verschijnt het venster van de site zelf in plaats van het account te blokkeren. Opgeslagen in credentials.json in je gegevensmap, leesbaar voor alles dat onder jouw account draait.",
  "detail.zoom": "Zoom",
  "detail.window": "Venster",
  "detail.window.hint": "Onthouden zodra je het venster sluit.",
  "detail.userAgent": "Browseridentiteit",
  "detail.userAgent.placeholder": "Standaard — de user agent van WebKit zelf",
  "detail.userAgent.hint":
    "Wat deze app tegen websites zegt te zijn. De standaard is de eerlijkste en wat botcontroles verwachten; sommige sites weigeren die en willen Chrome.",

  "detail.delete": "Verwijderen",
  "detail.delete.confirm": "Deze app en de snelkoppeling verwijderen?",
  "detail.cancel": "Annuleren",
  "detail.save": "Opslaan",
  "detail.saving": "Opslaan…",

  "settings.title": "Instellingen",
  "settings.subtitle": "Voorkeuren, advertentieblokkering en opslag",
  "settings.sections": "Onderdelen van de instellingen",
  "settings.general": "Algemeen",
  "settings.blocking": "Advertentieblokkering",
  "settings.storage": "Opslag",
  "settings.autosaved": "Wijzigingen worden meteen opgeslagen",
  "settings.done": "Klaar",

  "settings.startClose": "Starten en sluiten",
  "settings.autostart": "Starten met het systeem",
  "settings.autostart.hint":
    "Opent rustig in het systeemvak wanneer je aanmeldt — zonder venster.",
  "settings.tray": "Sluiten naar het systeemvak",
  "settings.tray.hint":
    "Een app sluiten verbergt hem in plaats van af te sluiten, dus hij is meteen terug en speelt verder waar hij was. Afsluiten via het menu in het systeemvak.",
  "settings.tray.note":
    "ZWebWrap houdt een pictogram in het systeemvak zolang een van deze aanstaat. Staan ze allebei uit, dan blijft er geen proces achter.",

  "settings.windows": "Vensters",
  "settings.fullscreen": "Volledig scherm vult het venster",
  "settings.fullscreen.hint":
    "Een video op volledig scherm vult het venster in plaats van het hele scherm over te nemen. Elke app kan dit in zijn eigen instellingen anders zetten.",

  "settings.newApps": "Nieuwe apps",
  "settings.desktopShortcut": "Nieuwe snelkoppelingen op het bureaublad zetten",
  "settings.desktopShortcut.hint":
    "Ze staan altijd in je programmamenu. Dit zet er ook een kopie op je bureaublad bij. Elke app kan dit anders zetten.",
  "settings.adblockDefault": "Advertenties blokkeren in nieuwe apps",
  "settings.adblockDefault.hint":
    "Apps die je al hebt gemaakt houden hun eigen instelling.",

  "settings.language": "Taal",
  "settings.language.hint": "Volgt je systeem, tenzij je hier iets kiest.",
  "settings.language.system": "Het systeem volgen",
  "settings.theme": "Thema",
  "settings.theme.hint": "Volgt je bureaublad tenzij je hier iets kiest.",
  "settings.theme.light": "Licht",
  "settings.theme.dark": "Donker",

  "settings.metric.rules": "netwerkregels",
  "settings.metric.lists": "filterlijsten",
  "settings.metric.updated": "laatst bijgewerkt",
  "settings.notReady":
    "De filtermotor laadt nog. Vensters die je nu opent blokkeren geen advertenties.",
  "settings.seedOnly":
    "Alleen de kleine ingebouwde lijst is actief — de volledige filterlijsten zijn nog niet gedownload. Die blokkeert verzoeken maar kan overgebleven advertentievlakken niet verbergen.",
  "settings.lists": "Filterlijsten",
  "settings.lists.hint":
    "Gedownload bij de eerste start, daarna elke 4 dagen vanzelf ververst.",
  "settings.update": "Nu bijwerken",
  "settings.updating": "Bijwerken…",
  "settings.updated": "Filters bijgewerkt — {count} regels",
  "settings.blocked": "Wat er geblokkeerd wordt",
  "settings.blocked.body":
    "Verzoeken worden gefilterd voordat ze vertrekken. Overgebleven advertentievlakken worden verborgen met site-specifieke en algemene regels, en YouTube-videoadvertenties worden uit het antwoord van de speler zelf gehaald.",
  "settings.dataFolder": "Gegevensmap",
  "settings.dataFolder.hint":
    "Apps, pictogrammen, filterlijsten en de aanmeldsessie die alle apps delen.",
  "settings.open": "Openen",

  "app.blocking.preparing": "Filters voorbereiden…",
  "app.blocking.basic": "Basisblokkering",
  "menu.open": "Openen",
  "menu.settings": "Instellingen…",
  "menu.copyAddress": "Adres kopiëren",
  "menu.adblockOff": "Advertentieblokkering uitzetten",
  "menu.adblockOn": "Advertentieblokkering aanzetten",
  "menu.delete": "Verwijderen…",
  "toast.created": "{name} gemaakt",
  "toast.saved": "{name} opgeslagen",
  "toast.deleted": "{name} verwijderd",
  "toast.copied": "Adres gekopieerd",
  "toast.copyFailed": "Kon het adres niet kopiëren",
  "confirm.delete.title": "{name} verwijderen?",
  "confirm.delete.body":
    "De snelkoppeling en het pictogram gaan ook weg. De site zelf blijft ongemoeid en je kunt hem altijd opnieuw toevoegen.",
  "ui.dismiss": "Sluiten",
  "ui.close": "Sluiten",
  "ui.cancel": "Annuleren",

  "detail.permissions": "Toestemmingen",
  "detail.perm.notifications": "Meldingen",
  "detail.perm.microphone": "Microfoon",
  "detail.perm.camera": "Camera",
  "detail.perm.location": "Locatie",
  "detail.perm.hint":
    "Wat deze app aan je systeem mag vragen. „Vragen” betekent dat het je één keer gevraagd wordt en het antwoord daarna onthouden blijft.",
  "rule.allow": "Toestaan",
  "rule.deny": "Blokkeren",
  "rule.ask": "Vragen",

  "detail.badge": "Ongelezen aantal in de titel tonen",
  "detail.badge.hint":
    "Neemt het aantal dat de pagina in haar eigen titel zet — (3) Discord, Postvak (12) — over in dit venster, zodat je taakbalk het toont.",
  "detail.userCss": "Jouw CSS",
  "detail.userCss.hint":
    "Wordt ingevoegd voordat de pagina getekend wordt. Handig om iets te verbergen wat de site zelf niet laat verbergen.",
  "detail.userJs": "Jouw JavaScript",
  "detail.userJs.hint":
    "Draait bij het begin van het document met de rechten van de pagina. Het komt niet bij ZWebWrap zelf.",

  "detail.hotkey": "Globale sneltoets",
  "detail.hotkey.hint":
    "Druk deze toetsen waar dan ook in het systeem om deze app naar voren te halen. Nog eens drukken terwijl hij vooraan staat verbergt hem weer. Klik in het veld en typ een combinatie.",
  "detail.hotkey.record": "Klik en druk toetsen",
  "detail.hotkey.clear": "Wissen",
  "detail.hotkey.taken":
    "Die combinatie wordt niet geaccepteerd. Probeer een andere.",
  "import.title": "Importeren uit je browser",
  "import.subtitle": "Bladwijzers die je al hebt, als apps",
  "import.reading": "Je browsers worden gelezen…",
  "import.none": "Geen bladwijzers gevonden die nog geen app zijn.",
  "import.search": "Bladwijzers filteren",
  "import.pick": "Kies wat je wilt",
  "import.chosen": "{n} geselecteerd",
  "import.submit": "{n} importeren",
  "import.working": "Importeren…",
  "import.done": "{n} apps geïmporteerd",
  "app.import": "Uit browser importeren",
  "settings.platform": "Dit systeem",
  "settings.platform.fine":
    "Alles wat ZWebWrap doet is op dit systeem beschikbaar.",
  "settings.platform.session": "Sessie",
  "time.never": "nooit",
  "time.justNow": "zojuist",
  "time.minutes": "{n} min geleden",
  "time.hours": "{n} u geleden",
  "time.days": "{n} d geleden",
  "dl.title": "Downloads",
  "dl.empty": "Nog niets gedownload.",
  "dl.clear": "Wissen",
  "dl.open": "Openen",
  "dl.reveal": "Toon in map",
  "dl.failed": "Mislukt",
  "dl.running": "{size} tot nu toe",
  "detail.session": "Aanmeldsessie",
  "detail.session.shared": "Gedeeld",
  "detail.session.isolated": "Apart",
  "detail.session.hint":
    "Deelt één aanmelding met je andere apps: eenmaal bij Google aanmelden geldt voor YouTube, Gmail en Drive.",
  "detail.session.hint.isolated":
    "Deze app meldt zich zelfstandig aan, zodat je een tweede account kunt gebruiken. Hij start afgemeld. Werkt zodra je hem opnieuw opent.",
  "settings.downloads": "Downloads",
  "settings.askWhereToSave": "Vragen waar elk bestand moet worden opgeslagen",
  "settings.askWhereToSave.hint":
    "Een app-venster heeft geen downloadbalk, dus een bestand dat zichzelf opslaat mis je snel. Zet dit uit om alles rechtstreeks naar je Downloads-map te sturen.",
  "dl.toast.done": "{name} gedownload",
  "dl.toast.failed": "Kon {name} niet downloaden",
  "settings.metric.hidden": "advertenties verborgen",
  "settings.tour": "Rondleiding",
  "settings.tour.hint":
    "Wordt één keer getoond bij een verse installatie. Je kunt hem altijd opnieuw starten.",
  "settings.tour.button": "Toon de rondleiding",
  "tour.title": "Een korte rondleiding",
  "tour.step": "Stap {n} van {total}",
  "tour.skip": "Rondleiding overslaan",
  "tour.back": "Terug",
  "tour.next": "Volgende",
  "tour.done": "Klaar",
  "tour.welcome.title": "Welkom bij ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap maakt van een website een bureaubladtoepassing: een eigen venster, een eigen pictogram in de taakbalk, geen tabbladen en geen adresbalk. Twee minuten en je kent het geheel.",
  "tour.add.title": "Voeg een app toe",
  "tour.add.body":
    "Plak hier een adres en druk op Toevoegen. ZWebWrap haalt het pictogram van de site op, geeft hem een venster en — tenzij je anders zegt — een snelkoppeling in je programmamenu.",
  "tour.import.title": "Of neem je bladwijzers mee",
  "tour.import.body":
    "Importeren leest de bladwijzers die al in Chrome, Edge, Brave, Vivaldi, Opera of Firefox staan en laat je kiezen welke apps worden. Er wordt niets verstuurd; de bestanden worden van je eigen schijf gelezen.",
  "tour.library.title": "Hier wonen je apps",
  "tour.library.body":
    "Klik op een tegel om hem te openen. Met de rechtermuisknop open je de instellingen, kopieer je het adres, zet je advertentieblokkering voor die site uit of verwijder je hem. Apps met dezelfde groep staan bij elkaar, hier en in het systeemvak.",
  "tour.toolbar.title": "De verborgen werkbalk",
  "tour.toolbar.body":
    "Een ingepakt venster heeft geen knoppenbalk — beweeg de muis naar de bovenrand en er verschijnt een kleine balk: terug, vooruit, herladen, zoeken op de pagina, zoom, altijd bovenop, downloads, en deze pagina in je echte browser openen.",
  "tour.blocking.title": "Advertenties worden standaard geblokkeerd",
  "tour.blocking.body":
    "Deze knop laat zien hoeveel filterregels geladen zijn en opent de instellingen die ze beheren. Blokkeren stel je per app in: volledig, alleen netwerk voor een site die cosmetische filtering stukmaakt, of uit.",
  "tour.settings.title": "De rest staat in de instellingen",
  "tour.settings.body":
    "Taal, thema, waar downloads heen gaan, meestarten met het systeem, het systeemvak, filterupdates — en de optionele browserextensie, die Chrome, Edge of Firefox een knop «Deze pagina inpakken» geeft. Daar kun je deze rondleiding opnieuw starten.",
  "settings.about": "Over",
  "about.version": "Versie {version}",
  "about.description":
    "Maakt van een website een bureaubladtoepassing: een eigen venster, een eigen pictogram en een navigatieslot dat hem bij één site houdt. Bouwt op de webview die je systeem al heeft in plaats van een meegeleverde browser-engine — vandaar een paar megabyte.",
  "about.author": "Auteur",
  "about.licence": "Licentie",
  "about.licence.hint":
    "Vrije software. Je mag hem gebruiken, bestuderen, wijzigen en delen; alles wat je ervan gebouwd verspreidt moet onder dezelfde licentie blijven.",
  "about.links": "Links",
  "about.link.downloads": "Downloadpagina",
  "about.link.source": "Broncode",
  "about.link.projects": "Meer projecten",
  "about.link.author": "De auteur op GitHub",
  "about.downloads.hint":
    "Kant-en-klare versies staan alleen op zsync.eu, nooit op GitHub. De GitHub-repository bevat uitsluitend de broncode; een voorgebouwde ZWebWrap daar komt niet van ons.",
  "detail.group": "Groep",
  "detail.group.placeholder": "Werk, Privé…",
  "detail.group.hint":
    "Apps met dezelfde groep staan bij elkaar in je bibliotheek en in het systeemvak. Laat leeg voor geen groep.",
};
