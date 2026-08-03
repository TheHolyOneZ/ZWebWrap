import type { Dictionary } from "./en";

export const de: Dictionary = {
  "app.tagline": "Eine Webseite als Desktop-App. Kein Browser.",
  "app.settings": "Einstellungen",
  "app.blocking.ready": "{count} Regeln",
  "app.blocking.loading": "Filter werden geladen…",
  "app.blocking.status": "Status der Werbeblockierung",
  "app.search": "Apps durchsuchen",
  "app.empty.title": "Noch keine Apps",
  "app.empty.body":
    "Füge oben eine Adresse ein und ZWebWrap macht daraus eine Desktop-App mit eigenem Fenster und Symbol.",
  "app.noMatch": "Nichts passt zu „{query}“",
  "app.clearSearch": "Suche zurücksetzen",
  "app.library": "Deine Apps",

  "media.title": "Auf diesem System werden keine Videos abgespielt",
  "media.body":
    "GStreamer fehlt {missing}. Jede eingebettete Seite zeigt zwar ihren Player, lädt aber nie — das betrifft alle WebKit-Apps, nicht nur ZWebWrap.",
  "media.installHint": "Installiere die fehlenden Plugins und prüfe es erneut:",
  "media.recheck": "Erneut prüfen",
  "media.found":
    "Medien-Plugins gefunden — öffne deine Apps neu, dann laufen Videos",
  "media.stillMissing": "Fehlt weiterhin: {missing}",

  "add.placeholder": "Adresse einfügen — youtube.com, claude.ai…",
  "add.submit": "App erstellen",
  "add.working": "Wird erstellt…",
  "add.busy": "Symbol wird geholt und die Verknüpfung erstellt…",
  "add.preview.suffix": "fügt eine Verknüpfung zu deinem App-Menü hinzu",
  "add.hint":
    "Du bekommst eine Verknüpfung, die diese Seite in einem eigenen Fenster öffnet. Keine Tabs, keine Adressleiste.",

  "card.open": "{name} öffnen",
  "card.settings": "Einstellungen für {name}",
  "card.blocking": "Blockiert",
  "card.blockingOff": "Aus",
  "card.blockingOn.title": "Werbeblockierung ist für diese App aktiv",
  "card.blockingOff.title": "Werbeblockierung ist für diese App deaktiviert",

  "detail.general": "Allgemein",
  "detail.behaviour": "Verhalten",
  "detail.appearance": "Darstellung",
  "detail.advanced": "Erweitert",
  "detail.sections": "Bereiche der App-Einstellungen",

  "detail.name": "Name",
  "detail.name.hint": "Erscheint auf der Verknüpfung und im Fenstertitel.",
  "detail.address": "Adresse",
  "detail.icon.change": "Ändern",
  "detail.icon.saving": "Wird gespeichert…",
  "detail.icon.reset": "Zurücksetzen",
  "detail.icon.reset.title": "Das Symbol der Seite erneut laden",

  "detail.adblock": "Werbung und Tracker blockieren",
  "detail.adblock.full": "Vollständig",
  "detail.adblock.networkOnly": "Nur Netzwerk",
  "detail.adblock.off": "Aus",
  "detail.adblock.hint.networkOnly":
    "Anfragen werden weiterhin gefiltert, aber nichts wird in die Seite eingefügt. Dafür gedacht, wenn das Ausblenden von Werbeflächen eine Seite kaputtmacht.",
  "detail.adblock.hint.off":
    "Für diese App wird nichts gefiltert oder ausgeblendet.",
  "detail.adblock.hint":
    "Filtert Anfragen, bevor sie das Gerät verlassen, und blendet übrig gebliebene Werbeflächen aus.",
  "detail.shortcut": "Desktop-Verknüpfung",
  "detail.shortcut.checking": "Wird geprüft…",
  "detail.shortcut.hint":
    "Diese App steht immer im Anwendungsmenü. Dies ist die Kopie auf deinem Desktop.",
  "detail.fullscreen": "Vollbild füllt dieses Fenster",
  "detail.fullscreen.hint":
    "Vollbild bei einem Video füllt dieses Fenster, statt den ganzen Bildschirm zu übernehmen. Ausschalten für normales Vollbild.",
  "detail.frame": "Fensterrahmen",
  "detail.frame.hint":
    "„Ohne Rand“ bittet die Arbeitsumgebung, den Größenänderungsrand wegzulassen — viele Themes haben ohnehin keinen, dann sieht es gleich aus. „Ohne Titelleiste“ sieht immer anders aus: Verschieben, Maximieren und Schließen übernimmt dann die eingeblendete Leiste.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "Ohne Rand",
  "detail.frame.frameless": "Ohne Titelleiste",

  "detail.offsite": "Wenn ein Link {host} verlässt",
  "detail.offsite.aria": "Wenn ein Link diese Seite verlässt",
  "detail.offsite.hint":
    "Dieses Fenster ist an eine Seite gebunden. Ein Link auf eine andere Adresse öffnet entweder im normalen Browser oder wird hier geöffnet. Anmeldungen bleiben in jedem Fall hier.",
  "detail.offsite.browser": "An meinen Browser geben",
  "detail.offsite.inWindow": "Hier öffnen",
  "detail.scope": "Gehört ebenfalls zu dieser App",
  "detail.scope.hint":
    "Weitere Domains, die im Fenster statt im Browser öffnen sollen. Mit Komma getrennt.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Leitet den Verkehr dieser App über einen Proxy — host:port oder socks5://host:port. Wird einer gesetzt, bekommt diese App ihre eigene Sitzung, denn beide Webview-Engines hängen einen Proxy an die Anmeldesitzung statt an das Fenster.",
  "detail.auth": "Website-Anmeldung",
  "detail.auth.username": "Benutzername",
  "detail.auth.password": "Passwort",
  "detail.auth.hint":
    "Für eine Seite hinter einem HTTP-Passwortfenster, wie es Intranet-Dashboards nutzen. Wird nur beim ersten Versuch beantwortet — wird es abgelehnt, erscheint das Fenster der Seite selbst, statt das Konto zu sperren. Gespeichert in credentials.json in deinem Datenordner, lesbar für alles, was unter deinem Konto läuft.",
  "detail.zoom": "Zoom",
  "detail.window": "Fenster",
  "detail.window.hint": "Wird beim Schließen des Fensters gemerkt.",
  "detail.userAgent": "Browser-Identität",
  "detail.userAgent.placeholder": "Standard — WebKits eigener User-Agent",
  "detail.userAgent.hint":
    "Als was sich diese App bei Websites ausgibt. Der Standard ist am ehrlichsten und was Bot-Prüfungen erwarten; manche Seiten lehnen ihn ab und brauchen Chrome.",

  "detail.delete": "Löschen",
  "detail.delete.confirm": "Diese App und ihre Verknüpfung löschen?",
  "detail.cancel": "Abbrechen",
  "detail.save": "Speichern",
  "detail.saving": "Wird gespeichert…",

  "settings.title": "Einstellungen",
  "settings.subtitle": "Voreinstellungen, Werbeblockierung und Speicher",
  "settings.sections": "Einstellungsbereiche",
  "settings.general": "Allgemein",
  "settings.blocking": "Werbeblockierung",
  "settings.storage": "Speicher",
  "settings.autosaved": "Änderungen werden sofort gespeichert",
  "settings.done": "Fertig",

  "settings.startClose": "Starten und Schließen",
  "settings.autostart": "Mit dem System starten",
  "settings.autostart.hint":
    "Startet beim Anmelden unauffällig im Systemabschnitt — ohne Fenster.",
  "settings.tray": "Ins Systemabschnitt schließen",
  "settings.tray.hint":
    "Schließen blendet eine App aus, statt sie zu beenden — sie ist sofort wieder da und läuft weiter. Beenden über das Menü im Systemabschnitt.",
  "settings.tray.note":
    "ZWebWrap behält ein Symbol im Systemabschnitt, solange eine dieser Optionen aktiv ist. Sind beide aus, bleibt kein Prozess zurück.",

  "settings.windows": "Fenster",
  "settings.fullscreen": "Vollbild füllt das Fenster",
  "settings.fullscreen.hint":
    "Vollbild bei einem Video füllt das Fenster, statt den ganzen Bildschirm zu übernehmen. Jede App kann das für sich anders einstellen.",

  "settings.newApps": "Neue Apps",
  "settings.desktopShortcut": "Neue Verknüpfungen auf den Desktop legen",
  "settings.desktopShortcut.hint":
    "Sie stehen immer im Anwendungsmenü. Dies legt zusätzlich eine Kopie auf den Desktop. Jede App kann das für sich anders einstellen.",
  "settings.adblockDefault": "Werbung in neuen Apps blockieren",
  "settings.adblockDefault.hint":
    "Bereits erstellte Apps behalten ihre eigene Einstellung.",

  "settings.language": "Sprache",
  "settings.language.hint": "Folgt dem System, sofern hier nichts gewählt ist.",
  "settings.language.system": "Dem System folgen",
  "settings.theme": "Erscheinungsbild",
  "settings.theme.hint": "Folgt deinem System, solange du hier nichts wählst.",
  "settings.theme.light": "Hell",
  "settings.theme.dark": "Dunkel",

  "settings.metric.rules": "Netzwerkregeln",
  "settings.metric.lists": "Filterlisten",
  "settings.metric.updated": "zuletzt aktualisiert",
  "settings.notReady":
    "Die Filter werden noch geladen. Jetzt geöffnete Fenster blockieren keine Werbung.",
  "settings.seedOnly":
    "Nur die kleine eingebaute Liste ist aktiv — die vollständigen Filterlisten wurden noch nicht geladen. Sie blockiert Anfragen, kann aber keine übrig gebliebenen Werbeflächen ausblenden.",
  "settings.lists": "Filterlisten",
  "settings.lists.hint":
    "Beim ersten Start geladen, danach alle 4 Tage automatisch aktualisiert.",
  "settings.update": "Jetzt aktualisieren",
  "settings.updating": "Wird aktualisiert…",
  "settings.updated": "Filter aktualisiert — {count} Regeln",
  "settings.blocked": "Was blockiert wird",
  "settings.blocked.body":
    "Anfragen werden gefiltert, bevor sie das Gerät verlassen. Übrig gebliebene Werbeflächen werden über seitenspezifische und allgemeine Regeln ausgeblendet, und YouTube-Videowerbung wird direkt aus der Antwort des Players entfernt.",
  "settings.dataFolder": "Datenordner",
  "settings.dataFolder.hint":
    "Apps, Symbole, Filterlisten und die Anmeldesitzung, die sich alle Apps teilen.",
  "settings.open": "Öffnen",

  "app.blocking.preparing": "Filter werden vorbereitet…",
  "app.blocking.basic": "Einfache Blockierung",
  "menu.open": "Öffnen",
  "menu.settings": "Einstellungen…",
  "menu.copyAddress": "Adresse kopieren",
  "menu.adblockOff": "Werbeblockierung ausschalten",
  "menu.adblockOn": "Werbeblockierung einschalten",
  "menu.delete": "Löschen…",
  "toast.created": "{name} erstellt",
  "toast.saved": "{name} gespeichert",
  "toast.deleted": "{name} gelöscht",
  "toast.copied": "Adresse kopiert",
  "toast.copyFailed": "Adresse konnte nicht kopiert werden",
  "confirm.delete.title": "{name} löschen?",
  "confirm.delete.body":
    "Die Desktop-Verknüpfung und das Symbol werden ebenfalls entfernt. Die Seite selbst bleibt unberührt und lässt sich jederzeit wieder hinzufügen.",
  "ui.dismiss": "Ausblenden",
  "ui.close": "Schließen",
  "ui.cancel": "Abbrechen",
  "detail.permissions": "Berechtigungen",
  "detail.perm.notifications": "Benachrichtigungen",
  "detail.perm.microphone": "Mikrofon",
  "detail.perm.camera": "Kamera",
  "detail.perm.location": "Standort",
  "detail.perm.hint":
    "Was diese App von deinem System anfordern darf. „Fragen“ heißt: einmal nachfragen, dann die Antwort merken.",
  "rule.allow": "Erlauben",
  "rule.deny": "Blockieren",
  "rule.ask": "Fragen",
  "detail.badge": "Ungelesen-Zähler im Titel anzeigen",
  "detail.badge.hint":
    "Übernimmt einen Zähler, den die Seite in ihren eigenen Titel schreibt — (3) Discord, Posteingang (12) — in dieses Fenster, damit die Taskleiste ihn zeigt.",
  "detail.userCss": "Dein CSS",
  "detail.userCss.hint":
    "Wird vor dem ersten Zeichnen eingefügt. Nützlich, um etwas auszublenden, das die Seite nicht ausblenden lässt.",
  "detail.userJs": "Dein JavaScript",
  "detail.userJs.hint":
    "Läuft beim Dokumentstart mit den Rechten der Seite. Es kommt nicht an ZWebWrap selbst heran.",
  "detail.hotkey": "Globales Tastenkürzel",
  "detail.hotkey.hint":
    "Drücke die Tasten von überall im System, um diese App nach vorn zu holen. Noch einmal drücken, während sie vorn ist, blendet sie wieder aus. Ins Feld klicken und eine Kombination tippen.",
  "detail.hotkey.record": "Klicken und Tasten drücken",
  "detail.hotkey.clear": "Löschen",
  "detail.hotkey.taken":
    "Diese Kombination wird nicht akzeptiert. Probiere eine andere.",
  "import.title": "Aus deinem Browser importieren",
  "import.subtitle": "Lesezeichen, die du schon hast, als Desktop-Apps",
  "import.reading": "Browser werden gelesen…",
  "import.none": "Keine Lesezeichen gefunden, die nicht schon Apps sind.",
  "import.search": "Lesezeichen filtern",
  "import.pick": "Wähle aus, was du möchtest",
  "import.chosen": "{n} ausgewählt",
  "import.submit": "{n} importieren",
  "import.working": "Wird importiert…",
  "import.done": "{n} Apps importiert",
  "app.import": "Aus Browser importieren",
  "settings.platform": "Dieses System",
  "settings.platform.fine":
    "Alles, was ZWebWrap kann, ist auf diesem System verfügbar.",
  "settings.platform.session": "Sitzung",
  "time.never": "nie",
  "time.justNow": "gerade eben",
  "time.minutes": "vor {n} Min.",
  "time.hours": "vor {n} Std.",
  "time.days": "vor {n} Tg.",
  "dl.title": "Downloads",
  "dl.empty": "Noch nichts heruntergeladen.",
  "dl.clear": "Leeren",
  "dl.open": "Öffnen",
  "dl.reveal": "Im Ordner anzeigen",
  "dl.failed": "Fehlgeschlagen",
  "dl.running": "bisher {size}",
  "detail.session": "Anmeldesitzung",
  "detail.session.shared": "Gemeinsam",
  "detail.session.isolated": "Getrennt",
  "detail.session.hint":
    "Teilt eine Anmeldung mit deinen anderen Apps: Einmal bei Google anmelden gilt für YouTube, Gmail und Drive.",
  "detail.session.hint.isolated":
    "Diese App meldet sich eigenständig an, sodass du ein zweites Konto nutzen kannst. Sie startet abgemeldet. Wirkt beim nächsten Öffnen.",
  "settings.downloads": "Downloads",
  "settings.askWhereToSave": "Bei jeder Datei nach dem Speicherort fragen",
  "settings.askWhereToSave.hint":
    "Ein App-Fenster hat keine Download-Leiste, deshalb übersieht man eine still gespeicherte Datei leicht. Ausschalten, um alles direkt in den Download-Ordner zu legen.",
  "dl.toast.done": "{name} heruntergeladen",
  "dl.toast.failed": "{name} konnte nicht heruntergeladen werden",
  "settings.metric.hidden": "Werbung ausgeblendet",
  "settings.tour": "Geführte Tour",
  "settings.tour.hint":
    "Wird bei einer frischen Installation einmal gezeigt. Du kannst sie jederzeit erneut starten.",
  "settings.tour.button": "Tour anzeigen",
  "tour.title": "Eine kurze Tour",
  "tour.step": "Schritt {n} von {total}",
  "tour.skip": "Tour überspringen",
  "tour.back": "Zurück",
  "tour.next": "Weiter",
  "tour.done": "Fertig",
  "tour.welcome.title": "Willkommen bei ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap macht aus einer Website eine Desktop-App: eigenes Fenster, eigenes Symbol in der Taskleiste, keine Tabs und keine Adressleiste. Zwei Minuten, und du kennst das Ganze.",
  "tour.add.title": "Eine App hinzufügen",
  "tour.add.body":
    "Füge hier eine Adresse ein und drücke Hinzufügen. ZWebWrap holt das Symbol der Seite, gibt ihr ein Fenster und – sofern du nichts anderes sagst – eine Verknüpfung im App-Menü.",
  "tour.import.title": "Oder bring deine Lesezeichen mit",
  "tour.import.body":
    "Der Import liest die Lesezeichen, die schon in Chrome, Edge, Brave, Vivaldi, Opera oder Firefox liegen, und du wählst aus, welche zu Apps werden. Nichts wird irgendwohin gesendet; die Dateien werden von deiner eigenen Festplatte gelesen.",
  "tour.library.title": "Hier leben deine Apps",
  "tour.library.body":
    "Klicke auf eine Kachel, um sie zu öffnen. Ein Rechtsklick öffnet ihre Einstellungen, kopiert die Adresse, schaltet den Werbeblocker für diese Seite ab oder löscht sie. Apps mit derselben Gruppe werden zusammen angezeigt – hier und im Tray.",
  "tour.toolbar.title": "Die versteckte Werkzeugleiste",
  "tour.toolbar.body":
    "Ein eingepacktes Fenster hat keine Bedienleiste – fahre mit dem Zeiger an den oberen Rand, und eine kleine Leiste erscheint: zurück, vorwärts, neu laden, Suche auf der Seite, Zoom, immer im Vordergrund, Downloads und diese Seite im echten Browser öffnen.",
  "tour.blocking.title": "Werbung wird standardmäßig blockiert",
  "tour.blocking.body":
    "Diese Schaltfläche zeigt, wie viele Filterregeln geladen sind, und öffnet die zugehörigen Einstellungen. Blockieren wird pro App eingestellt: vollständig, nur Netzwerk für eine Seite, die kosmetische Filter kaputt machen, oder aus.",
  "tour.settings.title": "Alles Weitere steht in den Einstellungen",
  "tour.settings.body":
    "Sprache, Design, Speicherort für Downloads, Start mit dem System, Tray, Filter-Updates – und die optionale Browser-Erweiterung, die Chrome, Edge oder Firefox eine Schaltfläche „Diese Seite einpacken“ hinzufügt. Dort kannst du diese Tour erneut starten.",
  "settings.about": "Über",
  "about.version": "Version {version}",
  "about.description":
    "Macht aus einer Website eine Desktop-App: eigenes Fenster, eigenes Symbol und eine Navigationssperre, die sie bei einer Seite hält. Baut auf der Webview auf, die dein System schon mitbringt, statt auf einer mitgelieferten Browser-Engine — deshalb sind es nur ein paar Megabyte.",
  "about.author": "Autor",
  "about.licence": "Lizenz",
  "about.licence.hint":
    "Freie Software. Du darfst sie nutzen, untersuchen, ändern und weitergeben; alles, was du daraus gebaut weitergibst, muss unter derselben Lizenz bleiben.",
  "about.links": "Links",
  "about.link.downloads": "Download-Seite",
  "about.link.source": "Quellcode",
  "about.link.projects": "Weitere Projekte",
  "about.link.author": "Der Autor auf GitHub",
  "about.downloads.hint":
    "Fertige Versionen gibt es nur auf zsync.eu — nie auf GitHub. Das GitHub-Repository enthält allein den Quellcode; was dort ein fertiges ZWebWrap anbietet, stammt nicht von uns.",
  "detail.group": "Gruppe",
  "detail.group.placeholder": "Arbeit, Privat…",
  "detail.group.hint":
    "Apps mit derselben Gruppe erscheinen in der Bibliothek und im Tray zusammen. Leer lassen für keine.",
};
