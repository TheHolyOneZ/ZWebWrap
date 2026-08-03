pub const TOOLBAR: &str = r#"
(function () {
  'use strict';
  if (window.__zwwBar || window.top !== window.self) return;
  window.__zwwBar = true;

  var invoke = null;
  function call(action, value) {
    if (!invoke) {
      invoke =
        (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) ||
        (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke);
    }
    if (!invoke) return Promise.reject(new Error('no bridge'));
    return invoke('zww_window_action', { action: action, value: value });
  }

  function build() {
    if (!document.body) return setTimeout(build, 60);

    var host = document.createElement('div');
    host.id = '__zww_bar_host';
    host.style.cssText =
      'position:fixed;top:0;left:0;right:0;height:0;z-index:2147483647;' +
      'pointer-events:none;color-scheme:dark';
    var root = host.attachShadow({ mode: 'closed' });

    var HTML =
      '<style>' +
      ':host{all:initial}' +
      '.zone{position:fixed;top:0;left:0;right:0;height:8px;pointer-events:auto}' +
      '.rail{position:fixed;top:0;left:0;right:0;display:flex;justify-content:center;' +
      'pointer-events:none}' +
      '.bar{transform:translateY(-140%);' +
      'display:flex;align-items:center;gap:6px;padding:6px 8px;pointer-events:auto;' +


      'background:linear-gradient(180deg,rgba(24,28,37,.82),rgba(16,19,26,.84));' +
      'border:1px solid rgba(255,255,255,.10);' +
      'border-top:none;border-radius:0 0 14px 14px;' +


      'box-shadow:0 14px 38px -12px rgba(0,0,0,.8),' +
      'inset 0 -1px 0 rgba(255,255,255,.05);' +
      '-webkit-backdrop-filter:blur(14px) saturate(140%);' +
      'backdrop-filter:blur(14px) saturate(140%);' +
      'font:12px/1.2 system-ui,-apple-system,"Segoe UI",Roboto,sans-serif;' +


      'accent-color:#5d7cfa;' +
      'transition:transform .22s cubic-bezier(.22,.68,.24,1),opacity .18s;opacity:0}' +
      '.bar.show{transform:translateY(0);opacity:1}' +


      'button{all:unset;display:grid;place-items:center;width:30px;height:30px;' +
      'border-radius:8px;cursor:pointer;color:#c8cede}' +
      'button{transition:background .14s ease,color .14s ease,transform .12s ' +
      'cubic-bezier(.34,1.56,.64,1)}' +
      'button:hover{background:rgba(255,255,255,.10);color:#fff}' +

      'button:active{transform:scale(.88);transition-duration:.06s}' +
      'button.on{background:#5d7cfa;color:#fff;' +
      'box-shadow:0 4px 12px -4px rgba(93,124,250,.65)}' +
      'button:disabled{opacity:.35;cursor:default;transform:none}' +


      '@keyframes zspin{from{transform:rotate(0)}to{transform:rotate(360deg)}}' +
      '@keyframes zback{30%{transform:translateX(-4px)}100%{transform:translateX(0)}}' +
      '@keyframes zfwd{30%{transform:translateX(4px)}100%{transform:translateX(0)}}' +
      '@keyframes zpop{40%{transform:scale(1.35)}100%{transform:scale(1)}}' +
      '@keyframes zlift{35%{transform:translateY(-4px)}100%{transform:translateY(0)}}' +
      '@keyframes zfly{35%{transform:translate(4px,-4px);opacity:.5}' +
      '70%{transform:translate(-3px,3px);opacity:1}100%{transform:translate(0,0)}}' +
      '@keyframes zshake{25%{transform:translateX(-2px)}75%{transform:translateX(2px)}' +
      '100%{transform:translateX(0)}}' +
      '.a-spin svg{animation:zspin .55s cubic-bezier(.4,0,.2,1)}' +
      '.a-back svg{animation:zback .35s cubic-bezier(.34,1.56,.64,1)}' +
      '.a-fwd svg{animation:zfwd .35s cubic-bezier(.34,1.56,.64,1)}' +
      '.a-pop svg{animation:zpop .34s cubic-bezier(.34,1.56,.64,1)}' +
      '.a-lift svg{animation:zlift .34s cubic-bezier(.34,1.56,.64,1)}' +
      '.a-fly svg{animation:zfly .45s cubic-bezier(.4,0,.2,1)}' +
      '.a-shake svg{animation:zshake .3s ease}' +
      '.a-loading svg{animation:zspin .9s linear infinite}' +


      '@media (prefers-reduced-motion:reduce){' +
      'button,button:active{transition:background .1s ease,color .1s ease;transform:none}' +
      '.a-spin svg,.a-back svg,.a-fwd svg,.a-pop svg,.a-lift svg,.a-fly svg,' +
      '.a-shake svg,.a-loading svg{animation:none}}' +
      'svg{width:16px;height:16px;fill:none;stroke:currentColor;stroke-width:2.25;' +
      'stroke-linecap:round;stroke-linejoin:round}' +


      '.seg{display:flex;align-items:center;gap:2px;padding:3px;' +
      'border-radius:11px;background:rgba(255,255,255,.055)}' +


      '.zoom{display:flex;align-items:center;gap:8px;padding:3px 11px 3px 9px;' +
      'border-radius:11px;background:rgba(255,255,255,.055);color:#98a0b3}' +


      '.grip{width:1px;align-self:stretch;margin:4px 4px;' +
      'background:rgba(255,255,255,.09);cursor:move}' +

      '.dlwrap{position:relative;display:none}' +
      '.dlwrap.show{display:flex}' +
      '.dlpop{position:absolute;top:100%;right:0;opacity:0;pointer-events:none;' +
      'transform:scale(.94);transform-origin:top right;' +
      'transition:opacity .16s,transform .16s cubic-bezier(.22,.68,.24,1)}' +
      '.dlpop.show{opacity:1;pointer-events:auto;transform:scale(1)}' +


      '.dlpanel{margin-top:6px;min-width:240px;max-width:320px;max-height:240px;' +
      'overflow-y:auto;scrollbar-gutter:stable;padding:6px;background:rgba(20,24,32,.97);' +
      'border:1px solid rgba(255,255,255,.10);border-radius:12px;' +
      'box-shadow:0 12px 30px -10px rgba(0,0,0,.8),inset 0 1px 0 rgba(255,255,255,.05);' +
      '-webkit-backdrop-filter:blur(14px);backdrop-filter:blur(14px)}' +
      '.dlpanel::-webkit-scrollbar{width:10px}' +
      '.dlpanel::-webkit-scrollbar-track{background:transparent}' +
      '.dlpanel::-webkit-scrollbar-thumb{background:rgba(255,255,255,.16);' +
      'background-clip:padding-box;border:3px solid transparent;border-radius:999px}' +
      '.dlpanel::-webkit-scrollbar-thumb:hover{background:rgba(255,255,255,.28);' +
      'background-clip:padding-box}' +
      '.dlrow{display:flex;align-items:center;gap:6px;padding:5px 6px;border-radius:8px}' +
      '.dlrow:hover{background:rgba(255,255,255,.07)}' +
      '.dlname{flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;' +
      'white-space:nowrap;color:#e9edf5;font-size:12px}' +
      '.dlmeta{color:#98a0b3;font-size:10.5px;font-variant-numeric:tabular-nums}' +
      '.dlrow.failed .dlmeta{color:#f2615c}' +
      '.dlact{all:unset;display:grid;place-items:center;width:24px;height:24px;' +
      'border-radius:6px;cursor:pointer;color:#c8cede}' +
      '.dlact:hover{background:rgba(255,255,255,.12);color:#fff}' +
      '.dlact svg{width:13px;height:13px}' +
      '.dlempty{padding:8px 6px;color:#98a0b3;font-size:11.5px}' +


      '@keyframes zpulse{50%{opacity:.45}}' +
      '.dl.busy svg{animation:zpulse 1.1s ease-in-out infinite}' +
      '.dl.failed{color:#f2615c}' +
      '@media (prefers-reduced-motion:reduce){.dl.busy svg{animation:none}}' +

      '.find{display:none;align-items:center;gap:2px;padding-left:4px}' +
      '.find.show{display:flex}' +
      '.findq{all:unset;width:150px;padding:4px 8px;border-radius:6px;' +
      'background:rgba(255,255,255,.08);color:#e9edf5;font:12px/1.2 inherit}' +
      '.findq::placeholder{color:#79839a}' +
      '.findq:focus{background:rgba(255,255,255,.14);box-shadow:0 0 0 1px #5d7cfa}' +
      '.find.empty .findq{box-shadow:0 0 0 1px #f2615c}' +


      '.findn{min-width:34px;text-align:center;color:#98a0b3;font-size:11px;'  +
      'font-variant-numeric:tabular-nums}' +
      '.find.empty .findn{color:#f2615c}' +


      '.wc svg{width:12px;height:12px;stroke-width:3}' +


      'button[data-a="shut"]:hover{background:#e5484d;color:#fff}' +


      'input[type=range]{-webkit-appearance:none;appearance:none;width:78px;height:4px;' +
      'border-radius:2px;background:rgba(255,255,255,.20);outline:none;cursor:pointer}' +
      'input[type=range]::-webkit-slider-thumb{-webkit-appearance:none;width:12px;' +
      'height:12px;border-radius:50%;background:#fff;cursor:pointer;' +
      'box-shadow:0 1px 3px rgba(0,0,0,.55)}' +
      '.pct{min-width:34px;text-align:right;color:#c8cede;' +
      'font-variant-numeric:tabular-nums;font-size:11px}' +
      '.vol{position:relative;display:flex}' +


      '.volpop{position:absolute;top:100%;left:0;right:0;' +
      'display:flex;justify-content:center;' +
      'transform:scale(.94);padding:0;margin-top:0;' +
      'opacity:0;pointer-events:none;transform-origin:top center;' +
      'transition:opacity .16s,transform .16s cubic-bezier(.22,.68,.24,1)}' +
      '.volpop.show{opacity:1;pointer-events:auto;transform:scale(1)}' +


      '.volpop .panel{display:flex;flex:none;flex-direction:column;' +
      'align-items:center;gap:7px;' +
      'width:46px;box-sizing:border-box;margin-top:6px;' +
      'padding:10px 8px;background:rgba(20,24,32,.97);' +
      'border:1px solid rgba(255,255,255,.10);border-radius:12px;' +
      'box-shadow:0 12px 30px -10px rgba(0,0,0,.8),' +
      'inset 0 1px 0 rgba(255,255,255,.05);' +
      '-webkit-backdrop-filter:blur(14px);backdrop-filter:blur(14px)}' +


      '.volpop input[type=range]{writing-mode:vertical-lr;direction:rtl;' +
      '-webkit-appearance:slider-vertical;width:22px;height:84px;' +
      'background:transparent;cursor:pointer}' +
      '.volnum{color:#c8cede;font-size:10.5px;font-variant-numeric:tabular-nums}' +
      '.tip{color:#fff;font-size:11px;padding:0 6px;opacity:0;transition:opacity .2s}' +
      '.tip.show{opacity:.75}' +
      '</style>' +
      '<div class="zone"></div>' +
      '<div class="rail"><div class="bar">' +
      '<div class="seg">' +
      '<button data-a="back" title="__ZWW_BACK__"><svg viewBox="0 0 24 24"><path d="M15 18l-6-6 6-6"/></svg></button>' +
      '<button data-a="forward" title="__ZWW_FORWARD__"><svg viewBox="0 0 24 24"><path d="M9 18l6-6-6-6"/></svg></button>' +
      '<button data-a="reload" title="__ZWW_RELOAD__"><svg viewBox="0 0 24 24"><path d="M21 12a9 9 0 11-3-6.7L21 8"/><path d="M21 3v5h-5"/></svg></button>' +


      '<button data-a="find" title="__ZWW_FIND__"><svg viewBox="0 0 24 24">' +
      '<circle cx="11" cy="11" r="7"/><path d="M20 20l-3.5-3.5"/></svg></button>' +
      '</div>' +


      '<div class="zoom"><svg viewBox="0 0 24 24"><path d="M3 19l5.5-14 5.5 14"/>' +
      '<path d="M5 14h7"/><path d="M16 19l3-8 3 8"/><path d="M17 16h4"/></svg>' +
      '<input type="range" min="50" max="250" step="10" value="100">' +
      '<span class="pct">100%</span></div>' +
      '<div class="seg">' +
      '<button data-a="pin" title="__ZWW_ONTOP__"><svg viewBox="0 0 24 24"><path d="M12 17v5"/><path d="M9 3h6l-1 6 3 3v2H7v-2l3-3z"/></svg></button>' +
      '<button data-a="compact" title="__ZWW_COMPACT__"><svg viewBox="0 0 24 24">' +
      '<rect x="3" y="6" width="18" height="12" rx="2"/><path d="M10 10l5 2-5 2z"/>' +
      '</svg></button>' +
      '<div class="vol">' +
      '<button data-a="mute" title="__ZWW_VOLUME__">' +
      '<svg viewBox="0 0 24 24"><path class="spk" d="M11 5L6 9H2v6h4l5 4z"/>' +
      '<path class="wave1" d="M15.5 8.5a5 5 0 010 7"/>' +
      '<path class="wave2" d="M18.5 5.5a9 9 0 010 13"/>' +
      '<path class="cross" d="M19 9l-6 6M13 9l6 6" style="display:none"/></svg></button>' +
      '<div class="volpop"><div class="panel">' +
      '<input type="range" class="volrange" min="0" max="100" step="1" value="100">' +
      '<span class="volnum">100</span>' +
      '</div></div></div>' +


      '<div class="dlwrap">' +
      '<button data-a="dl" class="dl" title="__ZWW_DOWNLOADS__"><svg viewBox="0 0 24 24">' +
      '<path d="M12 3v12"/><path d="M7 11l5 5 5-5"/><path d="M4 20h16"/></svg></button>' +
      '<div class="dlpop"><div class="dlpanel"></div></div>' +
      '</div>' +
      '<button data-a="copy" title="__ZWW_COPY__"><svg viewBox="0 0 24 24"><rect x="9" y="9" width="12" height="12" rx="2"/><path d="M5 15V5a2 2 0 012-2h10"/></svg></button>' +
      '<button data-a="browser" title="__ZWW_BROWSER__"><svg viewBox="0 0 24 24"><path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6"/><path d="M15 3h6v6M10 14L21 3"/></svg></button>' +
      '</div>' +
      '<div class="find"><input type="search" class="findq" placeholder="__ZWW_FIND__">' +
      '<span class="findn"></span>' +
      '<button data-a="findprev" title="__ZWW_PREV__"><svg viewBox="0 0 24 24">' +
      '<path d="M18 15l-6-6-6 6"/></svg></button>' +
      '<button data-a="findnext" title="__ZWW_NEXT__"><svg viewBox="0 0 24 24">' +
      '<path d="M6 9l6 6 6-6"/></svg></button>' +
      '<button data-a="findclose" title="__ZWW_CLOSE__"><svg viewBox="0 0 24 24">' +
      '<path d="M6 6l12 12M18 6L6 18"/></svg></button></div>' +
      '<div class="grip"></div>' +
      '<div class="seg">' +
      '<button data-a="max" class="wc" title="__ZWW_MAX__"><svg viewBox="0 0 24 24"><rect x="5" y="5" width="14" height="14" rx="2"/></svg></button>' +
      '<button data-a="shut" class="wc" title="__ZWW_CLOSE__"><svg viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></svg></button>' +
      '</div>' +
      '<span class="tip"></span>' +
      '</div></div>';


    var html = HTML;
    if (window.trustedTypes && window.trustedTypes.createPolicy) {
      try {
        var policy = window.trustedTypes.createPolicy('zwebwrap-toolbar', {
          createHTML: function (s) { return s; },
        });
        html = policy.createHTML(HTML);
      } catch (e) {


      }
    }
    try {
      root.innerHTML = html;
    } catch (e) {
      return;
    }


    function reattach() {
      if (host.isConnected && host.parentNode === document.body) return;
      var anchor = document.body || document.documentElement;
      if (anchor) anchor.appendChild(host);
    }
    reattach();

    new MutationObserver(reattach).observe(document, { childList: true, subtree: true });


    setInterval(reattach, 2000);

    var bar = root.querySelector('.bar');
    var zone = root.querySelector('.zone');
    var range = root.querySelector('input[type=range]');
    var pct = root.querySelector('.pct');
    var pin = root.querySelector('[data-a="pin"]');
    var mute = root.querySelector('[data-a="mute"]');
    var tip = root.querySelector('.tip');
    var hideTimer;

    function show() {
      clearTimeout(hideTimer);
      bar.classList.add('show');
    }
    function scheduleHide() {
      clearTimeout(hideTimer);
      hideTimer = setTimeout(function () {

        var pop = root.querySelector('.volpop');
        if (pop && pop.classList.contains('show')) return scheduleHide();
        bar.classList.remove('show');
      }, 700);
    }

    zone.addEventListener('mouseenter', show);
    bar.addEventListener('mouseenter', show);
    bar.addEventListener('mouseleave', scheduleHide);
    zone.addEventListener('mouseleave', scheduleHide);

    function flash(text) {
      tip.textContent = text;
      tip.classList.add('show');
      setTimeout(function () {
        tip.classList.remove('show');
      }, 1400);
    }


    var volRange = root.querySelector('.volrange');
    var volNum = root.querySelector('.volnum');
    var muteBtn = root.querySelector('[data-a="mute"]');
    var muted = false;
    var volume = 1;
    var adopted = false;


    var userSet = false;


    var writing = false;
    var wroteVolume = -1;
    var wroteMuted = null;

    function paintAudio() {
      var silent = muted || volume === 0;
      volRange.value = String(Math.round(volume * 100));
      volNum.textContent = volRange.value;
      muteBtn.classList.toggle('on', silent);
      root.querySelector('.cross').style.display = silent ? '' : 'none';

      root.querySelector('.wave1').style.display = silent || volume < 0.05 ? 'none' : '';
      root.querySelector('.wave2').style.display = silent || volume < 0.55 ? 'none' : '';
    }

    function mirror(el) {
      volume = el.volume;
      muted = el.muted;
      paintAudio();
    }


    function applyAudio() {
      var media = document.querySelectorAll('video,audio');
      for (var i = 0; i < media.length; i++) watch(media[i]);

      if (userSet) {
        var want = muted || volume === 0;
        writing = true;
        wroteVolume = volume;
        wroteMuted = want;
        try {
          for (var j = 0; j < media.length; j++) {
            if (media[j].volume !== volume) media[j].volume = volume;
            if (media[j].muted !== want) media[j].muted = want;
          }
        } finally {
          writing = false;
        }
      }

      paintAudio();
    }


    function watch(el) {
      if (el.__zwwAudio) return;
      el.__zwwAudio = true;

      el.addEventListener('volumechange', function () {
        if (writing) return;
        if (Math.abs(el.volume - wroteVolume) < 0.005 && el.muted === wroteMuted) return;
        mirror(el);
      });


      el.addEventListener('play', function () {
        if (userSet) {
          applyAudio();
        } else {
          adopted = true;
          mirror(el);
        }
      });

      if (!adopted) {
        adopted = true;
        mirror(el);
      }
    }

    new MutationObserver(applyAudio).observe(document.documentElement, {
      childList: true,
      subtree: true,
    });


    var volPop = root.querySelector('.volpop');
    var volWrap = root.querySelector('.vol');
    var volTimer;
    function volShow() {
      clearTimeout(volTimer);
      show();
      volPop.classList.add('show');
    }
    function volHide() {
      clearTimeout(volTimer);
      volTimer = setTimeout(function () {
        volPop.classList.remove('show');
      }, 700);
    }
    volWrap.addEventListener('mouseenter', volShow);
    volWrap.addEventListener('mouseleave', volHide);
    volPop.addEventListener('mouseenter', volShow);
    volPop.addEventListener('mouseleave', volHide);

    volPop.addEventListener('mousedown', volShow);


    var dlBtn = root.querySelector('.dl');
    var dlWrap = root.querySelector('.dlwrap');
    var dlPop = root.querySelector('.dlpop');
    var dlPanel = root.querySelector('.dlpanel');
    var dlLatest = null;
    var dlLinger;


    var dlAll = [];

    var ICON_OPEN =
      '<svg viewBox="0 0 24 24"><path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6"/>' +
      '<path d="M15 3h6v6M10 14L21 3"/></svg>';
    var ICON_FOLDER =
      '<svg viewBox="0 0 24 24"><path d="M3 7a2 2 0 012-2h4l2 2h8a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z"/></svg>';

    function bytes(n) {
      if (!n) return '';
      if (n < 1024) return n + ' B';
      var units = ['KB', 'MB', 'GB'];
      var v = n / 1024;
      var u = 0;
      while (v >= 1024 && u < units.length - 1) { v /= 1024; u++; }
      return (v < 10 ? v.toFixed(1) : Math.round(v)) + ' ' + units[u];
    }

    function drawDownloads() {
      if (!dlAll.length) {
        dlPanel.innerHTML = '<div class="dlempty">__ZWW_DLEMPTY__</div>';
        return;
      }
      var html = '';
      for (var i = dlAll.length - 1; i >= 0; i--) {
        var d = dlAll[i];
        var meta = d.state === 'active' ? bytes(d.bytes) + '…'
          : d.state === 'failed' ? '__ZWW_DLFAILED__'
          : bytes(d.bytes);


        var name = String(d.file_name)
          .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
        html +=
          '<div class="dlrow ' + d.state + '">' +
          '<span class="dlname" title="' + name + '">' + name + '</span>' +
          '<span class="dlmeta">' + meta + '</span>' +
          (d.state === 'done'
            ? '<button class="dlact" data-open="' + d.id + '" title="__ZWW_DLOPEN__">' + ICON_OPEN + '</button>' +
              '<button class="dlact" data-show="' + d.id + '" title="__ZWW_DLSHOW__">' + ICON_FOLDER + '</button>'
            : '') +
          '</div>';
      }
      dlPanel.innerHTML = html;
    }

    function dlPopShow() { show(); drawDownloads(); dlPop.classList.add('show'); }
    function dlPopHide() { dlPop.classList.remove('show'); }


    dlPanel.addEventListener('click', function (e) {
      var btn = e.target.closest ? e.target.closest('.dlact') : null;
      if (!btn) return;
      var open = btn.getAttribute('data-open');
      var show_ = btn.getAttribute('data-show');
      if (open) call('download_open', Number(open));
      else if (show_) call('download_show', Number(show_));
    });


    dlPop.addEventListener('mousedown', function () { show(); });

    window.__zwwDownload = function (record) {
      if (!record) return;
      dlLatest = record;
      var at = -1;
      for (var i = 0; i < dlAll.length; i++) if (dlAll[i].id === record.id) at = i;
      if (at >= 0) dlAll[at] = record; else dlAll.push(record);
      if (dlPop.classList.contains('show')) drawDownloads();
      dlWrap.classList.add('show');
      dlBtn.classList.toggle('busy', record.state === 'active');
      dlBtn.classList.toggle('failed', record.state === 'failed');


      function announce(text) {
        clearTimeout(dlLinger);
        show();
        flash(text);
        dlLinger = setTimeout(scheduleHide, 5000);
      }

      if (record.state === 'done') {
        announce(record.file_name || '__ZWW_DLDONE__');
        feedback(dlBtn, 'a-pop');
      } else if (record.state === 'failed') {
        announce('__ZWW_DLFAILED__');
      } else {


        show();
        dlLinger = setTimeout(scheduleHide, 2500);
      }
    };


    var findWrap = root.querySelector('.find');
    var findInput = root.querySelector('.findq');
    var findCount = root.querySelector('.findn');


    function findRun(forward) {
      var q = findInput.value;
      if (!q) {
        findWrap.classList.remove('empty');
        if (window.__zwwFindStop) window.__zwwFindStop();
        findCount.textContent = '';
        return;
      }
      var n = window.__zwwFind ? window.__zwwFind(q, forward !== false) : 0;
      findWrap.classList.toggle('empty', !n);
      findCount.textContent = n ? window.__zwwFindAt() + '/' + n : '0';
    }

    function findOpen() {
      show();
      findWrap.classList.add('show');
      findInput.focus();
      findInput.select();
    }

    function findClose() {
      findWrap.classList.remove('show');
      findWrap.classList.remove('empty');
      findInput.value = '';
      findCount.textContent = '';
      if (window.__zwwFindStop) window.__zwwFindStop();
    }

    findInput.addEventListener('input', function () { findRun(true); });
    findInput.addEventListener('keydown', function (e) {
      if (e.key === 'Enter') {
        e.preventDefault();
        findRun(!e.shiftKey);
      } else if (e.key === 'Escape') {
        e.preventDefault();
        findClose();
      }

      e.stopPropagation();
    });

    document.addEventListener('keydown', function (e) {
      if ((e.ctrlKey || e.metaKey) && (e.key === 'f' || e.key === 'F')) {
        e.preventDefault();
        findOpen();
        return;
      }


      if (e.key === 'F3' || ((e.ctrlKey || e.metaKey) && (e.key === 'g' || e.key === 'G'))) {
        e.preventDefault();
        if (!findWrap.classList.contains('show') || !findInput.value) {
          findOpen();
        } else {
          findRun(!e.shiftKey);
        }
      }
    }, true);


    var COMPACT_CSS =
      'html,body{overflow:hidden!important;background:#000!important;' +
      'margin:0!important;padding:0!important;border:0!important}' +
      'body *{visibility:hidden!important}' +


      '#__zww_bar_host{visibility:visible!important}' +
      'video{visibility:visible!important;position:fixed!important;' +
      'top:0!important;left:0!important;width:100vw!important;height:100vh!important;' +
      'max-width:none!important;max-height:none!important;object-fit:contain!important;' +
      'z-index:2147483000!important;background:#000!important;' +

      'border-radius:0!important;margin:0!important;padding:0!important;border:0!important}';


    var compactAncestors = [];

    function freeVideo(video) {
      compactAncestors = [];
      var el = video.parentElement;
      while (el && el !== document.documentElement) {
        compactAncestors.push({
          el: el,
          transform: el.style.transform,
          filter: el.style.filter,
          perspective: el.style.perspective,
          contain: el.style.contain,
          overflow: el.style.overflow,
        });
        el.style.setProperty('transform', 'none', 'important');
        el.style.setProperty('filter', 'none', 'important');
        el.style.setProperty('perspective', 'none', 'important');
        el.style.setProperty('contain', 'none', 'important');
        el.style.setProperty('overflow', 'visible', 'important');
        el = el.parentElement;
      }
    }

    function restoreAncestors() {
      for (var i = 0; i < compactAncestors.length; i++) {
        var was = compactAncestors[i];
        was.el.style.transform = was.transform;
        was.el.style.filter = was.filter;
        was.el.style.perspective = was.perspective;
        was.el.style.contain = was.contain;
        was.el.style.overflow = was.overflow;
      }
      compactAncestors = [];
    }

    var compactOn = false;
    var compactStyle = null;
    var compactVideo = null;
    var compactHadControls = false;

    function biggestVideo() {
      var best = null;
      var videos = document.querySelectorAll('video');
      for (var i = 0; i < videos.length; i++) {
        var v = videos[i];

        if (!v.videoWidth || !v.videoHeight) continue;
        if (!best || v.videoWidth * v.videoHeight > best.videoWidth * best.videoHeight) {
          best = v;
        }
      }
      return best;
    }

    function compactOff() {
      if (compactStyle) {
        compactStyle.remove();
        compactStyle = null;
      }
      restoreAncestors();
      if (compactVideo) {
        compactVideo.controls = compactHadControls;
        compactVideo = null;
      }
      compactOn = false;
      document.removeEventListener('keydown', compactEscape, true);
      root.querySelector('[data-a="compact"]').classList.remove('on');
      call('compact', 0);
    }


    function compactEscape(e) {
      if (e.key === 'Escape' && compactOn) {
        e.preventDefault();
        e.stopPropagation();
        compactOff();
      }
    }

    function compactEnter() {
      var video = biggestVideo();
      if (!video) {
        flash('__ZWW_NOVIDEO__');
        return;
      }

      compactStyle = document.createElement('style');
      compactStyle.setAttribute('data-zww', 'compact');
      compactStyle.textContent = COMPACT_CSS;
      (document.head || document.documentElement).appendChild(compactStyle);


      freeVideo(video);
      compactVideo = video;
      compactHadControls = video.controls;
      video.controls = true;

      compactOn = true;
      document.addEventListener('keydown', compactEscape, true);


      show();
      root.querySelector('[data-a="compact"]').classList.add('on');
      call('compact', video.videoWidth / video.videoHeight);
    }

    var reloadBtn = root.querySelector('[data-a="reload"]');
    if (document.readyState !== 'complete') {
      reloadBtn.classList.add('a-loading');
      window.addEventListener(
        'load',
        function () { reloadBtn.classList.remove('a-loading'); },
        { once: true },
      );
    }


    function inertSpot(target) {
      return !target.closest('button,input,.volpop,.dlpop,.find');
    }

    bar.addEventListener('mousedown', function (e) {
      if (e.button !== 0 || !inertSpot(e.target)) return;
      e.preventDefault();
      call('drag');
    });
    bar.addEventListener('dblclick', function (e) {
      if (inertSpot(e.target)) call('toggle_maximize');
    });

    volRange.addEventListener('input', function () {
      userSet = true;
      volume = Number(volRange.value) / 100;
      volNum.textContent = volRange.value;

      if (volume > 0) muted = false;
      applyAudio();
    });

    function call2(command, args) {
      var fn = bridge();
      if (!fn) return Promise.resolve(null);
      return fn(command, args).catch(function () { return null; });
    }


    var FEEDBACK = {
      reload: 'a-spin',
      back: 'a-back',
      forward: 'a-fwd',
      pin: 'a-lift',
      mute: 'a-pop',
      compact: 'a-pop',
      copy: 'a-pop',
      dl: 'a-lift',
      browser: 'a-fly',
      max: 'a-pop',
      shut: 'a-shake',
    };


    function feedback(btn, cls) {
      if (!cls) return;
      btn.classList.remove(cls);
      void btn.offsetWidth;
      btn.classList.add(cls);
      var stop = function () {
        btn.classList.remove(cls);
        btn.removeEventListener('animationend', stop);
      };
      btn.addEventListener('animationend', stop);
    }

    root.addEventListener('click', function (e) {
      var btn = e.target.closest ? e.target.closest('button') : null;
      if (!btn) return;
      var a = btn.getAttribute('data-a');
      feedback(btn, FEEDBACK[a]);

      if (a === 'pin') {
        var next = !btn.classList.contains('on');
        call('always_on_top', next ? 1 : 0).then(function (s) {
          btn.classList.toggle('on', s.always_on_top);
          flash(s.always_on_top ? '__ZWW_ONTOP__' : '__ZWW_NOTONTOP__');
        });
      } else if (a === 'mute') {
        userSet = true;
        muted = !muted;
        applyAudio();
        flash(muted ? '__ZWW_MUTED__' : '__ZWW_UNMUTED__');
      } else if (a === 'copy') {
        var url = window.location.href;
        var done = function () { flash('__ZWW_COPIED__'); };
        if (navigator.clipboard && navigator.clipboard.writeText) {
          navigator.clipboard.writeText(url).then(done, function () { fallbackCopy(url, done); });
        } else {
          fallbackCopy(url, done);
        }
      } else if (a === 'find') {
        if (findWrap.classList.contains('show')) findClose();
        else findOpen();
      } else if (a === 'dl') {
        if (dlPop.classList.contains('show')) dlPopHide();
        else dlPopShow();
      } else if (a === 'browser') {
        call('open_in_browser');
      } else if (a === 'compact') {
        if (compactOn) compactOff(); else compactEnter();
      } else if (a === 'max') {
        call('toggle_maximize');
      } else if (a === 'findnext') {
        findRun(true);
      } else if (a === 'findprev') {
        findRun(false);
      } else if (a === 'findclose') {
        findClose();
      } else if (a === 'shut') {
        call('close');
      } else {
        call(a);
      }
    });

    function fallbackCopy(text, done) {
      try {
        var ta = document.createElement('textarea');
        ta.value = text;
        ta.style.cssText = 'position:fixed;opacity:0';
        document.body.appendChild(ta);
        ta.select();
        document.execCommand('copy');
        document.body.removeChild(ta);
        done();
      } catch (e) {}
    }


    function paintZoom() {
      var min = Number(range.min);
      var span = Number(range.max) - min;
      var at = span ? ((Number(range.value) - min) / span) * 100 : 0;
      range.style.background =
        'linear-gradient(90deg,#5d7cfa ' + at + '%,rgba(255,255,255,.20) ' + at + '%)';
    }

    range.addEventListener('input', function () {
      pct.textContent = range.value + '%';
      paintZoom();
    });
    paintZoom();
    range.addEventListener('change', function () {
      call('zoom', Number(range.value) / 100);
    });


    call('state').then(function (s) {
      if (!s) return;
      pin.classList.toggle('on', s.always_on_top);
      var z = Math.round((s.zoom || 1) * 100);
      range.value = z;
      pct.textContent = z + '%';


      paintZoom();
    }).catch(function () {});
  }

  build();
})();
"#;

pub const FIND_JS: &str = r#"
(function () {
  'use strict';
  if (window.__zwwFind) return;

  var HL = 'zww-find';
  var ranges = [];
  var at = -1;

  function styled() {
    if (document.getElementById('__zww_find_style')) return;
    var s = document.createElement('style');
    s.id = '__zww_find_style';

    s.textContent =
      '::highlight(zww-find){background:#ffd54f;color:#000}' +
      '::highlight(zww-find-active){background:#ff9800;color:#000}';
    (document.head || document.documentElement).appendChild(s);
  }

  function clear() {
    ranges = [];
    at = -1;
    if (window.CSS && CSS.highlights) {
      CSS.highlights.delete(HL);
      CSS.highlights.delete(HL + '-active');
    }
  }


  function textNodes() {
    var out = [];
    var walker = document.createTreeWalker(document.body || document.documentElement,
      NodeFilter.SHOW_TEXT, {
        acceptNode: function (node) {
          if (!node.nodeValue || !node.nodeValue.trim()) return NodeFilter.FILTER_REJECT;
          var parent = node.parentElement;
          if (!parent) return NodeFilter.FILTER_REJECT;
          var tag = parent.tagName;
          if (tag === 'SCRIPT' || tag === 'STYLE' || tag === 'NOSCRIPT') {
            return NodeFilter.FILTER_REJECT;
          }


          if (parent.closest && parent.closest('#__zww_bar_host')) {
            return NodeFilter.FILTER_REJECT;
          }
          return NodeFilter.FILTER_ACCEPT;
        },
      });
    var n;
    while ((n = walker.nextNode())) out.push(n);
    return out;
  }

  function collect(query) {
    clear();
    if (!query) return 0;

    var needle = query.toLowerCase();
    var nodes = textNodes();
    for (var i = 0; i < nodes.length; i++) {
      var hay = nodes[i].nodeValue.toLowerCase();
      var from = 0;
      var hit;


      while ((hit = hay.indexOf(needle, from)) !== -1) {
        var r = document.createRange();
        try {
          r.setStart(nodes[i], hit);
          r.setEnd(nodes[i], hit + needle.length);
          ranges.push(r);
        } catch (e) {}
        from = hit + needle.length;
      }
    }
    return ranges.length;
  }

  function paint() {
    if (!window.CSS || !CSS.highlights || !window.Highlight) return;
    styled();


    var rest = new Highlight();
    var any = false;
    for (var i = 0; i < ranges.length; i++) {
      if (i === at) continue;
      rest.add(ranges[i]);
      any = true;
    }
    if (any) CSS.highlights.set(HL, rest);
    else CSS.highlights.delete(HL);
    if (at >= 0 && ranges[at]) CSS.highlights.set(HL + '-active', new Highlight(ranges[at]));
  }

  function reveal() {
    if (at < 0 || !ranges[at]) return;
    var node = ranges[at].startContainer;
    var el = node.nodeType === 1 ? node : node.parentElement;
    if (el && el.scrollIntoView) el.scrollIntoView({ block: 'center', inline: 'nearest' });
  }

  var lastQuery = null;

  window.__zwwFind = function (query, forward) {
    if (query !== lastQuery) {
      lastQuery = query;
      collect(query);
      at = -1;
    }
    if (!ranges.length) return 0;
    at = forward === false
      ? (at <= 0 ? ranges.length - 1 : at - 1)
      : (at + 1) % ranges.length;
    paint();
    reveal();
    return ranges.length;
  };


  window.__zwwFindAt = function () {
    return at + 1;
  };

  window.__zwwFindStop = function () {
    lastQuery = null;
    clear();
  };
})();
"#;

pub const GENERIC_COSMETIC: &str = r#"
(function () {
  'use strict';
  if (window.__zwwGeneric) return;
  window.__zwwGeneric = true;

  var invoke = null;
  function bridge() {
    if (!invoke) {
      invoke =
        (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) ||
        (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke);
    }
    return invoke;
  }

  var seen = new Set();
  var applied = new Set();
  var sheet = null;
  var pending = null;

  function styleSheet() {
    if (!sheet) {
      sheet = document.createElement('style');
      sheet.setAttribute('data-zww', 'generic');
      (document.head || document.documentElement).appendChild(sheet);
    }
    return sheet;
  }

  function collect(root) {
    var fresh = [];
    var nodes = root.querySelectorAll ? root.querySelectorAll('[class],[id]') : [];
    for (var i = 0; i < nodes.length; i++) {
      var el = nodes[i];
      if (el.id) {
        var key = '#' + el.id;
        if (!seen.has(key)) { seen.add(key); fresh.push(key); }
      }
      var cl = el.classList;
      for (var j = 0; j < cl.length; j++) {
        var ck = '.' + cl[j];
        if (!seen.has(ck)) { seen.add(ck); fresh.push(ck); }
      }
    }
    return fresh;
  }

  function ask(tokens) {
    var fn = bridge();
    if (!fn || !tokens.length) return;
    fn('zww_page_selectors', { tokens: tokens })
      .then(function (selectors) {
        if (!selectors || !selectors.length) return;
        var added = [];
        for (var i = 0; i < selectors.length; i++) {
          if (!applied.has(selectors[i])) {
            applied.add(selectors[i]);
            added.push(selectors[i]);
          }
        }
        if (added.length) {
          styleSheet().textContent +=
            added.join(',') + '{display:none!important}\n';


          reportHidden();
        }
      })
      .catch(function () {});
  }


  var reported = 0;

  function countHidden() {
    var total = 0;
    var sheets = document.querySelectorAll('style[data-zww]');
    for (var i = 0; i < sheets.length; i++) {
      var rules;
      try {
        rules = sheets[i].sheet && sheets[i].sheet.cssRules;
      } catch (e) {
        continue;
      }
      if (!rules) continue;
      for (var j = 0; j < rules.length; j++) {
        var sel = rules[j].selectorText;
        if (!sel) continue;
        try {
          total += document.querySelectorAll(sel).length;
        } catch (e) {

        }
      }
    }
    return total;
  }

  function reportHidden() {
    var now = countHidden();


    if (now <= reported) return;
    reported = now;
    var fn = bridge();
    if (fn) fn('zww_page_hidden', { count: now }).catch(function () {});
  }

  function scan(root) {
    var fresh = collect(root || document);
    if (fresh.length) ask(fresh);
    reportHidden();
  }


  function schedule() {
    if (pending) return;
    pending = setTimeout(function () {
      pending = null;
      scan(document);
    }, 400);
  }

  function start() {
    scan(document);
    new MutationObserver(schedule).observe(document.documentElement, {
      childList: true,
      subtree: true,
      attributes: true,
      attributeFilter: ['class', 'id'],
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', start);
  } else {
    start();
  }
})();
"#;

pub fn localized_toolbar() -> String {
    use zww_core::lang::{t, Key};

    let locale = zww_core::lang::active();
    let mut out = TOOLBAR.to_string();
    for (token, key) in [
        ("__ZWW_BACK__", Key::Back),
        ("__ZWW_FORWARD__", Key::Forward),
        ("__ZWW_RELOAD__", Key::Reload),
        ("__ZWW_ONTOP__", Key::AlwaysOnTop),
        ("__ZWW_NOTONTOP__", Key::NotOnTop),
        ("__ZWW_VOLUME__", Key::Volume),
        ("__ZWW_COPY__", Key::CopyAddress),
        ("__ZWW_BROWSER__", Key::OpenInBrowser),
        ("__ZWW_MAX__", Key::Maximise),
        ("__ZWW_CLOSE__", Key::Close),
        ("__ZWW_FIND__", Key::Find),
        ("__ZWW_PREV__", Key::Previous),
        ("__ZWW_NEXT__", Key::Next),
        ("__ZWW_COMPACT__", Key::Compact),
        ("__ZWW_NOVIDEO__", Key::NoVideo),
        ("__ZWW_DOWNLOADS__", Key::Downloads),
        ("__ZWW_DLDONE__", Key::DownloadFinished),
        ("__ZWW_DLFAILED__", Key::DownloadFailed),
        ("__ZWW_DLEMPTY__", Key::NoDownloadsYet),
        ("__ZWW_DLOPEN__", Key::OpenFile),
        ("__ZWW_DLSHOW__", Key::ShowInFolder),
        ("__ZWW_COPIED__", Key::AddressCopied),
        ("__ZWW_MUTED__", Key::Muted),
        ("__ZWW_UNMUTED__", Key::Unmuted),
    ] {
        out = out.replace(token, &t(locale, key).replace('\'', "\\'"));
    }
    out
}

pub fn user_scripts(app: &zww_core::WrappedApp) -> Option<String> {
    let css = app.user_css.as_deref().unwrap_or("").trim().to_string();
    let js = app.user_js.as_deref().unwrap_or("").trim().to_string();
    if css.is_empty() && js.is_empty() {
        return None;
    }

    let mut out = String::from("(function(){'use strict';\n");

    if !css.is_empty() {
        out.push_str(&format!(
            "try{{var s=document.createElement('style');\
             s.setAttribute('data-zww','user');\
             s.textContent={};\
             (document.head||document.documentElement).appendChild(s);\
             }}catch(e){{}}\n",
            serde_json::to_string(&css).unwrap_or_else(|_| "\"\"".into())
        ));
    }

    out.push_str("})();\n");

    if !js.is_empty() {
        out.push_str("(function(){try{\n");
        out.push_str(&js);
        out.push_str("\n}catch(e){console.error('ZWebWrap user script:',e);}})();");
    }

    Some(out)
}

pub const TITLE_BADGE: &str = r#"
(function () {
  if (window.__zwwBadge) return;
  window.__zwwBadge = true;

  var last = null;

  function count() {
    var t = document.title || '';

    var m = t.match(/\((\d+)\)/);
    if (m) return parseInt(m[1], 10) || 0;
    return /^\s*[•*]/.test(t) ? -1 : 0;
  }

  function report() {
    var n = count();
    if (n === last) return;
    last = n;
    var fn =
      (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) ||
      (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke);
    if (fn) fn('zww_window_action', { action: 'badge', value: n }).catch(function () {});
  }

  report();


  new MutationObserver(report).observe(document.querySelector('head') || document.documentElement, {
    subtree: true,
    childList: true,
    characterData: true,
  });
})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_placeholder_is_replaced() {
        let built = localized_toolbar();
        assert!(!built.contains("__ZWW_"), "a language placeholder survived into the toolbar");
    }

    #[test]
    fn every_command_the_toolbar_calls_is_permitted() {
        let allowlist = include_str!("../permissions/toolbar.toml");

        let scripts = [TOOLBAR, GENERIC_COSMETIC, TITLE_BADGE];
        let mut called: Vec<&str> = Vec::new();
        for script in scripts {
            for (index, _) in script.match_indices("('zww_") {
                let rest = &script[index + 2..];
                if let Some(end) = rest.find('\'') {
                    called.push(&rest[..end]);
                }
            }
        }
        called.sort_unstable();
        called.dedup();

        assert!(!called.is_empty(), "the extractor found no commands at all; it has drifted");

        for command in called {
            assert!(
                allowlist.contains(&format!("\"{command}\"")),
                "the toolbar calls `{command}` but permissions/toolbar.toml does not allow it, \
                 so a wrapped page's call will be denied and silently swallowed"
            );
        }
    }
}
