// Minimal CDP client built on raw `ws` — used instead of
// `chrome-remote-interface` because Android WebView 113 (shipped in the
// `api-level: 34, target: default` emulator image) silently drops the
// connection during the WebSocket handshake when chrome-remote-interface
// connects. Raw `ws` lets us:
//
//   * see the actual HTTP upgrade response (status / headers) on failure
//   * override request headers (some Android WebView CDP servers reject
//     unexpected Host / Origin combinations)
//   * skip every auto-init message chrome-remote-interface would send
//
// API mirrors the small subset of chrome-remote-interface we used:
//   const client = await connectCdp({ host, port });
//   await client.Runtime.enable();
//   await client.Runtime.evaluate({ expression, returnByValue, awaitPromise });
//   await client.Page.enable();
//   client.close();

import http from "node:http";
import WebSocket from "ws";

function httpGetJson(host, port, path) {
  return new Promise((resolve, reject) => {
    const req = http.get(
      { host, port, path, headers: { Host: `${host}:${port}` } },
      (res) => {
        let buf = "";
        res.setEncoding("utf8");
        res.on("data", (c) => (buf += c));
        res.on("end", () => {
          if (res.statusCode !== 200) {
            reject(
              new Error(
                `GET ${path} -> ${res.statusCode}\n${buf.slice(0, 500)}`,
              ),
            );
            return;
          }
          try {
            resolve(JSON.parse(buf));
          } catch (e) {
            reject(new Error(`failed to parse JSON from ${path}: ${e}\n${buf.slice(0, 500)}`));
          }
        });
      },
    );
    req.on("error", reject);
    req.setTimeout(10_000, () => req.destroy(new Error(`GET ${path} timed out`)));
  });
}

/**
 * @param {{host: string, port: number, target?: object|null, headers?: object}} opts
 * @returns {Promise<{
 *   ws: WebSocket,
 *   send: (method: string, params?: object) => Promise<any>,
 *   close: () => void,
 *   Runtime: {
 *     enable: () => Promise<any>,
 *     evaluate: (params: object) => Promise<any>,
 *   },
 *   Page: { enable: () => Promise<any> },
 * }>}
 */
export async function connectCdp({ host, port, target = null, headers = {} } = {}) {
  let pageTarget = target;
  if (!pageTarget) {
    const targets = await httpGetJson(host, port, "/json/list");
    pageTarget = targets.find((t) => t.type === "page");
    if (!pageTarget) {
      throw new Error(
        `no page target in WebView devtools; targets: ${JSON.stringify(targets)}`,
      );
    }
  }
  const wsUrl = pageTarget.webSocketDebuggerUrl;
  if (!wsUrl) {
    throw new Error(
      `target missing webSocketDebuggerUrl: ${JSON.stringify(pageTarget)}`,
    );
  }
  console.log(`[cdp] connecting to ${wsUrl}`);

  // CRITICAL: do NOT set the Origin header. Chromium DevTools (since
  // CVE-2022-1853 mitigation) rejects any WebSocket whose Origin isn't
  // in the `--remote-allow-origins` allowlist with 403 Forbidden — and
  // we can't pass flags to the Tauri WebView. Connections with no
  // Origin header are treated as "local" and accepted. The `ws` library
  // omits Origin by default, so we just don't add one.
  const ws = new WebSocket(wsUrl, {
    perMessageDeflate: false,
    handshakeTimeout: 15_000,
    headers: { ...headers },
  });

  // Verbose upgrade-failure logging — this is the whole point of the rewrite.
  ws.on("unexpected-response", (req, res) => {
    let body = "";
    res.on("data", (c) => (body += c.toString()));
    res.on("end", () => {
      console.error(
        `[cdp] WS handshake rejected: ${res.statusCode} ${res.statusMessage}\n` +
          `headers: ${JSON.stringify(res.headers)}\n` +
          `body: ${body.slice(0, 500)}`,
      );
    });
  });

  await new Promise((resolve, reject) => {
    const onOpen = () => {
      ws.off("error", onError);
      resolve();
    };
    const onError = (err) => {
      ws.off("open", onOpen);
      reject(new Error(`WS connect failed: ${err.message || err}`));
    };
    ws.once("open", onOpen);
    ws.once("error", onError);
  });
  console.log(`[cdp] connected`);

  let nextId = 1;
  const pending = new Map();
  const eventListeners = new Map(); // method -> Array<(params) => void>

  ws.on("message", (data) => {
    let msg;
    try {
      msg = JSON.parse(data.toString());
    } catch {
      return;
    }
    if (typeof msg.id === "number") {
      const cb = pending.get(msg.id);
      if (!cb) return;
      pending.delete(msg.id);
      if (msg.error) {
        cb.reject(new Error(`CDP error: ${JSON.stringify(msg.error)}`));
      } else {
        cb.resolve(msg.result);
      }
      return;
    }
    // Event (no id, has method + params).
    if (typeof msg.method === "string") {
      const ls = eventListeners.get(msg.method);
      if (ls) for (const l of ls) try { l(msg.params); } catch {}
    }
  });

  ws.on("close", (code, reason) => {
    const err = new Error(
      `CDP websocket closed: ${code} ${reason?.toString?.() || ""}`,
    );
    for (const { reject } of pending.values()) reject(err);
    pending.clear();
  });

  const send = (method, params = {}) =>
    new Promise((resolve, reject) => {
      const id = nextId++;
      pending.set(id, { resolve, reject });
      ws.send(JSON.stringify({ id, method, params }), (err) => {
        if (err) {
          pending.delete(id);
          reject(err);
        }
      });
    });

  function on(method, listener) {
    if (!eventListeners.has(method)) eventListeners.set(method, []);
    eventListeners.get(method).push(listener);
  }

  return {
    ws,
    send,
    on,
    close: () => ws.close(),
    Runtime: {
      enable: () => send("Runtime.enable"),
      evaluate: (params) => send("Runtime.evaluate", params),
    },
    Page: {
      enable: () => send("Page.enable"),
    },
    Network: {
      enable: () => send("Network.enable"),
      getCookies: (params = {}) => send("Network.getCookies", params),
      setCookie: (params) => send("Network.setCookie", params),
    },
  };
}
