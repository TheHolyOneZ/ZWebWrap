(function () {
  'use strict';
  if (!window.isSecureContext || window.__zwwHid) return;

  var internals = window.__TAURI_INTERNALS__;
  var invoke =
    (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) ||
    (internals && internals.invoke);
  if (!invoke) return;

  function toB64(buffer) {
    var bytes;
    if (buffer instanceof ArrayBuffer) bytes = new Uint8Array(buffer);
    else if (ArrayBuffer.isView(buffer))
      bytes = new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    else throw new TypeError('The data must be a BufferSource.');
    var out = '';
    for (var i = 0; i < bytes.length; i += 0x8000)
      out += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
    return btoa(out);
  }

  function fromB64(text) {
    var bin = atob(text);
    var bytes = new Uint8Array(bin.length);
    for (var i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
    return bytes;
  }

  function fail(err) {
    var text = typeof err === 'string' ? err : (err && err.message) || String(err);
    var m = /^([A-Za-z]+Error): ?(.*)$/.exec(text);
    return new DOMException(m ? m[2] : text, m ? m[1] : 'NotAllowedError');
  }

  function call(cmd, args) {
    return invoke(cmd, args).catch(function (e) {
      throw fail(e);
    });
  }

  var devices = new Map();
  var privateKey = {};

  function HIDInputReportEvent(device, reportId, data) {
    var event = new Event('inputreport');
    Object.defineProperties(event, {
      device: { value: device, enumerable: true },
      reportId: { value: reportId, enumerable: true },
      data: { value: data, enumerable: true },
    });
    return event;
  }

  function HIDConnectionEvent(type, device) {
    var event = new Event(type);
    Object.defineProperty(event, 'device', { value: device, enumerable: true });
    return event;
  }

  class HIDDevice extends EventTarget {
    constructor(key, info) {
      if (key !== privateKey) throw new TypeError('Illegal constructor');
      super();
      this._id = info.id;
      this._opened = false;
      this._busy = false;
      this.oninputreport = null;
      this._info = info;
      var self = this;
      this.addEventListener('inputreport', function (e) {
        if (typeof self.oninputreport === 'function') self.oninputreport.call(self, e);
      });
    }
    get opened() { return this._opened; }
    get vendorId() { return this._info.vendorId; }
    get productId() { return this._info.productId; }
    get productName() { return this._info.productName; }
    get collections() { return this._info.collections; }

    open() {
      var self = this;
      if (self._opened) return Promise.resolve();
      if (self._busy) return Promise.reject(new DOMException('An operation is already in progress.', 'InvalidStateError'));
      self._busy = true;
      return call('zww_hid_open', { id: self._id }).then(
        function () { self._opened = true; self._busy = false; },
        function (e) { self._busy = false; throw e; }
      );
    }

    close() {
      var self = this;
      if (!self._opened) return Promise.resolve();
      self._opened = false;
      return call('zww_hid_close', { id: self._id });
    }

    forget() {
      var self = this;
      self._opened = false;
      devices.delete(self._id);
      return call('zww_hid_forget', { id: self._id });
    }

    _need() {
      if (!this._opened)
        return Promise.reject(new DOMException('The device must be opened first.', 'InvalidStateError'));
      return null;
    }

    sendReport(reportId, data) {
      return this._need() || call('zww_hid_send_report', { id: this._id, reportId: reportId & 255, data: toB64(data) });
    }

    sendFeatureReport(reportId, data) {
      return this._need() || call('zww_hid_send_feature_report', { id: this._id, reportId: reportId & 255, data: toB64(data) });
    }

    receiveFeatureReport(reportId) {
      return (
        this._need() ||
        call('zww_hid_receive_feature_report', { id: this._id, reportId: reportId & 255 }).then(function (b64) {
          return new DataView(fromB64(b64).buffer);
        })
      );
    }
  }

  function wrap(info) {
    var existing = devices.get(info.id);
    if (existing) return existing;
    var device = new HIDDevice(privateKey, info);
    devices.set(info.id, device);
    return device;
  }

  var listenerCount = 0;
  var known = null;
  var timer = null;

  function poll() {
    call('zww_hid_get_devices').then(function (list) {
      var now = new Map();
      list.forEach(function (info) { now.set(info.id, info); });
      if (known) {
        now.forEach(function (info, id) {
          if (!known.has(id)) hid.dispatchEvent(HIDConnectionEvent('connect', wrap(info)));
        });
        known.forEach(function (_, id) {
          if (!now.has(id)) {
            var gone = devices.get(id);
            if (gone) {
              gone._opened = false;
              hid.dispatchEvent(HIDConnectionEvent('disconnect', gone));
            }
          }
        });
      }
      known = now;
    }, function () {});
  }

  function updatePolling() {
    if (listenerCount > 0 && !timer) {
      known = null;
      poll();
      timer = setInterval(poll, 1500);
    } else if (listenerCount === 0 && timer) {
      clearInterval(timer);
      timer = null;
    }
  }

  class HID extends EventTarget {
    constructor(key) {
      if (key !== privateKey) throw new TypeError('Illegal constructor');
      super();
      this.onconnect = null;
      this.ondisconnect = null;
      var self = this;
      this.addEventListener('connect', function (e) { if (typeof self.onconnect === 'function') self.onconnect.call(self, e); });
      this.addEventListener('disconnect', function (e) { if (typeof self.ondisconnect === 'function') self.ondisconnect.call(self, e); });
    }
    addEventListener(type, listener, options) {
      if (listener && (type === 'connect' || type === 'disconnect')) { listenerCount++; updatePolling(); }
      return super.addEventListener(type, listener, options);
    }
    removeEventListener(type, listener, options) {
      if (listener && (type === 'connect' || type === 'disconnect') && listenerCount > 0) { listenerCount--; updatePolling(); }
      return super.removeEventListener(type, listener, options);
    }
    getDevices() {
      return call('zww_hid_get_devices').then(function (list) { return list.map(wrap); });
    }
    requestDevice(options) {
      options = options || {};
      if (!Array.isArray(options.filters))
        return Promise.reject(new TypeError("Failed to execute 'requestDevice' on 'HID': required member filters is undefined."));
      if (navigator.userActivation && !navigator.userActivation.isActive)
        return Promise.reject(new DOMException('Must be handling a user gesture to show a permission request.', 'SecurityError'));
      return call('zww_hid_request_device', {
        filters: options.filters,
        exclusionFilters: options.exclusionFilters || [],
        multiple: !!options.multiple,
      }).then(function (list) { return list.map(wrap); });
    }
  }

  var hid = new HID(privateKey);
  var tag = function (cls, name) {
    Object.defineProperty(cls.prototype, Symbol.toStringTag, { value: name, configurable: true });
  };
  tag(HID, 'HID');
  tag(HIDDevice, 'HIDDevice');

  Object.defineProperty(Navigator.prototype, 'hid', { get: function () { return hid; }, configurable: true, enumerable: true });
  window.HID = HID;
  window.HIDDevice = HIDDevice;
  window.HIDInputReportEvent = HIDInputReportEvent;
  window.HIDConnectionEvent = HIDConnectionEvent;

  Object.defineProperty(window, '__zwwHid', {
    value: Object.freeze({
      input: function (id, reportId, b64) {
        var device = devices.get(id);
        if (!device || !device._opened) return;
        device.dispatchEvent(HIDInputReportEvent(device, reportId, new DataView(fromB64(b64).buffer)));
      },
      gone: function (id) {
        var device = devices.get(id);
        if (!device) return;
        device._opened = false;
        hid.dispatchEvent(HIDConnectionEvent('disconnect', device));
      },
    }),
  });
})();
