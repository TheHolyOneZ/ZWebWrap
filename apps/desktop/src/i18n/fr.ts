import type { Dictionary } from "./en";

export const fr: Dictionary = {
  "app.tagline": "Une page web en application de bureau. Pas un navigateur.",
  "app.settings": "Réglages",
  "app.blocking.ready": "{count} règles",
  "app.blocking.loading": "Chargement des filtres…",
  "app.blocking.status": "État du blocage des publicités",
  "app.search": "Rechercher dans vos applications",
  "app.empty.title": "Aucune application",
  "app.empty.body":
    "Collez une adresse ci-dessus et ZWebWrap en fait une application de bureau avec sa propre fenêtre et sa propre icône.",
  "app.noMatch": "Rien ne correspond à « {query} »",
  "app.clearSearch": "Effacer la recherche",
  "app.library": "Vos applications",

  "media.title": "Aucune vidéo ne sera lue sur ce système",
  "media.body":
    "Il manque {missing} à GStreamer. Chaque site affichera son lecteur sans jamais rien charger — cela concerne toutes les applications WebKit, pas seulement ZWebWrap.",
  "media.installHint":
    "Installez les greffons manquants, puis vérifiez à nouveau :",
  "media.recheck": "Vérifier à nouveau",
  "media.found":
    "Modules multimédias trouvés — rouvrez vos applications et la vidéo fonctionnera",
  "media.stillMissing": "Toujours manquant : {missing}",

  "add.placeholder": "Collez une adresse — youtube.com, claude.ai…",
  "add.submit": "Créer l’application",
  "add.working": "Création…",
  "add.busy": "Récupération de l’icône et création du raccourci…",
  "add.preview.suffix": "ajoute un raccourci à votre menu d’applications",
  "add.hint":
    "Vous obtenez un raccourci qui ouvre ce site dans sa propre fenêtre. Ni onglets, ni barre d’adresse.",

  "card.open": "Ouvrir {name}",
  "card.settings": "Réglages de {name}",
  "card.blocking": "Bloque",
  "card.blockingOff": "Désactivé",
  "card.blockingOn.title":
    "Le blocage des publicités est actif pour cette application",
  "card.blockingOff.title":
    "Le blocage des publicités est désactivé pour cette application",

  "detail.general": "Général",
  "detail.behaviour": "Comportement",
  "detail.appearance": "Apparence",
  "detail.advanced": "Avancé",
  "detail.sections": "Sections des réglages de l’application",

  "detail.name": "Nom",
  "detail.name.hint":
    "Affiché sur le raccourci et dans le titre de la fenêtre.",
  "detail.address": "Adresse",
  "detail.icon.change": "Changer",
  "detail.icon.saving": "Enregistrement…",
  "detail.icon.reset": "Réinitialiser",
  "detail.icon.reset.title": "Récupérer à nouveau l’icône du site",

  "detail.adblock": "Bloquer publicités et traqueurs",
  "detail.adblock.full": "Complet",
  "detail.adblock.networkOnly": "Réseau uniquement",
  "detail.adblock.off": "Désactivé",
  "detail.adblock.hint.networkOnly":
    "Les requêtes sont toujours filtrées, mais rien n'est injecté dans la page. À utiliser quand masquer les emplacements publicitaires casse un site.",
  "detail.adblock.hint.off":
    "Rien n'est filtré ni masqué pour cette application.",
  "detail.adblock.hint":
    "Filtre les requêtes avant leur départ et masque les emplacements publicitaires restants.",
  "detail.shortcut": "Raccourci sur le bureau",
  "detail.shortcut.checking": "Vérification…",
  "detail.shortcut.hint":
    "Cette application figure toujours dans votre menu d’applications. Ceci est la copie sur le bureau.",
  "detail.fullscreen": "Le plein écran remplit cette fenêtre",
  "detail.fullscreen.hint":
    "Le plein écran d’une vidéo remplit cette fenêtre au lieu de prendre tout l’écran. Désactivez pour un plein écran normal.",
  "detail.frame": "Cadre de la fenêtre",
  "detail.frame.hint":
    "« Sans bordure » demande à votre bureau de retirer la bordure de redimensionnement — beaucoup de thèmes n’en ont déjà pas, l’aspect peut donc être identique. « Sans barre de titre » change toujours quelque chose : déplacer, agrandir et fermer passent dans la barre escamotable.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "Sans bordure",
  "detail.frame.frameless": "Sans barre de titre",

  "detail.offsite": "Quand un lien quitte {host}",
  "detail.offsite.aria": "Quand un lien quitte ce site",
  "detail.offsite.hint":
    "Cette fenêtre est limitée à un seul site. Un lien vers ailleurs s’ouvre dans votre navigateur habituel, ou est suivi ici. Les connexions restent ici dans tous les cas.",
  "detail.offsite.browser": "Envoyer à mon navigateur",
  "detail.offsite.inWindow": "L’ouvrir ici",
  "detail.scope": "Fait aussi partie de cette application",
  "detail.scope.hint":
    "Domaines supplémentaires à ouvrir dans la fenêtre plutôt que dans le navigateur. Séparés par des virgules.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Fait passer le trafic de cette application par un proxy — hôte:port, ou socks5://hôte:port. En définir un donne à cette application sa propre session : les deux moteurs webview rattachent un proxy à la session, pas à la fenêtre.",
  "detail.auth": "Identifiants du site",
  "detail.auth.username": "Nom d'utilisateur",
  "detail.auth.password": "Mot de passe",
  "detail.auth.hint":
    "Pour un site derrière une fenêtre de mot de passe HTTP, comme en utilisent les tableaux de bord d'intranet. Envoyé à la première demande seulement — s'il est refusé, la fenêtre du site apparaît plutôt que de bloquer le compte. Stocké dans credentials.json, dans votre dossier de données, lisible par tout ce qui s'exécute sous votre compte.",
  "detail.zoom": "Zoom",
  "detail.window": "Fenêtre",
  "detail.window.hint": "Mémorisé à la fermeture de la fenêtre.",
  "detail.userAgent": "Identité du navigateur",
  "detail.userAgent.placeholder":
    "Par défaut — l’agent utilisateur propre à WebKit",
  "detail.userAgent.hint":
    "Ce que cette application déclare aux sites. La valeur par défaut est la plus honnête et celle qu’attendent les contrôles anti-robots ; certains sites la refusent et exigent Chrome.",

  "detail.delete": "Supprimer",
  "detail.delete.confirm": "Supprimer cette application et son raccourci ?",
  "detail.cancel": "Annuler",
  "detail.save": "Enregistrer",
  "detail.saving": "Enregistrement…",

  "settings.title": "Réglages",
  "settings.subtitle": "Préférences, blocage des publicités et stockage",
  "settings.sections": "Sections des réglages",
  "settings.general": "Général",
  "settings.blocking": "Blocage des publicités",
  "settings.storage": "Stockage",
  "settings.autosaved":
    "Les modifications sont enregistrées au fur et à mesure",
  "settings.done": "Terminé",

  "settings.startClose": "Démarrage et fermeture",
  "settings.autostart": "Démarrer avec le système",
  "settings.autostart.hint":
    "S’ouvre discrètement dans la zone de notification à la connexion — sans fenêtre.",
  "settings.tray": "Fermer vers la zone de notification",
  "settings.tray.hint":
    "Fermer une application la masque au lieu de quitter : elle revient instantanément et continue là où elle en était. Quittez depuis le menu de la zone de notification.",
  "settings.tray.note":
    "ZWebWrap garde une icône dans la zone de notification tant que l’une de ces options est active. Les deux désactivées, aucun processus ne subsiste.",

  "settings.windows": "Fenêtres",
  "settings.fullscreen": "Le plein écran remplit la fenêtre",
  "settings.fullscreen.hint":
    "Le plein écran d’une vidéo remplit la fenêtre au lieu de prendre tout l’écran. Chaque application peut en décider autrement dans ses propres réglages.",

  "settings.newApps": "Nouvelles applications",
  "settings.desktopShortcut": "Placer les nouveaux raccourcis sur le bureau",
  "settings.desktopShortcut.hint":
    "Ils figurent toujours dans votre menu d’applications. Ceci ajoute en plus une copie sur le bureau. Chaque application peut en décider autrement.",
  "settings.adblockDefault":
    "Bloquer les publicités dans les nouvelles applications",
  "settings.adblockDefault.hint":
    "Les applications déjà créées gardent leur propre réglage.",

  "settings.language": "Langue",
  "settings.language.hint":
    "Suit le système sauf si vous en choisissez une ici.",
  "settings.language.system": "Suivre le système",
  "settings.theme": "Thème",
  "settings.theme.hint": "Suit votre bureau sauf si vous en choisissez un ici.",
  "settings.theme.light": "Clair",
  "settings.theme.dark": "Sombre",

  "settings.metric.rules": "règles réseau",
  "settings.metric.lists": "listes de filtres",
  "settings.metric.updated": "dernière mise à jour",
  "settings.notReady":
    "Le moteur de filtrage charge encore. Les fenêtres ouvertes maintenant ne bloqueront pas les publicités.",
  "settings.seedOnly":
    "Seule la petite liste intégrée est active — les listes complètes ne sont pas encore téléchargées. Elle bloque les requêtes mais ne peut pas masquer les emplacements publicitaires restants.",
  "settings.lists": "Listes de filtres",
  "settings.lists.hint":
    "Téléchargées au premier lancement, puis actualisées seules tous les 4 jours.",
  "settings.update": "Mettre à jour",
  "settings.updating": "Mise à jour…",
  "settings.updated": "Filtres mis à jour — {count} règles",
  "settings.blocked": "Ce qui est bloqué",
  "settings.blocked.body":
    "Les requêtes sont filtrées avant leur départ. Les emplacements publicitaires restants sont masqués par des règles propres au site et génériques, et les publicités vidéo de YouTube sont retirées de la réponse même du lecteur.",
  "settings.dataFolder": "Dossier de données",
  "settings.dataFolder.hint":
    "Applications, icônes, listes de filtres et la session de connexion partagée par toutes les applications.",
  "settings.open": "Ouvrir",

  "app.blocking.preparing": "Préparation des filtres…",
  "app.blocking.basic": "Blocage de base",
  "menu.open": "Ouvrir",
  "menu.settings": "Réglages…",
  "menu.copyAddress": "Copier l’adresse",
  "menu.adblockOff": "Désactiver le blocage des publicités",
  "menu.adblockOn": "Activer le blocage des publicités",
  "menu.delete": "Supprimer…",
  "toast.created": "{name} créée",
  "toast.saved": "{name} enregistrée",
  "toast.deleted": "{name} supprimée",
  "toast.copied": "Adresse copiée",
  "toast.copyFailed": "Impossible de copier l’adresse",
  "confirm.delete.title": "Supprimer {name} ?",
  "confirm.delete.body":
    "Son raccourci et son icône sont supprimés également. Le site lui-même n’est pas touché et vous pouvez l’ajouter de nouveau à tout moment.",
  "ui.dismiss": "Masquer",
  "ui.close": "Fermer",
  "ui.cancel": "Annuler",
  "detail.permissions": "Autorisations",
  "detail.perm.notifications": "Notifications",
  "detail.perm.microphone": "Micro",
  "detail.perm.camera": "Caméra",
  "detail.perm.location": "Position",
  "detail.perm.hint":
    "Ce que cette application peut demander à votre système. « Demander » signifie qu’on vous consulte une fois, puis la réponse est retenue.",
  "rule.allow": "Autoriser",
  "rule.deny": "Bloquer",
  "rule.ask": "Demander",
  "detail.badge": "Afficher les non-lus dans le titre",
  "detail.badge.hint":
    "Reprend dans cette fenêtre le compte que la page met dans son propre titre — (3) Discord, Boîte (12) — pour que la barre des tâches l’affiche.",
  "detail.userCss": "Votre CSS",
  "detail.userCss.hint":
    "Injecté avant le premier rendu. Pratique pour masquer ce que le site refuse de masquer.",
  "detail.userJs": "Votre JavaScript",
  "detail.userJs.hint":
    "S’exécute au démarrage du document avec les droits de la page. Il n’atteint pas ZWebWrap.",
  "detail.hotkey": "Raccourci global",
  "detail.hotkey.hint":
    "Appuyez sur ces touches depuis n’importe où pour ramener cette application au premier plan. Appuyez de nouveau quand elle est devant pour la masquer. Cliquez dans le champ et tapez une combinaison.",
  "detail.hotkey.record": "Cliquez et tapez les touches",
  "detail.hotkey.clear": "Effacer",
  "detail.hotkey.taken":
    "Cette combinaison n’est pas acceptée. Essayez-en une autre.",
  "import.title": "Importer depuis votre navigateur",
  "import.subtitle": "Vos favoris existants, en applications",
  "import.reading": "Lecture de vos navigateurs…",
  "import.none": "Aucun favori qui ne soit déjà une application.",
  "import.search": "Filtrer les favoris",
  "import.pick": "Choisissez ceux que vous voulez",
  "import.chosen": "{n} sélectionnés",
  "import.submit": "Importer {n}",
  "import.working": "Importation…",
  "import.done": "{n} applications importées",
  "app.import": "Importer du navigateur",
  "settings.platform": "Ce système",
  "settings.platform.fine":
    "Tout ce que fait ZWebWrap est disponible sur ce système.",
  "settings.platform.session": "Session",
  "time.never": "jamais",
  "time.justNow": "à l’instant",
  "time.minutes": "il y a {n} min",
  "time.hours": "il y a {n} h",
  "time.days": "il y a {n} j",
  "dl.title": "Téléchargements",
  "dl.empty": "Rien de téléchargé pour l'instant.",
  "dl.clear": "Vider",
  "dl.open": "Ouvrir",
  "dl.reveal": "Afficher dans le dossier",
  "dl.failed": "Échec",
  "dl.running": "{size} pour l'instant",
  "detail.session": "Session de connexion",
  "detail.session.shared": "Partagée",
  "detail.session.isolated": "Séparée",
  "detail.session.hint":
    "Partage une connexion avec tes autres applications : se connecter une fois à Google couvre YouTube, Gmail et Drive.",
  "detail.session.hint.isolated":
    "Cette application se connecte de son côté, ce qui permet d'utiliser un second compte. Elle démarre déconnectée. Effectif à la prochaine ouverture.",
  "settings.downloads": "Téléchargements",
  "settings.askWhereToSave": "Demander où enregistrer chaque fichier",
  "settings.askWhereToSave.hint":
    "Une fenêtre d'application n'a pas de barre de téléchargement : un fichier enregistré en silence passe facilement inaperçu. Désactive pour tout envoyer directement dans ton dossier de téléchargements.",
  "dl.toast.done": "{name} téléchargé",
  "dl.toast.failed": "Impossible de télécharger {name}",
  "settings.metric.hidden": "pubs masquées",
  "settings.tour": "Visite guidée",
  "settings.tour.hint":
    "Affichée une fois sur une installation neuve. Relancez-la quand vous voulez.",
  "settings.tour.button": "Lancer la visite",
  "tour.title": "Une visite rapide",
  "tour.step": "Étape {n} sur {total}",
  "tour.skip": "Passer la visite",
  "tour.back": "Retour",
  "tour.next": "Suivant",
  "tour.done": "Terminer",
  "tour.welcome.title": "Bienvenue dans ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap transforme un site web en application de bureau : sa propre fenêtre, sa propre icône dans la barre des tâches, ni onglets ni barre d'adresse. Deux minutes et vous saurez tout.",
  "tour.add.title": "Ajouter une application",
  "tour.add.body":
    "Collez une adresse ici et appuyez sur Ajouter. ZWebWrap récupère l'icône du site, lui donne une fenêtre et — sauf avis contraire — un raccourci dans votre menu d'applications.",
  "tour.import.title": "Ou importez vos favoris",
  "tour.import.body":
    "L'import lit les favoris déjà présents dans Chrome, Edge, Brave, Vivaldi, Opera ou Firefox et vous laisse choisir lesquels deviennent des applications. Rien n'est envoyé nulle part ; les fichiers sont lus depuis votre propre disque.",
  "tour.library.title": "Vos applications vivent ici",
  "tour.library.body":
    "Cliquez sur une tuile pour l'ouvrir. Le clic droit donne accès à ses réglages, copie son adresse, désactive le blocage des publicités pour ce site ou la supprime. Les applications d'un même groupe sont affichées ensemble, ici comme dans la zone de notification.",
  "tour.toolbar.title": "La barre d'outils cachée",
  "tour.toolbar.body":
    "Une fenêtre encapsulée n'a aucun habillage : amenez le pointeur tout en haut et une petite barre apparaît — précédent, suivant, recharger, rechercher dans la page, zoom, garder au premier plan, téléchargements, et ouvrir cette page dans votre vrai navigateur.",
  "tour.blocking.title": "Les publicités sont bloquées par défaut",
  "tour.blocking.body":
    "Ce bouton indique combien de règles de filtrage sont chargées et ouvre les réglages qui les gèrent. Le blocage se règle par application : complet, réseau seulement pour un site que le filtrage cosmétique casse, ou désactivé.",
  "tour.settings.title": "Tout le reste est dans les réglages",
  "tour.settings.body":
    "Langue, thème, destination des téléchargements, démarrage avec le système, zone de notification, mise à jour des filtres — et l'extension de navigateur optionnelle, qui ajoute un bouton « Encapsuler cette page » à Chrome, Edge ou Firefox. Vous pourrez y relancer cette visite.",
  "settings.about": "À propos",
  "about.version": "Version {version}",
  "about.description":
    "Transforme un site web en application de bureau : sa propre fenêtre, sa propre icône et un verrou de navigation qui la maintient sur un seul site. Elle s'appuie sur le webview déjà présent sur votre système plutôt que sur un moteur de navigateur embarqué, d'où sa taille de quelques mégaoctets.",
  "about.author": "Auteur",
  "about.licence": "Licence",
  "about.licence.hint":
    "Logiciel libre. Vous pouvez l'utiliser, l'étudier, le modifier et le partager ; tout ce que vous distribuez à partir de lui doit rester sous la même licence.",
  "about.links": "Liens",
  "about.link.downloads": "Page de téléchargement",
  "about.link.source": "Code source",
  "about.link.projects": "Autres projets",
  "about.link.author": "L'auteur sur GitHub",
  "about.downloads.hint":
    "Les versions prêtes à l'emploi sont uniquement sur zsync.eu, jamais sur GitHub. Le dépôt GitHub ne contient que le code source : tout ZWebWrap précompilé qu'on y trouverait ne vient pas de nous.",
  "detail.group": "Groupe",
  "detail.group.placeholder": "Travail, Perso…",
  "detail.group.hint":
    "Les applications d'un même groupe sont réunies dans la bibliothèque et dans la zone de notification. Laisse vide pour aucun.",
};
