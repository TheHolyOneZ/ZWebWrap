import type { Dictionary } from "./en";

export const ja: Dictionary = {
  "app.tagline":
    "ウェブページを、デスクトップアプリに。ブラウザではありません。",
  "app.settings": "設定",
  "app.blocking.ready": "{count} ルール",
  "app.blocking.loading": "フィルターを読み込み中…",
  "app.blocking.status": "広告ブロックの状態",
  "app.search": "アプリを検索",
  "app.empty.title": "まだアプリがありません",
  "app.empty.body":
    "上に URL を貼り付けると、ZWebWrap が専用のウィンドウとアイコンを持つデスクトップアプリに変えます。",
  "app.noMatch": "「{query}」に一致するものはありません",
  "app.clearSearch": "検索をクリア",
  "app.library": "アプリ",

  "media.title": "このシステムでは動画を再生できません",
  "media.body":
    "GStreamer に {missing} がありません。どのサイトもプレーヤーは表示しますが、何も読み込まれません。ZWebWrap だけでなく、WebKit を使うすべてのアプリに影響します。",
  "media.installHint":
    "不足しているプラグインを入れてから、もう一度確認してください:",
  "media.recheck": "もう一度確認",
  "media.found":
    "メディアプラグインが見つかりました — アプリを開き直すと動画が再生できます",
  "media.stillMissing": "まだ不足しています: {missing}",

  "add.placeholder": "URL を貼り付け — youtube.com、claude.ai…",
  "add.submit": "アプリを作成",
  "add.working": "作成中…",
  "add.busy": "アイコンを取得してショートカットを作成しています…",
  "add.preview.suffix": "アプリメニューにショートカットを追加します",
  "add.hint":
    "このサイトを専用ウィンドウで開くショートカットができます。タブもアドレスバーもありません。",

  "card.open": "{name} を開く",
  "card.settings": "{name} の設定",
  "card.blocking": "ブロック中",
  "card.blockingOff": "オフ",
  "card.blockingOn.title": "このアプリでは広告ブロックが有効です",
  "card.blockingOff.title": "このアプリでは広告ブロックが無効です",

  "detail.general": "一般",
  "detail.behaviour": "動作",
  "detail.appearance": "外観",
  "detail.advanced": "詳細",
  "detail.sections": "アプリ設定のセクション",

  "detail.name": "名前",
  "detail.name.hint": "ショートカットとウィンドウのタイトルに表示されます。",
  "detail.address": "アドレス",
  "detail.icon.change": "変更",
  "detail.icon.saving": "保存中…",
  "detail.icon.reset": "リセット",
  "detail.icon.reset.title": "サイトのアイコンを取得し直す",

  "detail.adblock": "広告とトラッカーをブロック",
  "detail.adblock.full": "すべて",
  "detail.adblock.networkOnly": "ネットワークのみ",
  "detail.adblock.off": "オフ",
  "detail.adblock.hint.networkOnly":
    "リクエストは引き続き遮断されますが、ページには何も挿入されません。広告枠を隠すとサイトが壊れる場合に使います。",
  "detail.adblock.hint.off": "このアプリでは何も遮断も非表示もされません。",
  "detail.adblock.hint": "リクエストを送信前に遮断し、残った広告枠を隠します。",
  "detail.shortcut": "デスクトップのショートカット",
  "detail.shortcut.checking": "確認中…",
  "detail.shortcut.hint":
    "このアプリは常にアプリメニューにあります。ここで扱うのはデスクトップ上のコピーです。",
  "detail.fullscreen": "全画面をこのウィンドウに収める",
  "detail.fullscreen.hint":
    "動画を全画面にすると、画面全体ではなくこのウィンドウいっぱいに広がります。通常の全画面にするにはオフにしてください。",
  "detail.frame": "ウィンドウの枠",
  "detail.frame.hint":
    "「枠なし」はリサイズ用の縁を省くようデスクトップに要求します。もともと縁のないテーマも多いため、見た目が変わらないことがあります。「タイトルバーなし」は必ず見た目が変わり、移動・最大化・閉じるはホバーツールバーが担います。",
  "detail.frame.normal": "通常",
  "detail.frame.noBorder": "枠なし",
  "detail.frame.frameless": "タイトルバーなし",

  "detail.offsite": "リンクが {host} の外に出たとき",
  "detail.offsite.aria": "リンクがこのサイトの外に出たとき",
  "detail.offsite.hint":
    "このウィンドウは 1 つのサイトに固定されています。他所へのリンクは通常のブラウザで開くか、ここで表示されます。ログインはどちらの場合もここに残ります。",
  "detail.offsite.browser": "ブラウザに渡す",
  "detail.offsite.inWindow": "ここで開く",
  "detail.scope": "このアプリに含めるドメイン",
  "detail.scope.hint":
    "ブラウザではなくウィンドウ内で開くドメインを追加します。カンマ区切り。",

  "detail.proxy": "プロキシ",
  "detail.proxy.hint":
    "このアプリの通信をプロキシ経由にします。host:port または socks5://host:port の形式です。設定するとこのアプリは専用のセッションに移ります。どちらのウェブビューエンジンも、プロキシをウィンドウではなくセッションに結び付けるためです。",
  "detail.auth": "サイトのログイン",
  "detail.auth.username": "ユーザー名",
  "detail.auth.password": "パスワード",
  "detail.auth.hint":
    "社内ダッシュボードなどで使われる HTTP のパスワード入力に応じます。最初の一回だけ送信し、拒否された場合はアカウントを締め出さずサイト自身の入力欄を表示します。データフォルダーの credentials.json に保存され、あなたの権限で動くものからは読み取れます。",
  "detail.zoom": "ズーム",
  "detail.window": "ウィンドウ",
  "detail.window.hint": "ウィンドウを閉じたときに記憶されます。",
  "detail.userAgent": "ブラウザの名乗り",
  "detail.userAgent.placeholder": "既定 — WebKit 自身のユーザーエージェント",
  "detail.userAgent.hint":
    "このアプリがサイトに何と名乗るか。既定がもっとも正直で、ボット判定もそれを想定しています。ただし一部のサイトはこれを拒み、Chrome を要求します。",

  "detail.delete": "削除",
  "detail.delete.confirm": "このアプリとショートカットを削除しますか?",
  "detail.cancel": "キャンセル",
  "detail.save": "保存",
  "detail.saving": "保存中…",

  "settings.title": "設定",
  "settings.subtitle": "環境設定、広告ブロック、保存先",
  "settings.sections": "設定のセクション",
  "settings.general": "一般",
  "settings.blocking": "広告ブロック",
  "settings.storage": "保存先",
  "settings.autosaved": "変更は自動で保存されます",
  "settings.done": "完了",

  "settings.startClose": "起動と終了",
  "settings.autostart": "システムと一緒に起動",
  "settings.autostart.hint":
    "ログイン時にウィンドウを出さず、トレイに静かに常駐します。",
  "settings.tray": "閉じたらトレイへ",
  "settings.tray.hint":
    "アプリを閉じても終了せず隠れるだけなので、すぐに戻り、続きから再生されます。終了はトレイのメニューから。",
  "settings.tray.note":
    "どちらかが有効な間、ZWebWrap はトレイにアイコンを残します。両方とも無効なら、プロセスは一切残りません。",

  "settings.windows": "ウィンドウ",
  "settings.fullscreen": "全画面をウィンドウに収める",
  "settings.fullscreen.hint":
    "動画を全画面にすると、画面全体ではなくウィンドウいっぱいに広がります。アプリごとに個別の設定で上書きできます。",

  "settings.newApps": "新しいアプリ",
  "settings.desktopShortcut": "新しいショートカットをデスクトップに置く",
  "settings.desktopShortcut.hint":
    "アプリメニューには必ず入ります。これはデスクトップにもコピーを置く設定です。アプリごとに上書きできます。",
  "settings.adblockDefault": "新しいアプリで広告をブロック",
  "settings.adblockDefault.hint":
    "作成済みのアプリはそれぞれの設定を保ちます。",

  "settings.language": "言語",
  "settings.language.hint": "ここで選ばない限り、システムに従います。",
  "settings.language.system": "システムに従う",
  "settings.theme": "外観",
  "settings.theme.hint": "ここで選ばないかぎり、システムに従います。",
  "settings.theme.light": "ライト",
  "settings.theme.dark": "ダーク",

  "settings.metric.rules": "ネットワークルール",
  "settings.metric.lists": "フィルターリスト",
  "settings.metric.updated": "最終更新",
  "settings.notReady":
    "フィルターエンジンを読み込み中です。いま開いたウィンドウでは広告はブロックされません。",
  "settings.seedOnly":
    "内蔵の小さなリストだけが有効です。完全なフィルターリストはまだ取得できていません。リクエストは遮断できますが、残った広告枠は隠せません。",
  "settings.lists": "フィルターリスト",
  "settings.lists.hint":
    "初回起動時に取得し、その後は 4 日ごとに自動で更新されます。",
  "settings.update": "今すぐ更新",
  "settings.updating": "更新中…",
  "settings.updated": "フィルターを更新しました — {count} ルール",
  "settings.blocked": "何がブロックされるか",
  "settings.blocked.body":
    "リクエストは送信前に遮断されます。残った広告枠はサイト固有と汎用の両方のルールで隠し、YouTube の動画広告はプレーヤーの応答そのものから取り除きます。",
  "settings.dataFolder": "データフォルダー",
  "settings.dataFolder.hint":
    "アプリ、アイコン、フィルターリスト、そしてすべてのアプリで共有するログインセッション。",
  "settings.open": "開く",

  "app.blocking.preparing": "フィルターを準備中…",
  "app.blocking.basic": "基本のブロック",
  "menu.open": "開く",
  "menu.settings": "設定…",
  "menu.copyAddress": "アドレスをコピー",
  "menu.adblockOff": "広告ブロックをオフにする",
  "menu.adblockOn": "広告ブロックをオンにする",
  "menu.delete": "削除…",
  "toast.created": "{name} を作成しました",
  "toast.saved": "{name} を保存しました",
  "toast.deleted": "{name} を削除しました",
  "toast.copied": "アドレスをコピーしました",
  "toast.copyFailed": "アドレスをコピーできませんでした",
  "confirm.delete.title": "{name} を削除しますか?",
  "confirm.delete.body":
    "デスクトップのショートカットとアイコンも削除されます。サイト自体はそのままで、いつでも追加し直せます。",
  "ui.dismiss": "閉じる",
  "ui.close": "閉じる",
  "ui.cancel": "キャンセル",

  "detail.permissions": "権限",
  "detail.perm.notifications": "通知",
  "detail.perm.microphone": "マイク",
  "detail.perm.camera": "カメラ",
  "detail.perm.location": "位置情報",
  "detail.perm.hint":
    "このアプリがシステムに要求できること。「確認する」は一度だけ尋ね、その答えを記憶します。",
  "rule.allow": "許可",
  "rule.deny": "ブロック",
  "rule.ask": "確認する",

  "detail.badge": "未読件数をタイトルに表示",
  "detail.badge.hint":
    "ページが自身のタイトルに書く件数 —(3) Discord、受信トレイ (12) — をこのウィンドウに反映し、タスクバーに表示させます。",
  "detail.userCss": "あなたの CSS",
  "detail.userCss.hint":
    "ページが描画される前に挿入されます。サイトが隠させてくれないものを隠すのに便利です。",
  "detail.userJs": "あなたの JavaScript",
  "detail.userJs.hint":
    "ドキュメントの開始時に、ページと同じ権限で実行されます。ZWebWrap 自体には手が届きません。",

  "detail.hotkey": "グローバルショートカット",
  "detail.hotkey.hint":
    "システムのどこからでもこのキーを押すと、このアプリが前面に出ます。前面にある状態でもう一度押すと隠れます。欄をクリックして組み合わせを入力してください。",
  "detail.hotkey.record": "クリックしてキーを押す",
  "detail.hotkey.clear": "クリア",
  "detail.hotkey.taken":
    "この組み合わせは使えません。別のものをお試しください。",
  "import.title": "ブラウザから取り込む",
  "import.subtitle": "すでにあるブックマークを、デスクトップアプリに",
  "import.reading": "ブラウザを読み込み中…",
  "import.none": "まだアプリになっていないブックマークは見つかりませんでした。",
  "import.search": "ブックマークを絞り込む",
  "import.pick": "欲しいものを選んでください",
  "import.chosen": "{n} 件選択中",
  "import.submit": "{n} 件を取り込む",
  "import.working": "取り込み中…",
  "import.done": "{n} 件のアプリを取り込みました",
  "app.import": "ブラウザから取り込む",
  "settings.platform": "このシステム",
  "settings.platform.fine":
    "ZWebWrap のすべての機能がこのシステムで利用できます。",
  "settings.platform.session": "セッション",
  "time.never": "なし",
  "time.justNow": "たった今",
  "time.minutes": "{n} 分前",
  "time.hours": "{n} 時間前",
  "time.days": "{n} 日前",
  "dl.title": "ダウンロード",
  "dl.empty": "まだ何もダウンロードしていません。",
  "dl.clear": "消去",
  "dl.open": "開く",
  "dl.reveal": "フォルダーで表示",
  "dl.failed": "失敗",
  "dl.running": "現在 {size}",
  "detail.session": "ログインセッション",
  "detail.session.shared": "共有",
  "detail.session.isolated": "個別",
  "detail.session.hint":
    "他のアプリとログインを共有します。Google に一度ログインすれば YouTube、Gmail、Drive に有効です。",
  "detail.session.hint.isolated":
    "このアプリは個別にログインするため、2 つ目のアカウントを使えます。最初はログアウト状態です。次回開いたときに反映されます。",
  "settings.downloads": "ダウンロード",
  "settings.askWhereToSave": "保存先を毎回確認する",
  "settings.askWhereToSave.hint":
    "アプリのウィンドウにはダウンロードバーがないため、黙って保存されたファイルは見落としがちです。オフにすると、すべてダウンロードフォルダーに直接保存されます。",
  "dl.toast.done": "{name} をダウンロードしました",
  "dl.toast.failed": "{name} をダウンロードできませんでした",
  "settings.metric.hidden": "非表示の広告",
  "settings.tour": "ガイドツアー",
  "settings.tour.hint":
    "新規インストール時に一度だけ表示されます。いつでも見直せます。",
  "settings.tour.button": "ツアーを表示",
  "tour.title": "かんたんなツアー",
  "tour.step": "ステップ {n} / {total}",
  "tour.skip": "ツアーをスキップ",
  "tour.back": "戻る",
  "tour.next": "次へ",
  "tour.done": "完了",
  "tour.welcome.title": "ZWebWrap へようこそ",
  "tour.welcome.body":
    "ZWebWrap はウェブサイトをデスクトップアプリに変えます。専用のウィンドウ、タスクバーの専用アイコン、タブもアドレスバーもありません。2 分ですべて分かります。",
  "tour.add.title": "アプリを追加する",
  "tour.add.body":
    "ここにアドレスを貼り付けて「追加」を押してください。ZWebWrap がサイトのアイコンを取得し、ウィンドウを与え、指定がなければアプリメニューにショートカットも作ります。",
  "tour.import.title": "ブックマークから取り込む",
  "tour.import.body":
    "インポートは Chrome、Edge、Brave、Vivaldi、Opera、Firefox にあるブックマークを読み取り、どれをアプリにするかを選べます。どこにも送信されません。ファイルはあなたのディスクから読むだけです。",
  "tour.library.title": "アプリはここに並びます",
  "tour.library.body":
    "タイルをクリックすると開きます。右クリックで設定、アドレスのコピー、そのサイトの広告ブロックの解除、削除ができます。同じグループのアプリは、ここでもトレイでもまとめて表示されます。",
  "tour.toolbar.title": "隠れたツールバー",
  "tour.toolbar.body":
    "ラップされたウィンドウには操作部分がありません。ポインタを上端に寄せると小さなバーが現れます。戻る、進む、再読み込み、ページ内検索、拡大縮小、最前面、ダウンロード、そして本物のブラウザーで開く。",
  "tour.blocking.title": "広告は既定でブロックされます",
  "tour.blocking.body":
    "このボタンは読み込まれているフィルター規則の数を示し、それを管理する設定を開きます。ブロックはアプリごとに選べます。完全、装飾フィルターで壊れるサイト向けのネットワークのみ、またはオフ。",
  "tour.settings.title": "残りはすべて設定にあります",
  "tour.settings.body":
    "言語、テーマ、ダウンロード先、システム起動時の自動起動、トレイ、フィルターの更新 — そして Chrome、Edge、Firefox に「このページをラップ」ボタンを追加する任意のブラウザー拡張機能。このツアーもそこから再実行できます。",
  "settings.about": "このアプリについて",
  "about.version": "バージョン {version}",
  "about.description":
    "ウェブサイトをデスクトップアプリに変えます。専用のウィンドウ、専用のアイコン、そして一つのサイトから出さないナビゲーションロック。ブラウザーエンジンを同梱せず、システムにすでにある webview を使うので、容量は数メガバイトで済みます。",
  "about.author": "作者",
  "about.licence": "ライセンス",
  "about.licence.hint":
    "自由ソフトウェアです。使用・調査・改変・共有ができ、これを元に作って配布するものは同じライセンスのままである必要があります。",
  "about.links": "リンク",
  "about.link.downloads": "ダウンロードページ",
  "about.link.source": "ソースコード",
  "about.link.projects": "ほかのプロジェクト",
  "about.link.author": "作者の GitHub",
  "about.downloads.hint":
    "ビルド済みの配布物は zsync.eu にのみあります。GitHub には決してありません。GitHub のリポジトリはソースコードだけを置く場所なので、そこで配布されているビルド済みの ZWebWrap は私たちのものではありません。",
  "detail.group": "グループ",
  "detail.group.placeholder": "仕事、プライベート…",
  "detail.group.hint":
    "同じグループのアプリはライブラリとトレイでまとめて表示されます。空欄にするとグループなしです。",
};
