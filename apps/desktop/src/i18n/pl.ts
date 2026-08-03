import type { Dictionary } from "./en";

export const pl: Dictionary = {
  "app.tagline":
    "Strona internetowa jako program na pulpicie. To nie przeglądarka.",
  "app.settings": "Ustawienia",
  "app.blocking.ready": "{count} reguł",
  "app.blocking.loading": "Wczytywanie filtrów…",
  "app.blocking.status": "Stan blokowania reklam",
  "app.search": "Szukaj w swoich aplikacjach",
  "app.empty.title": "Nie ma jeszcze aplikacji",
  "app.empty.body":
    "Wklej adres powyżej, a ZWebWrap zrobi z niego program na pulpicie z własnym oknem i ikoną.",
  "app.noMatch": "Nic nie pasuje do „{query}”",
  "app.clearSearch": "Wyczyść wyszukiwanie",
  "app.library": "Twoje aplikacje",

  "media.title": "Na tym systemie wideo się nie odtworzy",
  "media.body":
    "GStreamerowi brakuje {missing}. Każda strona pokaże swój odtwarzacz, ale nic się nie wczyta — dotyczy to wszystkich programów opartych na WebKit, nie tylko ZWebWrap.",
  "media.installHint": "Zainstaluj brakujące wtyczki i sprawdź ponownie:",
  "media.recheck": "Sprawdź ponownie",
  "media.found":
    "Znaleziono wtyczki multimedialne — otwórz aplikacje ponownie, a wideo zadziała",
  "media.stillMissing": "Nadal brakuje: {missing}",

  "add.placeholder": "Wklej adres — youtube.com, claude.ai…",
  "add.submit": "Utwórz aplikację",
  "add.working": "Tworzenie…",
  "add.busy": "Pobieranie ikony i tworzenie skrótu…",
  "add.preview.suffix": "doda skrót do menu aplikacji",
  "add.hint":
    "Dostajesz skrót, który otwiera tę stronę we własnym oknie. Bez kart, bez paska adresu.",

  "card.open": "Otwórz {name}",
  "card.settings": "Ustawienia {name}",
  "card.blocking": "Blokuje",
  "card.blockingOff": "Wyłączone",
  "card.blockingOn.title": "Blokowanie reklam jest włączone dla tej aplikacji",
  "card.blockingOff.title":
    "Blokowanie reklam jest wyłączone dla tej aplikacji",

  "detail.general": "Ogólne",
  "detail.behaviour": "Zachowanie",
  "detail.appearance": "Wygląd",
  "detail.advanced": "Zaawansowane",
  "detail.sections": "Sekcje ustawień aplikacji",

  "detail.name": "Nazwa",
  "detail.name.hint": "Widoczna na skrócie i w tytule okna.",
  "detail.address": "Adres",
  "detail.icon.change": "Zmień",
  "detail.icon.saving": "Zapisywanie…",
  "detail.icon.reset": "Przywróć",
  "detail.icon.reset.title": "Pobierz ikonę strony ponownie",

  "detail.adblock": "Blokuj reklamy i śledzenie",
  "detail.adblock.full": "Pełne",
  "detail.adblock.networkOnly": "Tylko sieć",
  "detail.adblock.off": "Wyłączone",
  "detail.adblock.hint.networkOnly":
    "Żądania są nadal filtrowane, ale nic nie jest wstrzykiwane do strony. Użyj, gdy ukrywanie miejsc reklamowych psuje witrynę.",
  "detail.adblock.hint.off":
    "Dla tej aplikacji nic nie jest filtrowane ani ukrywane.",
  "detail.adblock.hint":
    "Filtruje żądania, zanim wyjdą, i ukrywa pozostałe miejsca reklamowe.",
  "detail.shortcut": "Skrót na pulpicie",
  "detail.shortcut.checking": "Sprawdzanie…",
  "detail.shortcut.hint":
    "Ta aplikacja zawsze jest w menu programów. To jest kopia na pulpicie.",
  "detail.fullscreen": "Pełny ekran wypełnia to okno",
  "detail.fullscreen.hint":
    "Włączenie pełnego ekranu na filmie wypełnia to okno, zamiast zająć cały ekran. Wyłącz, aby mieć zwykły pełny ekran.",
  "detail.frame": "Ramka okna",
  "detail.frame.hint":
    "„Bez obramowania” prosi środowisko, by pominęło krawędź do zmiany rozmiaru — wiele motywów i tak jej nie ma, więc może wyglądać identycznie. „Bez paska tytułu” zawsze wygląda inaczej: przenoszenie, maksymalizacja i zamykanie przechodzą na wysuwany pasek.",
  "detail.frame.normal": "Zwykła",
  "detail.frame.noBorder": "Bez obramowania",
  "detail.frame.frameless": "Bez paska tytułu",

  "detail.offsite": "Gdy odnośnik opuszcza {host}",
  "detail.offsite.aria": "Gdy odnośnik opuszcza tę stronę",
  "detail.offsite.hint":
    "To okno jest przypisane do jednej strony. Odnośnik do czegokolwiek innego otwiera się w twojej zwykłej przeglądarce albo zostaje otwarty tutaj. Logowania i tak zostają tutaj.",
  "detail.offsite.browser": "Wyślij do mojej przeglądarki",
  "detail.offsite.inWindow": "Otwórz tutaj",
  "detail.scope": "Też należy do tej aplikacji",
  "detail.scope.hint":
    "Dodatkowe domeny, które mają się otwierać w oknie zamiast w przeglądarce. Oddzielone przecinkami.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Przepuszcza ruch tej aplikacji przez proxy — host:port albo socks5://host:port. Ustawienie proxy przenosi aplikację na własną sesję, ponieważ oba silniki webview wiążą proxy z sesją, a nie z oknem.",
  "detail.auth": "Logowanie do witryny",
  "detail.auth.username": "Nazwa użytkownika",
  "detail.auth.password": "Hasło",
  "detail.auth.hint":
    "Dla witryny za okienkiem hasła HTTP, jakiego używają panele w intranecie. Wysyłane tylko przy pierwszym pytaniu — po odrzuceniu pojawia się okienko samej witryny, zamiast blokować konto. Przechowywane w credentials.json w twoim folderze danych, czytelne dla wszystkiego, co działa na twoim koncie.",
  "detail.zoom": "Powiększenie",
  "detail.window": "Okno",
  "detail.window.hint": "Zapamiętywane przy zamykaniu okna.",
  "detail.userAgent": "Tożsamość przeglądarki",
  "detail.userAgent.placeholder": "Domyślnie — własny user agent WebKit",
  "detail.userAgent.hint":
    "Za co ta aplikacja podaje się stronom. Wartość domyślna jest najuczciwsza i tego oczekują zabezpieczenia przed botami; niektóre strony ją odrzucają i wymagają Chrome.",

  "detail.delete": "Usuń",
  "detail.delete.confirm": "Usunąć tę aplikację i jej skrót?",
  "detail.cancel": "Anuluj",
  "detail.save": "Zapisz",
  "detail.saving": "Zapisywanie…",

  "settings.title": "Ustawienia",
  "settings.subtitle": "Preferencje, blokowanie reklam i dane",
  "settings.sections": "Sekcje ustawień",
  "settings.general": "Ogólne",
  "settings.blocking": "Blokowanie reklam",
  "settings.storage": "Dane",
  "settings.autosaved": "Zmiany zapisują się same",
  "settings.done": "Gotowe",

  "settings.startClose": "Uruchamianie i zamykanie",
  "settings.autostart": "Uruchamiaj z systemem",
  "settings.autostart.hint":
    "Po zalogowaniu otwiera się dyskretnie w zasobniku — bez okna.",
  "settings.tray": "Zamykaj do zasobnika",
  "settings.tray.hint":
    "Zamknięcie aplikacji ukrywa ją zamiast kończyć, więc wraca natychmiast i gra dalej w tym samym miejscu. Zakończ z menu w zasobniku.",
  "settings.tray.note":
    "ZWebWrap trzyma ikonę w zasobniku, dopóki któraś z tych opcji jest włączona. Przy obu wyłączonych nie zostaje żaden proces.",

  "settings.windows": "Okna",
  "settings.fullscreen": "Pełny ekran wypełnia okno",
  "settings.fullscreen.hint":
    "Włączenie pełnego ekranu na filmie wypełnia okno, zamiast zająć cały ekran. Każda aplikacja może to ustawić po swojemu.",

  "settings.newApps": "Nowe aplikacje",
  "settings.desktopShortcut": "Umieszczaj nowe skróty na pulpicie",
  "settings.desktopShortcut.hint":
    "Zawsze pojawiają się w menu programów. To dokłada jeszcze kopię na pulpicie. Każda aplikacja może to ustawić po swojemu.",
  "settings.adblockDefault": "Blokuj reklamy w nowych aplikacjach",
  "settings.adblockDefault.hint":
    "Aplikacje, które już utworzyłeś, zachowują własne ustawienie.",

  "settings.language": "Język",
  "settings.language.hint": "Idzie za systemem, chyba że wybierzesz coś tutaj.",
  "settings.language.system": "Zgodnie z systemem",
  "settings.theme": "Motyw",
  "settings.theme.hint": "Podąża za systemem, dopóki nie wybierzesz tutaj.",
  "settings.theme.light": "Jasny",
  "settings.theme.dark": "Ciemny",

  "settings.metric.rules": "reguł sieciowych",
  "settings.metric.lists": "list filtrów",
  "settings.metric.updated": "ostatnia aktualizacja",
  "settings.notReady":
    "Silnik filtrów jeszcze się wczytuje. Okna otwarte teraz nie będą blokować reklam.",
  "settings.seedOnly":
    "Działa tylko mała lista wbudowana — pełne listy filtrów jeszcze się nie pobrały. Blokuje żądania, ale nie ukryje pozostałych miejsc reklamowych.",
  "settings.lists": "Listy filtrów",
  "settings.lists.hint":
    "Pobierane przy pierwszym uruchomieniu, potem odświeżane same co 4 dni.",
  "settings.update": "Zaktualizuj teraz",
  "settings.updating": "Aktualizowanie…",
  "settings.updated": "Filtry zaktualizowane — {count} reguł",
  "settings.blocked": "Co jest blokowane",
  "settings.blocked.body":
    "Żądania są filtrowane, zanim wyjdą. Pozostałe miejsca reklamowe są ukrywane regułami ogólnymi i tymi dla konkretnej strony, a reklamy wideo w YouTube są usuwane z samej odpowiedzi odtwarzacza.",
  "settings.dataFolder": "Folder danych",
  "settings.dataFolder.hint":
    "Aplikacje, ikony, listy filtrów i sesja logowania wspólna dla wszystkich aplikacji.",
  "settings.open": "Otwórz",

  "app.blocking.preparing": "Przygotowywanie filtrów…",
  "app.blocking.basic": "Podstawowe blokowanie",
  "menu.open": "Otwórz",
  "menu.settings": "Ustawienia…",
  "menu.copyAddress": "Kopiuj adres",
  "menu.adblockOff": "Wyłącz blokowanie reklam",
  "menu.adblockOn": "Włącz blokowanie reklam",
  "menu.delete": "Usuń…",
  "toast.created": "Utworzono {name}",
  "toast.saved": "Zapisano {name}",
  "toast.deleted": "Usunięto {name}",
  "toast.copied": "Adres skopiowany",
  "toast.copyFailed": "Nie udało się skopiować adresu",
  "confirm.delete.title": "Usunąć {name}?",
  "confirm.delete.body":
    "Skrót na pulpicie i ikona też znikną. Sama strona pozostaje nietknięta i możesz ją dodać ponownie w każdej chwili.",
  "ui.dismiss": "Zamknij",
  "ui.close": "Zamknij",
  "ui.cancel": "Anuluj",

  "detail.permissions": "Uprawnienia",
  "detail.perm.notifications": "Powiadomienia",
  "detail.perm.microphone": "Mikrofon",
  "detail.perm.camera": "Kamera",
  "detail.perm.location": "Lokalizacja",
  "detail.perm.hint":
    "O co ta aplikacja może poprosić twój system. „Pytaj” znaczy, że zostaniesz zapytany raz, a odpowiedź zostanie zapamiętana.",
  "rule.allow": "Zezwól",
  "rule.deny": "Blokuj",
  "rule.ask": "Pytaj",

  "detail.badge": "Pokazuj liczbę nieprzeczytanych w tytule",
  "detail.badge.hint":
    "Przenosi do tego okna liczbę, którą strona wpisuje we własny tytuł — (3) Discord, Odebrane (12) — żeby pokazał ją pasek zadań.",
  "detail.userCss": "Twój CSS",
  "detail.userCss.hint":
    "Wstrzykiwany, zanim strona się narysuje. Przydatny, by ukryć coś, czego strona ukryć nie pozwala.",
  "detail.userJs": "Twój JavaScript",
  "detail.userJs.hint":
    "Uruchamia się na starcie dokumentu z uprawnieniami strony. Nie sięgnie do samego ZWebWrap.",

  "detail.hotkey": "Skrót globalny",
  "detail.hotkey.hint":
    "Naciśnij te klawisze w dowolnym miejscu systemu, aby przywołać tę aplikację. Naciśnij ponownie, gdy jest na wierzchu, aby ją schować. Kliknij pole i wpisz kombinację.",
  "detail.hotkey.record": "Kliknij i naciśnij klawisze",
  "detail.hotkey.clear": "Wyczyść",
  "detail.hotkey.taken": "Ta kombinacja nie jest przyjmowana. Spróbuj innej.",
  "import.title": "Importuj z przeglądarki",
  "import.subtitle": "Zakładki, które już masz, jako aplikacje",
  "import.reading": "Czytanie przeglądarek…",
  "import.none": "Nie znaleziono zakładek, które nie są już aplikacjami.",
  "import.search": "Filtruj zakładki",
  "import.pick": "Wybierz te, które chcesz",
  "import.chosen": "Wybrano {n}",
  "import.submit": "Importuj {n}",
  "import.working": "Importowanie…",
  "import.done": "Zaimportowano {n} aplikacji",
  "app.import": "Importuj z przeglądarki",
  "settings.platform": "Ten system",
  "settings.platform.fine":
    "Wszystko, co robi ZWebWrap, jest dostępne w tym systemie.",
  "settings.platform.session": "Sesja",
  "time.never": "nigdy",
  "time.justNow": "przed chwilą",
  "time.minutes": "{n} min temu",
  "time.hours": "{n} godz. temu",
  "time.days": "{n} dni temu",
  "dl.title": "Pobrane",
  "dl.empty": "Nic jeszcze nie pobrano.",
  "dl.clear": "Wyczyść",
  "dl.open": "Otwórz",
  "dl.reveal": "Pokaż w folderze",
  "dl.failed": "Niepowodzenie",
  "dl.running": "dotąd {size}",
  "detail.session": "Sesja logowania",
  "detail.session.shared": "Wspólna",
  "detail.session.isolated": "Osobna",
  "detail.session.hint":
    "Współdzieli jedno logowanie z innymi aplikacjami: jedno logowanie do Google obejmuje YouTube, Gmail i Dysk.",
  "detail.session.hint.isolated":
    "Ta aplikacja loguje się samodzielnie, więc możesz użyć drugiego konta. Startuje wylogowana. Zadziała przy następnym otwarciu.",
  "settings.downloads": "Pobrane",
  "settings.askWhereToSave": "Pytaj, gdzie zapisać każdy plik",
  "settings.askWhereToSave.hint":
    "Okno aplikacji nie ma paska pobierania, więc po cichu zapisany plik łatwo przeoczyć. Wyłącz, aby wszystko trafiało prosto do folderu Pobrane.",
  "dl.toast.done": "Pobrano {name}",
  "dl.toast.failed": "Nie udało się pobrać {name}",
  "settings.metric.hidden": "ukrytych reklam",
  "settings.tour": "Przewodnik",
  "settings.tour.hint":
    "Pokazywany raz przy nowej instalacji. Możesz go uruchomić ponownie w dowolnej chwili.",
  "settings.tour.button": "Pokaż przewodnik",
  "tour.title": "Krótki przewodnik",
  "tour.step": "Krok {n} z {total}",
  "tour.skip": "Pomiń przewodnik",
  "tour.back": "Wstecz",
  "tour.next": "Dalej",
  "tour.done": "Zakończ",
  "tour.welcome.title": "Witamy w ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap zamienia stronę internetową w aplikację na pulpit: własne okno, własna ikona na pasku zadań, żadnych kart ani paska adresu. Dwie minuty i poznasz całość.",
  "tour.add.title": "Dodaj aplikację",
  "tour.add.body":
    "Wklej tutaj adres i naciśnij Dodaj. ZWebWrap pobierze ikonę strony, da jej okno i — o ile nie powiesz inaczej — skrót w menu aplikacji.",
  "tour.import.title": "Albo przenieś swoje zakładki",
  "tour.import.body":
    "Import czyta zakładki z Chrome, Edge, Brave, Vivaldi, Opery lub Firefoksa i pozwala wybrać, które mają stać się aplikacjami. Nic nigdzie nie jest wysyłane; pliki są czytane z twojego dysku.",
  "tour.library.title": "Tutaj mieszkają twoje aplikacje",
  "tour.library.body":
    "Kliknij kafelek, aby go otworzyć. Prawy przycisk otwiera ustawienia, kopiuje adres, wyłącza blokowanie reklam dla danej strony albo ją usuwa. Aplikacje z tą samą grupą są pokazywane razem — tutaj i w zasobniku.",
  "tour.toolbar.title": "Ukryty pasek narzędzi",
  "tour.toolbar.body":
    "Opakowane okno nie ma żadnych przycisków — przesuń wskaźnik na samą górę, a pojawi się mały pasek: wstecz, dalej, odśwież, znajdź na stronie, powiększenie, zawsze na wierzchu, pobrane pliki i otwarcie tej strony w prawdziwej przeglądarce.",
  "tour.blocking.title": "Reklamy są blokowane domyślnie",
  "tour.blocking.body":
    "Ten przycisk pokazuje, ile reguł filtrowania jest wczytanych, i otwiera zarządzające nimi ustawienia. Blokowanie ustawia się osobno dla każdej aplikacji: pełne, tylko sieć dla strony, którą psuje filtrowanie kosmetyczne, albo wyłączone.",
  "tour.settings.title": "Cała reszta jest w ustawieniach",
  "tour.settings.body":
    "Język, motyw, miejsce pobierania, uruchamianie z systemem, zasobnik, aktualizacje filtrów — oraz opcjonalne rozszerzenie przeglądarki, które dodaje przycisk „Opakuj tę stronę” w Chrome, Edge lub Firefoksie. Stamtąd możesz też ponownie uruchomić ten przewodnik.",
  "settings.about": "O programie",
  "about.version": "Wersja {version}",
  "about.description":
    "Zamienia stronę internetową w aplikację na pulpit: własne okno, własna ikona i blokada nawigacji trzymająca ją przy jednej witrynie. Opiera się na webview, który system już ma, zamiast na dołączonym silniku przeglądarki — stąd kilka megabajtów.",
  "about.author": "Autor",
  "about.licence": "Licencja",
  "about.licence.hint":
    "Wolne oprogramowanie. Możesz go używać, badać, zmieniać i udostępniać; wszystko, co rozpowszechniasz zbudowane na jego podstawie, musi pozostać na tej samej licencji.",
  "about.links": "Odnośniki",
  "about.link.downloads": "Strona pobierania",
  "about.link.source": "Kod źródłowy",
  "about.link.projects": "Więcej projektów",
  "about.link.author": "Autor na GitHubie",
  "about.downloads.hint":
    "Gotowe wersje są tylko na zsync.eu — nigdy na GitHubie. Repozytorium GitHub zawiera wyłącznie kod źródłowy, więc gotowy ZWebWrap oferowany tam nie pochodzi od nas.",
  "detail.group": "Grupa",
  "detail.group.placeholder": "Praca, Prywatne…",
  "detail.group.hint":
    "Aplikacje z tą samą grupą są pokazywane razem w bibliotece i w zasobniku. Zostaw puste, aby nie grupować.",
};
