import type { Dictionary } from "./en";

export const it: Dictionary = {
  "app.tagline": "Una pagina web come applicazione desktop. Non è un browser.",
  "app.settings": "Impostazioni",
  "app.blocking.ready": "{count} regole",
  "app.blocking.loading": "Caricamento dei filtri…",
  "app.blocking.status": "Stato del blocco pubblicità",
  "app.search": "Cerca nelle tue applicazioni",
  "app.empty.title": "Ancora nessuna applicazione",
  "app.empty.body":
    "Incolla un indirizzo qui sopra e ZWebWrap lo trasforma in un'applicazione desktop con una finestra e un'icona tutte sue.",
  "app.noMatch": "Nessun risultato per “{query}”",
  "app.clearSearch": "Cancella la ricerca",
  "app.library": "Le tue app",

  "media.title": "Su questo sistema i video non verranno riprodotti",
  "media.body":
    "A GStreamer manca {missing}. Ogni sito mostrerà il proprio lettore senza caricare nulla — riguarda tutte le applicazioni WebKit, non solo ZWebWrap.",
  "media.installHint": "Installa i plugin mancanti, poi controlla di nuovo:",
  "media.recheck": "Controlla di nuovo",
  "media.found":
    "Plugin multimediali trovati — riapri le tue app e i video funzioneranno",
  "media.stillMissing": "Ancora mancante: {missing}",

  "add.placeholder": "Incolla un indirizzo — youtube.com, claude.ai…",
  "add.submit": "Crea applicazione",
  "add.working": "Creazione…",
  "add.busy": "Recupero dell’icona e creazione del collegamento…",
  "add.preview.suffix": "aggiunge un collegamento al menu delle app",
  "add.hint":
    "Ottieni un collegamento che apre questo sito in una finestra dedicata. Niente schede, niente barra degli indirizzi.",

  "card.open": "Apri {name}",
  "card.settings": "Impostazioni di {name}",
  "card.blocking": "Blocca",
  "card.blockingOff": "Disattivo",
  "card.blockingOn.title":
    "Il blocco pubblicità è attivo per questa applicazione",
  "card.blockingOff.title":
    "Il blocco pubblicità è disattivato per questa applicazione",

  "detail.general": "Generale",
  "detail.behaviour": "Comportamento",
  "detail.appearance": "Aspetto",
  "detail.advanced": "Avanzate",
  "detail.sections": "Sezioni delle impostazioni dell'applicazione",

  "detail.name": "Nome",
  "detail.name.hint": "Compare sul collegamento e nel titolo della finestra.",
  "detail.address": "Indirizzo",
  "detail.icon.change": "Cambia",
  "detail.icon.saving": "Salvataggio…",
  "detail.icon.reset": "Ripristina",
  "detail.icon.reset.title": "Scarica di nuovo l'icona del sito",

  "detail.adblock": "Blocca pubblicità e tracciatori",
  "detail.adblock.full": "Completo",
  "detail.adblock.networkOnly": "Solo rete",
  "detail.adblock.off": "Disattivato",
  "detail.adblock.hint.networkOnly":
    "Le richieste vengono ancora filtrate, ma non viene inserito nulla nella pagina. Da usare quando nascondere gli spazi pubblicitari rompe un sito.",
  "detail.adblock.hint.off":
    "Per questa app non viene filtrato né nascosto nulla.",
  "detail.adblock.hint":
    "Filtra le richieste prima che partano e nasconde gli spazi pubblicitari rimasti.",
  "detail.shortcut": "Collegamento sul desktop",
  "detail.shortcut.checking": "Controllo…",
  "detail.shortcut.hint":
    "Questa applicazione è sempre nel menu delle applicazioni. Questa è la copia sul desktop.",
  "detail.fullscreen": "Lo schermo intero riempie questa finestra",
  "detail.fullscreen.hint":
    "Mettere un video a schermo intero riempie questa finestra invece di occupare tutto lo schermo. Disattivalo per uno schermo intero normale.",
  "detail.frame": "Cornice della finestra",
  "detail.frame.hint":
    "«Senza bordo» chiede al desktop di togliere il bordo di ridimensionamento — molti temi non ne hanno già, quindi può sembrare identico. «Senza barra del titolo» cambia sempre qualcosa: spostare, ingrandire e chiudere passano alla barra a comparsa.",
  "detail.frame.normal": "Normale",
  "detail.frame.noBorder": "Senza bordo",
  "detail.frame.frameless": "Senza barra del titolo",

  "detail.offsite": "Quando un link esce da {host}",
  "detail.offsite.aria": "Quando un link esce da questo sito",
  "detail.offsite.hint":
    "Questa finestra è vincolata a un solo sito. Un link verso qualsiasi altro si apre nel tuo browser abituale, oppure viene seguito qui. Gli accessi restano qui in ogni caso.",
  "detail.offsite.browser": "Manda al mio browser",
  "detail.offsite.inWindow": "Aprilo qui",
  "detail.scope": "Fa parte anche di questa applicazione",
  "detail.scope.hint":
    "Domini aggiuntivi da aprire nella finestra invece che nel browser. Separati da virgole.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Instrada il traffico di questa app attraverso un proxy — host:porta, oppure socks5://host:porta. Impostarne uno dà a questa app una sessione propria, perché entrambi i motori webview legano il proxy alla sessione e non alla finestra.",
  "detail.auth": "Accesso al sito",
  "detail.auth.username": "Nome utente",
  "detail.auth.password": "Password",
  "detail.auth.hint":
    "Per un sito dietro una finestra di password HTTP, quella usata dai cruscotti intranet. Risponde solo al primo tentativo — se viene rifiutata compare la finestra del sito stesso, invece di bloccare l'account. Salvata in credentials.json nella tua cartella dati, leggibile da qualunque cosa giri con il tuo utente.",
  "detail.zoom": "Zoom",
  "detail.window": "Finestra",
  "detail.window.hint": "Memorizzata alla chiusura della finestra.",
  "detail.userAgent": "Identità del browser",
  "detail.userAgent.placeholder": "Predefinito — lo user agent di WebKit",
  "detail.userAgent.hint":
    "Ciò che questa applicazione dichiara ai siti. Il valore predefinito è il più onesto ed è quello che si aspettano i controlli antibot; alcuni siti lo rifiutano e richiedono Chrome.",

  "detail.delete": "Elimina",
  "detail.delete.confirm":
    "Eliminare questa applicazione e il suo collegamento?",
  "detail.cancel": "Annulla",
  "detail.save": "Salva",
  "detail.saving": "Salvataggio…",

  "settings.title": "Impostazioni",
  "settings.subtitle": "Preferenze, blocco pubblicità e archiviazione",
  "settings.sections": "Sezioni delle impostazioni",
  "settings.general": "Generale",
  "settings.blocking": "Blocco pubblicità",
  "settings.storage": "Archiviazione",
  "settings.autosaved": "Le modifiche si salvano da sole",
  "settings.done": "Fatto",

  "settings.startClose": "Avvio e chiusura",
  "settings.autostart": "Avvia con il sistema",
  "settings.autostart.hint":
    "Si apre discretamente nell'area di notifica quando accedi — senza finestra.",
  "settings.tray": "Chiudi nell'area di notifica",
  "settings.tray.hint":
    "Chiudere un'applicazione la nasconde invece di uscire, così torna subito e continua da dov'era. Esci dal menu dell'area di notifica.",
  "settings.tray.note":
    "ZWebWrap mantiene un'icona nell'area di notifica finché una di queste è attiva. Con entrambe disattivate non lascia alcun processo.",

  "settings.windows": "Finestre",
  "settings.fullscreen": "Lo schermo intero riempie la finestra",
  "settings.fullscreen.hint":
    "Mettere un video a schermo intero riempie la finestra invece di occupare tutto lo schermo. Ogni applicazione può decidere diversamente nelle proprie impostazioni.",

  "settings.newApps": "Nuove applicazioni",
  "settings.desktopShortcut": "Metti i nuovi collegamenti sul desktop",
  "settings.desktopShortcut.hint":
    "Compaiono sempre nel menu delle applicazioni. Questo aggiunge anche una copia sul desktop. Ogni applicazione può decidere diversamente.",
  "settings.adblockDefault": "Blocca la pubblicità nelle nuove applicazioni",
  "settings.adblockDefault.hint":
    "Le applicazioni già create mantengono la propria impostazione.",

  "settings.language": "Lingua",
  "settings.language.hint":
    "Segue il sistema, a meno che tu non ne scelga una qui.",
  "settings.language.system": "Segui il sistema",
  "settings.theme": "Tema",
  "settings.theme.hint": "Segue il tuo desktop finché non ne scegli uno qui.",
  "settings.theme.light": "Chiaro",
  "settings.theme.dark": "Scuro",

  "settings.metric.rules": "regole di rete",
  "settings.metric.lists": "liste di filtri",
  "settings.metric.updated": "ultimo aggiornamento",
  "settings.notReady":
    "Il motore dei filtri sta ancora caricando. Le finestre aperte adesso non bloccheranno la pubblicità.",
  "settings.seedOnly":
    "È attiva solo la piccola lista integrata — le liste complete non sono ancora state scaricate. Blocca le richieste ma non può nascondere gli spazi pubblicitari rimasti.",
  "settings.lists": "Liste di filtri",
  "settings.lists.hint":
    "Scaricate al primo avvio, poi aggiornate da sole ogni 4 giorni.",
  "settings.update": "Aggiorna ora",
  "settings.updating": "Aggiornamento…",
  "settings.updated": "Filtri aggiornati — {count} regole",
  "settings.blocked": "Che cosa viene bloccato",
  "settings.blocked.body":
    "Le richieste vengono filtrate prima di partire. Gli spazi pubblicitari rimasti vengono nascosti con regole specifiche del sito e generiche, e la pubblicità video di YouTube viene rimossa dalla risposta stessa del lettore.",
  "settings.dataFolder": "Cartella dei dati",
  "settings.dataFolder.hint":
    "Applicazioni, icone, liste di filtri e la sessione di accesso condivisa da tutte le applicazioni.",
  "settings.open": "Apri",

  "app.blocking.preparing": "Preparazione dei filtri…",
  "app.blocking.basic": "Blocco di base",
  "menu.open": "Apri",
  "menu.settings": "Impostazioni…",
  "menu.copyAddress": "Copia l'indirizzo",
  "menu.adblockOff": "Disattiva il blocco pubblicità",
  "menu.adblockOn": "Attiva il blocco pubblicità",
  "menu.delete": "Elimina…",
  "toast.created": "{name} creata",
  "toast.saved": "{name} salvata",
  "toast.deleted": "{name} eliminata",
  "toast.copied": "Indirizzo copiato",
  "toast.copyFailed": "Impossibile copiare l'indirizzo",
  "confirm.delete.title": "Eliminare {name}?",
  "confirm.delete.body":
    "Vengono rimossi anche il collegamento sul desktop e l'icona. Il sito non viene toccato e puoi aggiungerlo di nuovo quando vuoi.",
  "ui.dismiss": "Chiudi",
  "ui.close": "Chiudi",
  "ui.cancel": "Annulla",

  "detail.permissions": "Autorizzazioni",
  "detail.perm.notifications": "Notifiche",
  "detail.perm.microphone": "Microfono",
  "detail.perm.camera": "Fotocamera",
  "detail.perm.location": "Posizione",
  "detail.perm.hint":
    "Che cosa questa applicazione può chiedere al sistema. «Chiedi» significa che ti viene chiesto una volta e poi la risposta viene ricordata.",
  "rule.allow": "Consenti",
  "rule.deny": "Blocca",
  "rule.ask": "Chiedi",

  "detail.badge": "Mostra i non letti nel titolo",
  "detail.badge.hint":
    "Riporta in questa finestra il numero che la pagina mette nel proprio titolo — (3) Discord, Posta (12) — così la barra delle applicazioni lo mostra.",
  "detail.userCss": "Il tuo CSS",
  "detail.userCss.hint":
    "Inserito prima che la pagina venga disegnata. Utile per nascondere qualcosa che il sito non ti lascia nascondere.",
  "detail.userJs": "Il tuo JavaScript",
  "detail.userJs.hint":
    "Viene eseguito all'avvio del documento con i privilegi della pagina. Non può raggiungere ZWebWrap.",

  "detail.hotkey": "Scorciatoia globale",
  "detail.hotkey.hint":
    "Premi questi tasti da qualsiasi punto del sistema per portare avanti questa applicazione. Premili di nuovo mentre è in primo piano per nasconderla. Fai clic nel campo e digita una combinazione.",
  "detail.hotkey.record": "Fai clic e premi i tasti",
  "detail.hotkey.clear": "Cancella",
  "detail.hotkey.taken":
    "Questa combinazione non è accettata. Provane un'altra.",
  "import.title": "Importa dal tuo browser",
  "import.subtitle": "I segnalibri che hai già, come applicazioni",
  "import.reading": "Lettura dei browser…",
  "import.none": "Nessun segnalibro che non sia già un'applicazione.",
  "import.search": "Filtra i segnalibri",
  "import.pick": "Scegli quelli che vuoi",
  "import.chosen": "{n} selezionati",
  "import.submit": "Importa {n}",
  "import.working": "Importazione…",
  "import.done": "{n} applicazioni importate",
  "app.import": "Importa dal browser",
  "settings.platform": "Questo sistema",
  "settings.platform.fine":
    "Tutto ciò che fa ZWebWrap è disponibile su questo sistema.",
  "settings.platform.session": "Sessione",
  "time.never": "mai",
  "time.justNow": "proprio ora",
  "time.minutes": "{n} min fa",
  "time.hours": "{n} h fa",
  "time.days": "{n} g fa",
  "dl.title": "Download",
  "dl.empty": "Non hai ancora scaricato nulla.",
  "dl.clear": "Svuota",
  "dl.open": "Apri",
  "dl.reveal": "Mostra nella cartella",
  "dl.failed": "Non riuscito",
  "dl.running": "{size} finora",
  "detail.session": "Sessione di accesso",
  "detail.session.shared": "Condivisa",
  "detail.session.isolated": "Separata",
  "detail.session.hint":
    "Condivide un accesso con le altre app: accedere una volta a Google vale per YouTube, Gmail e Drive.",
  "detail.session.hint.isolated":
    "Questa app accede per conto proprio, così puoi usare un secondo account. Parte disconnessa. Attivo alla prossima apertura.",
  "settings.downloads": "Download",
  "settings.askWhereToSave": "Chiedere dove salvare ogni file",
  "settings.askWhereToSave.hint":
    "Una finestra dell'app non ha una barra dei download, quindi un file salvato in silenzio passa inosservato. Disattiva per mandare tutto direttamente nella cartella Download.",
  "dl.toast.done": "{name} scaricato",
  "dl.toast.failed": "Impossibile scaricare {name}",
  "settings.metric.hidden": "annunci nascosti",
  "settings.tour": "Tour guidato",
  "settings.tour.hint":
    "Mostrato una volta su un'installazione nuova. Puoi rivederlo quando vuoi.",
  "settings.tour.button": "Mostra il tour",
  "tour.title": "Un tour veloce",
  "tour.step": "Passo {n} di {total}",
  "tour.skip": "Salta il tour",
  "tour.back": "Indietro",
  "tour.next": "Avanti",
  "tour.done": "Fine",
  "tour.welcome.title": "Benvenuto in ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap trasforma un sito web in un'applicazione desktop: una finestra propria, un'icona propria nella barra delle applicazioni, niente schede né barra degli indirizzi. Due minuti e saprai tutto.",
  "tour.add.title": "Aggiungi un'app",
  "tour.add.body":
    "Incolla qui un indirizzo e premi Aggiungi. ZWebWrap scarica l'icona del sito, gli dà una finestra e — se non dici altro — un collegamento nel menu delle applicazioni.",
  "tour.import.title": "Oppure porta i tuoi segnalibri",
  "tour.import.body":
    "L'importazione legge i segnalibri già presenti in Chrome, Edge, Brave, Vivaldi, Opera o Firefox e ti lascia scegliere quali diventano app. Non viene inviato nulla da nessuna parte: i file sono letti dal tuo disco.",
  "tour.library.title": "Le tue app stanno qui",
  "tour.library.body":
    "Fai clic su un riquadro per aprirlo. Con il tasto destro apri le sue impostazioni, copi l'indirizzo, disattivi il blocco della pubblicità per quel sito oppure lo elimini. Le app dello stesso gruppo sono mostrate insieme, qui e nell'area di notifica.",
  "tour.toolbar.title": "La barra nascosta",
  "tour.toolbar.body":
    "Una finestra incapsulata non ha comandi: porta il puntatore sul bordo superiore e compare una piccola barra con indietro, avanti, ricarica, trova nella pagina, zoom, sempre in primo piano, download e apri questa pagina nel browser vero.",
  "tour.blocking.title":
    "La pubblicità è bloccata per impostazione predefinita",
  "tour.blocking.body":
    "Questo pulsante mostra quante regole di filtro sono caricate e apre le impostazioni che le gestiscono. Il blocco si sceglie per app: completo, solo rete per un sito che il filtro cosmetico rompe, oppure disattivato.",
  "tour.settings.title": "Tutto il resto è nelle impostazioni",
  "tour.settings.body":
    "Lingua, tema, dove finiscono i download, avvio col sistema, area di notifica, aggiornamento dei filtri — e l'estensione del browser opzionale, che aggiunge un pulsante «Incapsula questa pagina» a Chrome, Edge o Firefox. Da lì puoi rivedere questo tour.",
  "settings.about": "Informazioni",
  "about.version": "Versione {version}",
  "about.description":
    "Trasforma un sito web in un'applicazione desktop: una finestra propria, un'icona propria e un blocco di navigazione che la tiene su un solo sito. Si appoggia al webview che il sistema ha già invece di un motore di browser incluso, ed è per questo che occupa pochi megabyte.",
  "about.author": "Autore",
  "about.licence": "Licenza",
  "about.licence.hint":
    "Software libero. Puoi usarlo, studiarlo, modificarlo e condividerlo; tutto ciò che distribuisci costruito a partire da esso deve restare sotto la stessa licenza.",
  "about.links": "Collegamenti",
  "about.link.downloads": "Pagina di download",
  "about.link.source": "Codice sorgente",
  "about.link.projects": "Altri progetti",
  "about.link.author": "L'autore su GitHub",
  "about.downloads.hint":
    "Le versioni già compilate si trovano solo su zsync.eu, mai su GitHub. Il repository GitHub contiene soltanto il codice sorgente: qualunque ZWebWrap precompilato lì non viene da noi.",
  "detail.group": "Gruppo",
  "detail.group.placeholder": "Lavoro, Personale…",
  "detail.group.hint":
    "Le app dello stesso gruppo appaiono insieme nella libreria e nell'area di notifica. Lascia vuoto per nessuno.",
};
