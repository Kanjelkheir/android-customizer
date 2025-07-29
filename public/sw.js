if (!self.define) {
  let e, c = {};
  const i = (i, s) => (i = new URL(i + ".js", s).href, c[i] || new Promise(c => {
    if ("document" in self) {
      const e = document.createElement("script");
      e.src = i, e.onload = c, document.head.appendChild(e)
    } else e = i, importScripts(i), c()
  }).then(() => {
    let e = c[i];
    if (!e) throw new Error(`Module ${i} didn’t register its module`);
    return e
  }));
  self.define = (s, r) => {
    const f = e || ("document" in self ? document.currentScript.src : "") || location.href;
    if (c[f]) return;
    let o = {};
    const b = e => i(e, f),
      n = {
        module: {
          uri: f
        },
        exports: o,
        require: b
      };
    c[f] = Promise.all(s.map(e => n[e] || b(e))).then(e => (r(...e), o))
  }
}

define(["./workbox-9a0b7ece"], function (workbox) {
  "use strict";

  self.addEventListener("message", e => {
    if (e.data && e.data.type === "SKIP_WAITING") self.skipWaiting();
  });

  workbox.precacheAndRoute([
    { url: "index.html", revision: "655ea5655d117eff8df4da141c2445cb" },
    { url: "apple-touch-icon.png", revision: "b30cd81f52af505424077a0b62d191e9" },
    { url: "favicon-96x96.png", revision: "7fcba9695736ed14d94b0c52bd3cdedc" },
    { url: "favicon.ico", revision: "37f8ae4acbc99a9e9fc6bb6e715ffe4a" },
    { url: "favicon.svg", revision: "f0316ea1a8ff633f1a9d54346b4664ff" },
    { url: "site.webmanifest", revision: "4fd4b1e97e80504f79a5b3f5e4fcc03e" },
    { url: "web-app-manifest-192x192.png", revision: "848f4a2cb68bbeedf2a531c5b1e11c19" },
    { url: "web-app-manifest-512x512.png", revision: "536eff01ed108d190372b3c85c0de3bc" },
    { url: "build/bundle.css", revision: "46c68de0f37e92c80b7c97aba2a489e2" },
    { url: "build/bundle.js", revision: "6734d6ec77bff140a46e5c981c3aeda4" }
  ], {
    ignoreURLParametersMatching: [/^utm_/, /^fbclid$/]
  });
});
