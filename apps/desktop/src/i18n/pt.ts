import type { Dictionary } from "./en";

export const pt: Dictionary = {
  "app.tagline":
    "Uma página web como aplicativo de desktop. Não é um navegador.",
  "app.settings": "Configurações",
  "app.blocking.ready": "{count} regras",
  "app.blocking.loading": "Carregando filtros…",
  "app.blocking.status": "Estado do bloqueio de anúncios",
  "app.search": "Buscar nos seus aplicativos",
  "app.empty.title": "Nenhum aplicativo ainda",
  "app.empty.body":
    "Cole um endereço acima e o ZWebWrap o transforma em um aplicativo de desktop com janela e ícone próprios.",
  "app.noMatch": "Nada corresponde a “{query}”",
  "app.clearSearch": "Limpar a busca",
  "app.library": "As suas apps",

  "media.title": "Nenhum vídeo será reproduzido neste sistema",
  "media.body":
    "Falta {missing} no GStreamer. Todos os sites abertos mostrarão o player, mas nada vai carregar — isso afeta todos os aplicativos WebKit, não só o ZWebWrap.",
  "media.installHint": "Instale os plugins que faltam e verifique de novo:",
  "media.recheck": "Verificar de novo",
  "media.found":
    "Plugins multimédia encontrados — reabra as suas apps e o vídeo funcionará",
  "media.stillMissing": "Continua em falta: {missing}",

  "add.placeholder": "Cole um endereço — youtube.com, claude.ai…",
  "add.submit": "Criar aplicativo",
  "add.working": "Criando…",
  "add.busy": "A obter o ícone e a criar o atalho…",
  "add.preview.suffix": "adiciona um atalho ao menu de aplicações",
  "add.hint":
    "Você ganha um atalho que abre este site na própria janela. Sem abas, sem barra de endereço.",

  "card.open": "Abrir {name}",
  "card.settings": "Configurações de {name}",
  "card.blocking": "Bloqueando",
  "card.blockingOff": "Desligado",
  "card.blockingOn.title": "O bloqueio de anúncios está ativo neste aplicativo",
  "card.blockingOff.title":
    "O bloqueio de anúncios está desligado neste aplicativo",

  "detail.general": "Geral",
  "detail.behaviour": "Comportamento",
  "detail.appearance": "Aparência",
  "detail.advanced": "Avançado",
  "detail.sections": "Seções das configurações do aplicativo",

  "detail.name": "Nome",
  "detail.name.hint": "Aparece no atalho e no título da janela.",
  "detail.address": "Endereço",
  "detail.icon.change": "Alterar",
  "detail.icon.saving": "Salvando…",
  "detail.icon.reset": "Restaurar",
  "detail.icon.reset.title": "Buscar novamente o ícone do site",

  "detail.adblock": "Bloquear anúncios e rastreadores",
  "detail.adblock.full": "Completo",
  "detail.adblock.networkOnly": "Apenas rede",
  "detail.adblock.off": "Desativado",
  "detail.adblock.hint.networkOnly":
    "Os pedidos continuam a ser filtrados, mas nada é injetado na página. Use quando ocultar espaços publicitários estraga um site.",
  "detail.adblock.hint.off": "Nada é filtrado ou ocultado para esta aplicação.",
  "detail.adblock.hint":
    "Filtra as requisições antes que saiam e oculta os espaços de anúncio que sobram.",
  "detail.shortcut": "Atalho na área de trabalho",
  "detail.shortcut.checking": "Verificando…",
  "detail.shortcut.hint":
    "Este aplicativo sempre aparece no menu de aplicativos. Esta é a cópia na área de trabalho.",
  "detail.fullscreen": "A tela cheia preenche esta janela",
  "detail.fullscreen.hint":
    "Colocar um vídeo em tela cheia preenche esta janela em vez de tomar a tela inteira. Desligue para uma tela cheia normal.",
  "detail.frame": "Moldura da janela",
  "detail.frame.hint":
    "“Sem borda” pede ao seu desktop para remover a borda de redimensionamento — muitos temas já não têm, então pode ficar igual. “Sem barra de título” sempre muda: mover, maximizar e fechar passam para a barra flutuante.",
  "detail.frame.normal": "Normal",
  "detail.frame.noBorder": "Sem borda",
  "detail.frame.frameless": "Sem barra de título",

  "detail.offsite": "Quando um link sai de {host}",
  "detail.offsite.aria": "Quando um link sai deste site",
  "detail.offsite.hint":
    "Esta janela está presa a um único site. Um link para qualquer outro abre no seu navegador normal, ou é seguido aqui. Os logins ficam aqui de qualquer forma.",
  "detail.offsite.browser": "Enviar ao meu navegador",
  "detail.offsite.inWindow": "Abrir aqui",
  "detail.scope": "Também faz parte deste aplicativo",
  "detail.scope.hint":
    "Domínios extras que devem abrir na janela em vez do navegador. Separados por vírgula.",

  "detail.proxy": "Proxy",
  "detail.proxy.hint":
    "Encaminha o tráfego desta aplicação por um proxy — anfitrião:porta, ou socks5://anfitrião:porta. Definir um passa esta aplicação a ter sessão própria, porque ambos os motores webview ligam o proxy à sessão e não à janela.",
  "detail.auth": "Início de sessão do site",
  "detail.auth.username": "Utilizador",
  "detail.auth.password": "Palavra-passe",
  "detail.auth.hint":
    "Para um site atrás de uma janela de palavra-passe HTTP, do tipo que os painéis de intranet usam. Só responde ao primeiro pedido — se for recusada, aparece a janela do próprio site em vez de bloquear a conta. Guardada em credentials.json, na tua pasta de dados, legível por tudo o que corra com o teu utilizador.",
  "detail.zoom": "Zoom",
  "detail.window": "Janela",
  "detail.window.hint": "Lembrado quando você fecha a janela.",
  "detail.userAgent": "Identidade do navegador",
  "detail.userAgent.placeholder":
    "Padrão — o agente de usuário do próprio WebKit",
  "detail.userAgent.hint":
    "O que este aplicativo diz ser para os sites. O padrão é o mais honesto e o que as verificações antibô esperam; alguns sites o recusam e exigem o Chrome.",

  "detail.delete": "Excluir",
  "detail.delete.confirm": "Excluir este aplicativo e o atalho dele?",
  "detail.cancel": "Cancelar",
  "detail.save": "Salvar",
  "detail.saving": "Salvando…",

  "settings.title": "Configurações",
  "settings.subtitle": "Preferências, bloqueio de anúncios e armazenamento",
  "settings.sections": "Seções das configurações",
  "settings.general": "Geral",
  "settings.blocking": "Bloqueio de anúncios",
  "settings.storage": "Armazenamento",
  "settings.autosaved": "As alterações são salvas sozinhas",
  "settings.done": "Concluído",

  "settings.startClose": "Início e fechamento",
  "settings.autostart": "Iniciar com o sistema",
  "settings.autostart.hint":
    "Abre discretamente na bandeja quando você faz login — sem janela.",
  "settings.tray": "Fechar para a bandeja",
  "settings.tray.hint":
    "Fechar um aplicativo o esconde em vez de encerrar, então ele volta na hora e continua de onde parou. Encerre pelo menu da bandeja.",
  "settings.tray.note":
    "O ZWebWrap mantém um ícone na bandeja enquanto uma destas estiver ligada. Com as duas desligadas, nenhum processo fica para trás.",

  "settings.windows": "Janelas",
  "settings.fullscreen": "A tela cheia preenche a janela",
  "settings.fullscreen.hint":
    "Colocar um vídeo em tela cheia preenche a janela em vez de tomar a tela inteira. Cada aplicativo pode mudar isso nas próprias configurações.",

  "settings.newApps": "Novos aplicativos",
  "settings.desktopShortcut": "Colocar novos atalhos na área de trabalho",
  "settings.desktopShortcut.hint":
    "Eles sempre aparecem no menu de aplicativos. Isto adiciona também uma cópia na área de trabalho. Cada aplicativo pode mudar isso.",
  "settings.adblockDefault": "Bloquear anúncios em novos aplicativos",
  "settings.adblockDefault.hint":
    "Os aplicativos já criados mantêm a configuração deles.",

  "settings.language": "Idioma",
  "settings.language.hint":
    "Segue o sistema, a menos que você escolha um aqui.",
  "settings.language.system": "Seguir o sistema",
  "settings.theme": "Tema",
  "settings.theme.hint":
    "Segue o seu ambiente de trabalho, a não ser que escolha aqui.",
  "settings.theme.light": "Claro",
  "settings.theme.dark": "Escuro",

  "settings.metric.rules": "regras de rede",
  "settings.metric.lists": "listas de filtros",
  "settings.metric.updated": "última atualização",
  "settings.notReady":
    "O motor de filtros ainda está carregando. As janelas abertas agora não bloquearão anúncios.",
  "settings.seedOnly":
    "Só a pequena lista interna está ativa — as listas completas ainda não foram baixadas. Ela bloqueia as requisições, mas não consegue ocultar os espaços de anúncio que sobram.",
  "settings.lists": "Listas de filtros",
  "settings.lists.hint":
    "Baixadas na primeira execução e depois atualizadas sozinhas a cada 4 dias.",
  "settings.update": "Atualizar agora",
  "settings.updating": "Atualizando…",
  "settings.updated": "Filtros atualizados — {count} regras",
  "settings.blocked": "O que é bloqueado",
  "settings.blocked.body":
    "As requisições são filtradas antes de sair. Os espaços de anúncio que sobram são ocultados com regras específicas do site e genéricas, e os anúncios em vídeo do YouTube são removidos da própria resposta do player.",
  "settings.dataFolder": "Pasta de dados",
  "settings.dataFolder.hint":
    "Aplicativos, ícones, listas de filtros e a sessão de login compartilhada por todos os aplicativos.",
  "settings.open": "Abrir",

  "app.blocking.preparing": "Preparando os filtros…",
  "app.blocking.basic": "Bloqueio básico",
  "menu.open": "Abrir",
  "menu.settings": "Configurações…",
  "menu.copyAddress": "Copiar o endereço",
  "menu.adblockOff": "Desligar o bloqueio de anúncios",
  "menu.adblockOn": "Ligar o bloqueio de anúncios",
  "menu.delete": "Excluir…",
  "toast.created": "{name} criado",
  "toast.saved": "{name} salvo",
  "toast.deleted": "{name} excluído",
  "toast.copied": "Endereço copiado",
  "toast.copyFailed": "Não foi possível copiar o endereço",
  "confirm.delete.title": "Excluir {name}?",
  "confirm.delete.body":
    "O atalho e o ícone também são removidos. O site em si não é tocado e você pode adicioná-lo de novo quando quiser.",
  "ui.dismiss": "Dispensar",
  "ui.close": "Fechar",
  "ui.cancel": "Cancelar",
  "detail.permissions": "Permissões",
  "detail.perm.notifications": "Notificações",
  "detail.perm.microphone": "Microfone",
  "detail.perm.camera": "Câmera",
  "detail.perm.location": "Localização",
  "detail.perm.hint":
    "O que este aplicativo pode pedir ao seu sistema. “Perguntar” significa que você é consultado uma vez e a resposta é lembrada.",
  "rule.allow": "Permitir",
  "rule.deny": "Bloquear",
  "rule.ask": "Perguntar",
  "detail.badge": "Mostrar não lidos no título",
  "detail.badge.hint":
    "Espelha nesta janela a contagem que a página coloca no próprio título — (3) Discord, Caixa (12) — para a barra de tarefas mostrar.",
  "detail.userCss": "Seu CSS",
  "detail.userCss.hint":
    "Injetado antes de a página ser desenhada. Útil para esconder algo que o site não deixa esconder.",
  "detail.userJs": "Seu JavaScript",
  "detail.userJs.hint":
    "Roda no início do documento com os privilégios da página. Não alcança o ZWebWrap.",
  "detail.hotkey": "Atalho global",
  "detail.hotkey.hint":
    "Aperte essas teclas de qualquer lugar do sistema para trazer este aplicativo à frente. Aperte de novo enquanto ele está na frente para escondê-lo. Clique no campo e digite uma combinação.",
  "detail.hotkey.record": "Clique e aperte as teclas",
  "detail.hotkey.clear": "Limpar",
  "detail.hotkey.taken": "Essa combinação não é aceita. Tente outra.",
  "import.title": "Importar do seu navegador",
  "import.subtitle": "Os favoritos que você já tem, como aplicativos",
  "import.reading": "Lendo seus navegadores…",
  "import.none": "Nenhum favorito que já não seja um aplicativo.",
  "import.search": "Filtrar favoritos",
  "import.pick": "Escolha os que quiser",
  "import.chosen": "{n} selecionados",
  "import.submit": "Importar {n}",
  "import.working": "Importando…",
  "import.done": "{n} aplicativos importados",
  "app.import": "Importar do navegador",
  "settings.platform": "Este sistema",
  "settings.platform.fine":
    "Tudo o que o ZWebWrap faz está disponível neste sistema.",
  "settings.platform.session": "Sessão",
  "time.never": "nunca",
  "time.justNow": "agora mesmo",
  "time.minutes": "há {n} min",
  "time.hours": "há {n} h",
  "time.days": "há {n} d",
  "dl.title": "Transferências",
  "dl.empty": "Ainda não transferiu nada.",
  "dl.clear": "Limpar",
  "dl.open": "Abrir",
  "dl.reveal": "Mostrar na pasta",
  "dl.failed": "Falhou",
  "dl.running": "{size} até agora",
  "detail.session": "Sessão de início",
  "detail.session.shared": "Partilhada",
  "detail.session.isolated": "Separada",
  "detail.session.hint":
    "Partilha um início de sessão com as outras aplicações: entrar uma vez no Google cobre o YouTube, o Gmail e o Drive.",
  "detail.session.hint.isolated":
    "Esta aplicação inicia sessão por si própria, para poder usar uma segunda conta. Começa com sessão terminada. Aplica-se da próxima vez que a abrir.",
  "settings.downloads": "Transferências",
  "settings.askWhereToSave": "Perguntar onde guardar cada ficheiro",
  "settings.askWhereToSave.hint":
    "Uma janela de aplicação não tem barra de transferências, por isso um ficheiro guardado em silêncio passa despercebido. Desligue para enviar tudo diretamente para a pasta de transferências.",
  "dl.toast.done": "{name} transferido",
  "dl.toast.failed": "Não foi possível transferir {name}",
  "settings.metric.hidden": "anúncios ocultados",
  "settings.tour": "Visita guiada",
  "settings.tour.hint":
    "Mostrada uma vez numa instalação nova. Podes vê-la de novo quando quiseres.",
  "settings.tour.button": "Ver a visita",
  "tour.title": "Uma visita rápida",
  "tour.step": "Passo {n} de {total}",
  "tour.skip": "Ignorar a visita",
  "tour.back": "Voltar",
  "tour.next": "Seguinte",
  "tour.done": "Concluir",
  "tour.welcome.title": "Bem-vindo ao ZWebWrap",
  "tour.welcome.body":
    "O ZWebWrap transforma um site numa aplicação de ambiente de trabalho: janela própria, ícone próprio na barra de tarefas, sem separadores nem barra de endereço. Dois minutos e ficas a conhecer tudo.",
  "tour.add.title": "Adicionar uma aplicação",
  "tour.add.body":
    "Cola aqui um endereço e carrega em Adicionar. O ZWebWrap vai buscar o ícone do site, dá-lhe uma janela e — a não ser que digas o contrário — um atalho no teu menu de aplicações.",
  "tour.import.title": "Ou traz os teus marcadores",
  "tour.import.body":
    "A importação lê os marcadores que já tens no Chrome, Edge, Brave, Vivaldi, Opera ou Firefox e deixa-te escolher quais se tornam aplicações. Nada é enviado para lado nenhum; os ficheiros são lidos do teu próprio disco.",
  "tour.library.title": "As tuas aplicações estão aqui",
  "tour.library.body":
    "Clica num cartão para o abrir. Com o botão direito abres as definições, copias o endereço, desligas o bloqueio de anúncios para esse site ou apagas a aplicação. Aplicações do mesmo grupo aparecem juntas, aqui e na área de notificação.",
  "tour.toolbar.title": "A barra escondida",
  "tour.toolbar.body":
    "Uma janela encapsulada não tem controlos — leva o ponteiro ao limite superior e aparece uma pequena barra: retroceder, avançar, recarregar, procurar na página, zoom, manter à frente, transferências e abrir esta página no teu browser verdadeiro.",
  "tour.blocking.title": "Os anúncios são bloqueados por omissão",
  "tour.blocking.body":
    "Este botão mostra quantas regras de filtragem estão carregadas e abre as definições que as gerem. O bloqueio é por aplicação: completo, apenas rede para um site que a filtragem cosmética estraga, ou desligado.",
  "tour.settings.title": "Tudo o resto está nas definições",
  "tour.settings.body":
    "Idioma, tema, para onde vão as transferências, arrancar com o sistema, a área de notificação, atualização de filtros — e a extensão de browser opcional, que acrescenta um botão «Encapsular esta página» ao Chrome, Edge ou Firefox. É aí que podes repetir esta visita.",
  "settings.about": "Acerca",
  "about.version": "Versão {version}",
  "about.description":
    "Transforma um site numa aplicação de ambiente de trabalho: janela própria, ícone próprio e um bloqueio de navegação que a mantém num só site. Assenta no webview que o teu sistema já traz em vez de um motor de browser incluído, e por isso ocupa poucos megabytes.",
  "about.author": "Autor",
  "about.licence": "Licença",
  "about.licence.hint":
    "Software livre. Podes usá-lo, estudá-lo, alterá-lo e partilhá-lo; tudo o que distribuíres construído a partir dele tem de continuar sob a mesma licença.",
  "about.links": "Ligações",
  "about.link.downloads": "Página de transferências",
  "about.link.source": "Código-fonte",
  "about.link.projects": "Mais projetos",
  "about.link.author": "O autor no GitHub",
  "about.downloads.hint":
    "As versões já compiladas estão apenas em zsync.eu, nunca no GitHub. O repositório do GitHub contém só o código-fonte, por isso qualquer ZWebWrap pré-compilado que lá apareça não vem de nós.",
  "detail.group": "Grupo",
  "detail.group.placeholder": "Trabalho, Pessoal…",
  "detail.group.hint":
    "As aplicações do mesmo grupo aparecem juntas na biblioteca e na bandeja. Deixe vazio para nenhum.",
};
