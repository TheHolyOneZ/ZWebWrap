import type { Dictionary } from "./en";

export const es: Dictionary = {
  "app.tagline":
    "Una página web como aplicación de escritorio. No es un navegador.",
  "app.settings": "Ajustes",
  "app.blocking.ready": "{count} reglas",
  "app.blocking.loading": "Cargando filtros…",
  "app.blocking.status": "Estado del bloqueo de anuncios",
  "app.search": "Buscar en tus aplicaciones",
  "app.empty.title": "Aún no hay aplicaciones",
  "app.empty.body":
    "Pega una dirección arriba y ZWebWrap la convierte en una aplicación de escritorio con su propia ventana e icono.",
  "app.noMatch": "Nada coincide con «{query}»",
  "app.clearSearch": "Borrar la búsqueda",
  "app.library": "Tus aplicaciones",

  "media.title": "En este sistema no se reproducirá vídeo",
  "media.body":
    "Falta {missing} en GStreamer. Cada sitio abierto mostrará su reproductor pero nunca cargará nada — afecta a todas las aplicaciones WebKit, no solo a ZWebWrap.",
  "media.installHint":
    "Instala los complementos que faltan y vuelve a comprobarlo:",
  "media.recheck": "Comprobar de nuevo",
  "media.found":
    "Complementos multimedia encontrados: vuelve a abrir tus apps y el vídeo funcionará",
  "media.stillMissing": "Sigue faltando: {missing}",

  "add.placeholder": "Pega una dirección — youtube.com, claude.ai…",
  "add.submit": "Crear aplicación",
  "add.working": "Creando…",
  "add.busy": "Obteniendo el icono y creando el acceso directo…",
  "add.preview.suffix": "añade un acceso directo a tu menú de aplicaciones",
  "add.hint":
    "Obtienes un acceso directo que abre este sitio en su propia ventana. Sin pestañas ni barra de direcciones.",

  "card.open": "Abrir {name}",
  "card.settings": "Ajustes de {name}",
  "card.blocking": "Bloqueando",
  "card.blockingOff": "Desactivado",
  "card.blockingOn.title":
    "El bloqueo de anuncios está activo en esta aplicación",
  "card.blockingOff.title":
    "El bloqueo de anuncios está desactivado en esta aplicación",

  "detail.general": "General",
  "detail.behaviour": "Comportamiento",
  "detail.appearance": "Apariencia",
  "detail.advanced": "Avanzado",
  "detail.sections": "Secciones de ajustes de la aplicación",

  "detail.name": "Nombre",
  "detail.name.hint":
    "Aparece en el acceso directo y en el título de la ventana.",
  "detail.address": "Dirección",
  "detail.icon.change": "Cambiar",
  "detail.icon.saving": "Guardando…",
  "detail.icon.reset": "Restablecer",
  "detail.icon.reset.title": "Volver a obtener el icono del sitio",

  "detail.adblock": "Bloquear anuncios y rastreadores",
  "detail.adblock.full": "Completo",
  "detail.adblock.networkOnly": "Solo red",
  "detail.adblock.off": "Desactivado",
  "detail.adblock.hint.networkOnly":
    "Las solicitudes se siguen filtrando, pero no se inyecta nada en la página. Úsalo cuando ocultar los huecos publicitarios rompe un sitio.",
  "detail.adblock.hint.off":
    "Para esta aplicación no se filtra ni se oculta nada.",
  "detail.adblock.hint":
    "Filtra las peticiones antes de que salgan y oculta los huecos de anuncios que queden.",
  "detail.shortcut": "Acceso directo en el escritorio",
  "detail.shortcut.checking": "Comprobando…",
  "detail.shortcut.hint":
    "Esta aplicación siempre aparece en tu menú de aplicaciones. Esta es la copia del escritorio.",
  "detail.fullscreen": "La pantalla completa llena esta ventana",
  "detail.fullscreen.hint":
    "Al poner un vídeo en pantalla completa se llena esta ventana en lugar de toda la pantalla. Desactívalo para una pantalla completa normal.",
  "detail.frame": "Marco de la ventana",
  "detail.frame.hint":
    "«Sin borde» pide al escritorio que quite el borde de redimensionado — muchos temas ya no lo tienen, así que puede verse igual. «Sin barra de título» siempre se ve distinto: mover, maximizar y cerrar pasan a la barra emergente.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "Sin borde",
  "detail.frame.frameless": "Sin barra de título",

  "detail.offsite": "Cuando un enlace sale de {host}",
  "detail.offsite.aria": "Cuando un enlace sale de este sitio",
  "detail.offsite.hint":
    "Esta ventana está limitada a un solo sitio. Un enlace a cualquier otro se abre en tu navegador habitual, o se sigue aquí. Los inicios de sesión se quedan aquí en cualquier caso.",
  "detail.offsite.browser": "Enviar a mi navegador",
  "detail.offsite.inWindow": "Seguirlo aquí",
  "detail.scope": "También forma parte de esta aplicación",
  "detail.scope.hint":
    "Dominios adicionales que deben abrirse en la ventana en lugar del navegador. Separados por comas.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Envía el tráfico de esta aplicación a través de un proxy — host:puerto, o socks5://host:puerto. Al poner uno, esta aplicación pasa a tener su propia sesión, porque ambos motores webview asocian el proxy a la sesión y no a la ventana.",
  "detail.auth": "Inicio de sesión del sitio",
  "detail.auth.username": "Usuario",
  "detail.auth.password": "Contraseña",
  "detail.auth.hint":
    "Para un sitio detrás de una ventana de contraseña HTTP, la que usan los paneles de intranet. Se responde solo en el primer intento — si se rechaza, aparece la ventana del propio sitio en lugar de bloquear la cuenta. Se guarda en credentials.json, en tu carpeta de datos, legible por cualquier cosa que se ejecute como tú.",
  "detail.zoom": "Zoom",
  "detail.window": "Ventana",
  "detail.window.hint": "Se recuerda al cerrar la ventana.",
  "detail.userAgent": "Identidad del navegador",
  "detail.userAgent.placeholder":
    "Predeterminado — el agente de usuario propio de WebKit",
  "detail.userAgent.hint":
    "Lo que esta aplicación dice ser a los sitios web. El valor predeterminado es el más honesto y el que esperan las comprobaciones antibots; algunos sitios lo rechazan y necesitan Chrome.",

  "detail.delete": "Eliminar",
  "detail.delete.confirm": "¿Eliminar esta aplicación y su acceso directo?",
  "detail.cancel": "Cancelar",
  "detail.save": "Guardar",
  "detail.saving": "Guardando…",

  "settings.title": "Ajustes",
  "settings.subtitle": "Preferencias, bloqueo de anuncios y almacenamiento",
  "settings.sections": "Secciones de ajustes",
  "settings.general": "General",
  "settings.blocking": "Bloqueo de anuncios",
  "settings.storage": "Almacenamiento",
  "settings.autosaved": "Los cambios se guardan solos",
  "settings.done": "Listo",

  "settings.startClose": "Inicio y cierre",
  "settings.autostart": "Iniciar con el sistema",
  "settings.autostart.hint":
    "Se abre discretamente en la bandeja al iniciar sesión — sin ventana.",
  "settings.tray": "Cerrar a la bandeja",
  "settings.tray.hint":
    "Cerrar una aplicación la oculta en vez de salir, así vuelve al instante y sigue reproduciendo donde estaba. Salir desde el menú de la bandeja.",
  "settings.tray.note":
    "ZWebWrap mantiene un icono en la bandeja mientras alguna de estas esté activa. Con ambas desactivadas no deja ningún proceso.",

  "settings.windows": "Ventanas",
  "settings.fullscreen": "La pantalla completa llena la ventana",
  "settings.fullscreen.hint":
    "Al poner un vídeo en pantalla completa se llena la ventana en lugar de toda la pantalla. Cada aplicación puede cambiarlo en sus propios ajustes.",

  "settings.newApps": "Aplicaciones nuevas",
  "settings.desktopShortcut":
    "Poner los accesos directos nuevos en el escritorio",
  "settings.desktopShortcut.hint":
    "Siempre aparecen en tu menú de aplicaciones. Esto añade además una copia al escritorio. Cada aplicación puede cambiarlo en sus propios ajustes.",
  "settings.adblockDefault": "Bloquear anuncios en las aplicaciones nuevas",
  "settings.adblockDefault.hint":
    "Las aplicaciones que ya creaste conservan su propio ajuste.",

  "settings.language": "Idioma",
  "settings.language.hint": "Sigue al sistema salvo que elijas uno aquí.",
  "settings.language.system": "Seguir al sistema",
  "settings.theme": "Tema",
  "settings.theme.hint": "Sigue tu escritorio salvo que elijas uno aquí.",
  "settings.theme.light": "Claro",
  "settings.theme.dark": "Oscuro",

  "settings.metric.rules": "reglas de red",
  "settings.metric.lists": "listas de filtros",
  "settings.metric.updated": "última actualización",
  "settings.notReady":
    "El motor de filtrado aún se está cargando. Las ventanas que abras ahora no bloquearán anuncios.",
  "settings.seedOnly":
    "Solo está activa la pequeña lista integrada — las listas completas todavía no se han descargado. Bloquea las peticiones, pero no puede ocultar los huecos de anuncios que queden.",
  "settings.lists": "Listas de filtros",
  "settings.lists.hint":
    "Se descargan en el primer arranque y luego se actualizan solas cada 4 días.",
  "settings.update": "Actualizar ahora",
  "settings.updating": "Actualizando…",
  "settings.updated": "Filtros actualizados — {count} reglas",
  "settings.blocked": "Qué se bloquea",
  "settings.blocked.body":
    "Las peticiones se filtran antes de salir. Los huecos de anuncios que queden se ocultan con reglas específicas del sitio y genéricas, y los anuncios de vídeo de YouTube se eliminan de la propia respuesta del reproductor.",
  "settings.dataFolder": "Carpeta de datos",
  "settings.dataFolder.hint":
    "Aplicaciones, iconos, listas de filtros y la sesión de inicio compartida por todas las aplicaciones.",
  "settings.open": "Abrir",

  "app.blocking.preparing": "Preparando los filtros…",
  "app.blocking.basic": "Bloqueo básico",
  "menu.open": "Abrir",
  "menu.settings": "Ajustes…",
  "menu.copyAddress": "Copiar la dirección",
  "menu.adblockOff": "Desactivar el bloqueo de anuncios",
  "menu.adblockOn": "Activar el bloqueo de anuncios",
  "menu.delete": "Eliminar…",
  "toast.created": "{name} creada",
  "toast.saved": "{name} guardada",
  "toast.deleted": "{name} eliminada",
  "toast.copied": "Dirección copiada",
  "toast.copyFailed": "No se pudo copiar la dirección",
  "confirm.delete.title": "¿Eliminar {name}?",
  "confirm.delete.body":
    "También se eliminan su acceso directo y su icono. El sitio en sí no se toca y puedes añadirlo de nuevo cuando quieras.",
  "ui.dismiss": "Descartar",
  "ui.close": "Cerrar",
  "ui.cancel": "Cancelar",
  "detail.permissions": "Permisos",
  "detail.perm.notifications": "Notificaciones",
  "detail.perm.microphone": "Micrófono",
  "detail.perm.camera": "Cámara",
  "detail.perm.location": "Ubicación",
  "detail.perm.hint":
    "Lo que esta aplicación puede pedirle a tu sistema. «Preguntar» significa que se te consulta una vez y luego se recuerda la respuesta.",
  "rule.allow": "Permitir",
  "rule.deny": "Bloquear",
  "rule.ask": "Preguntar",
  "detail.badge": "Mostrar los no leídos en el título",
  "detail.badge.hint":
    "Refleja en esta ventana el número que la página pone en su propio título — (3) Discord, Bandeja (12) — para que la barra de tareas lo muestre.",
  "detail.userCss": "Tu CSS",
  "detail.userCss.hint":
    "Se inyecta antes de que la página se dibuje. Útil para ocultar algo que el sitio no te deja ocultar.",
  "detail.userJs": "Tu JavaScript",
  "detail.userJs.hint":
    "Se ejecuta al inicio del documento con los permisos de la página. No puede alcanzar a ZWebWrap.",
  "detail.hotkey": "Atajo global",
  "detail.hotkey.hint":
    "Pulsa esas teclas desde cualquier parte del sistema para traer esta aplicación al frente. Púlsalas otra vez mientras está delante para ocultarla. Haz clic en el campo y teclea una combinación.",
  "detail.hotkey.record": "Haz clic y pulsa teclas",
  "detail.hotkey.clear": "Borrar",
  "detail.hotkey.taken": "Esa combinación no se acepta. Prueba con otra.",
  "import.title": "Importar desde tu navegador",
  "import.subtitle": "Los marcadores que ya tienes, como aplicaciones",
  "import.reading": "Leyendo tus navegadores…",
  "import.none": "No hay marcadores que no sean ya aplicaciones.",
  "import.search": "Filtrar marcadores",
  "import.pick": "Elige los que quieras",
  "import.chosen": "{n} seleccionados",
  "import.submit": "Importar {n}",
  "import.working": "Importando…",
  "import.done": "{n} aplicaciones importadas",
  "app.import": "Importar del navegador",
  "settings.platform": "Este sistema",
  "settings.platform.fine":
    "Todo lo que hace ZWebWrap está disponible en este sistema.",
  "settings.platform.session": "Sesión",
  "time.never": "nunca",
  "time.justNow": "ahora mismo",
  "time.minutes": "hace {n} min",
  "time.hours": "hace {n} h",
  "time.days": "hace {n} d",
  "dl.title": "Descargas",
  "dl.empty": "Todavía no has descargado nada.",
  "dl.clear": "Vaciar",
  "dl.open": "Abrir",
  "dl.reveal": "Mostrar en la carpeta",
  "dl.failed": "Error",
  "dl.running": "{size} hasta ahora",
  "detail.session": "Sesión de inicio",
  "detail.session.shared": "Compartida",
  "detail.session.isolated": "Separada",
  "detail.session.hint":
    "Comparte un inicio de sesión con tus otras aplicaciones: entrar en Google una vez cubre YouTube, Gmail y Drive.",
  "detail.session.hint.isolated":
    "Esta aplicación inicia sesión por su cuenta, así que puedes usar una segunda cuenta. Empieza sin sesión. Se aplica la próxima vez que la abras.",
  "settings.downloads": "Descargas",
  "settings.askWhereToSave": "Preguntar dónde guardar cada archivo",
  "settings.askWhereToSave.hint":
    "Una ventana envuelta no tiene barra de descargas, así que un archivo que se guarda solo pasa desapercibido. Desactívalo para enviar todo directamente a tu carpeta de descargas.",
  "dl.toast.done": "{name} descargado",
  "dl.toast.failed": "No se pudo descargar {name}",
  "settings.metric.hidden": "anuncios ocultos",
  "settings.tour": "Visita guiada",
  "settings.tour.hint":
    "Se muestra una vez en una instalación nueva. Puedes repetirla cuando quieras.",
  "settings.tour.button": "Ver la visita",
  "tour.title": "Una visita rápida",
  "tour.step": "Paso {n} de {total}",
  "tour.skip": "Omitir la visita",
  "tour.back": "Atrás",
  "tour.next": "Siguiente",
  "tour.done": "Finalizar",
  "tour.welcome.title": "Te damos la bienvenida a ZWebWrap",
  "tour.welcome.body":
    "ZWebWrap convierte un sitio web en una aplicación de escritorio: su propia ventana, su propio icono en la barra de tareas y ni pestañas ni barra de direcciones. En dos minutos lo conocerás entero.",
  "tour.add.title": "Añade una aplicación",
  "tour.add.body":
    "Pega aquí una dirección y pulsa Añadir. ZWebWrap descarga el icono del sitio, le da una ventana y —salvo que digas lo contrario— un acceso directo en tu menú de aplicaciones.",
  "tour.import.title": "O trae tus marcadores",
  "tour.import.body":
    "Importar lee los marcadores que ya tienes en Chrome, Edge, Brave, Vivaldi, Opera o Firefox y te deja elegir cuáles se convierten en aplicaciones. No se envía nada a ninguna parte; los archivos se leen de tu propio disco.",
  "tour.library.title": "Aquí viven tus aplicaciones",
  "tour.library.body":
    "Haz clic en una ficha para abrirla. Con el botón derecho accedes a sus ajustes, copias su dirección, desactivas el bloqueo de anuncios para ese sitio o la eliminas. Las aplicaciones del mismo grupo se muestran juntas, aquí y en la bandeja.",
  "tour.toolbar.title": "La barra oculta",
  "tour.toolbar.body":
    "Una ventana envuelta no tiene controles: lleva el puntero al borde superior y aparecerá una pequeña barra con atrás, adelante, recargar, buscar en la página, zoom, mantener encima, descargas y abrir esta página en tu navegador de verdad.",
  "tour.blocking.title": "Los anuncios se bloquean por defecto",
  "tour.blocking.body":
    "Este botón muestra cuántas reglas de filtrado hay cargadas y abre los ajustes que las gestionan. El bloqueo se decide por aplicación: completo, solo red para un sitio que el filtrado cosmético estropea, o desactivado.",
  "tour.settings.title": "Todo lo demás está en los ajustes",
  "tour.settings.body":
    "Idioma, tema, dónde van las descargas, iniciar con el sistema, la bandeja, la actualización de filtros — y la extensión opcional del navegador, que añade un botón «Envolver esta página» a Chrome, Edge o Firefox. Desde ahí puedes repetir esta visita.",
  "settings.about": "Acerca de",
  "about.version": "Versión {version}",
  "about.description":
    "Convierte un sitio web en una aplicación de escritorio: su propia ventana, su propio icono y un bloqueo de navegación que la mantiene en un solo sitio. Se apoya en el webview que tu sistema ya incluye en lugar de un motor de navegador empaquetado, y por eso ocupa unos pocos megabytes.",
  "about.author": "Autor",
  "about.licence": "Licencia",
  "about.licence.hint":
    "Software libre. Puedes usarlo, estudiarlo, modificarlo y compartirlo; todo lo que distribuyas construido a partir de él debe seguir con la misma licencia.",
  "about.links": "Enlaces",
  "about.link.downloads": "Página de descargas",
  "about.link.source": "Código fuente",
  "about.link.projects": "Más proyectos",
  "about.link.author": "El autor en GitHub",
  "about.downloads.hint":
    "Las versiones ya compiladas están solo en zsync.eu, nunca en GitHub. El repositorio de GitHub contiene únicamente el código fuente, así que cualquier ZWebWrap precompilado que aparezca allí no viene de nosotros.",
  "detail.group": "Grupo",
  "detail.group.placeholder": "Trabajo, Personal…",
  "detail.group.hint":
    "Las aplicaciones del mismo grupo aparecen juntas en tu biblioteca y en la bandeja. Déjalo vacío para ninguno.",
};
