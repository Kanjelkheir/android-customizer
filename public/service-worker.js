const CACHE_NAME = 'android-customizer-cache-v1';
const urlsToCache = [
  '/',
  '/index.html',
  '/build/bundle.css',
  '/build/bundle.js',
  '/build/bundle.js.map',
  '/global.css',
  '/apple-touch-icon.png',
  '/favicon-96x96.png',
  '/favicon.ico',
  '/favicon.svg',
  '/site.webmanifest',
  '/manifest.json',
  '/icon-192.png',
  '/icon-512.png',
  '/maskable-icon-192.png',
  '/maskable-icon-512.png',
];

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => response || fetch(event.request))
  );
});

