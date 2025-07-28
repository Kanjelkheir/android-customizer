const CACHE_NAME = 'android-customizer-cache-v1';
const urlsToCache = [
  '/',
  'index.html',
  'global.css',
  '/build/bundle.css',
  '/build/bundle.js',
  'favicon-32x32.ico',
  'icons/icon-192.png',
  'icons/icon-512.png',
];

// Install event: cache all app shell files
self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
      .then(() => self.skipWaiting())
  );
});

// Activate event: clear old caches
self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys().then(keys =>
      Promise.all(
        keys.filter(key => key !== CACHE_NAME)
          .map(key => caches.delete(key))
      )
    ).then(() => self.clients.claim())
  );
});

// Fetch event: serve cached resources when offline
self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request).then(cachedResponse => {
      return cachedResponse || fetch(event.request);
    }).catch(() => caches.match('/index.html')) // fallback to index.html offline
  );
});
