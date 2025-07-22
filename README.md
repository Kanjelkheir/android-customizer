_**⚠️ WIP ⚠️**_

# AndroidCustomizer Svelte App

This is a Svelte implementation of the AndroidCustomizer website, a platform for Android device customization resources.

## Description

AndroidCustomizer is a one-stop solution for Android customization. The platform provides all the resources, tools, and guides users need for any Android device - ready to use in minutes, not days.

## Features

- Comprehensive database of Android ROMs, kernels, and mods
- Device-specific guides for rooting and customization
- Fast direct downloads for firmware files and tools
- Active community support
- Troubleshooting tools and resources
- Update notifications for new ROMs and tools

## Getting Started

### Prerequisites

- Node.js (version 14 or higher recommended)
- npm or yarn package manager

### Installation

1. Clone this repository
   ```bash
   git clone <repository-url>
   cd AndroidSvelte
   ```

2. Install dependencies
   ```bash
   npm install
   # or
   yarn install
   ```

3. Start the development server
   ```bash
   npm run dev
   # or
   yarn dev
   ```

4. Build for production
   ```bash
   npm run build
   # or
   yarn build
   ```

## Project Structure

```
AndroidSvelte/
├── public/
│   ├── build/       # Generated bundle files
│   ├── favicon.png  # Site favicon
│   ├── global.css   # Global CSS styles
│   └── index.html   # HTML entry point
├── src/
│   ├── components/  # Svelte components
│   │   ├── Header.svelte
│   │   ├── Hero.svelte
│   │   ├── Features.svelte
│   │   ├── HowItWorks.svelte
│   │   ├── PopularDevices.svelte
│   │   ├── CTA.svelte
│   │   └── Footer.svelte
│   ├── App.svelte   # Main app component
│   └── main.js      # JavaScript entry point
├── package.json     # Project dependencies and scripts
└── rollup.config.js # Rollup configuration
```

## Technologies Used

- [Svelte](https://svelte.dev/) - Frontend framework
- [Rollup](https://rollupjs.org/) - Module bundler
- [FontAwesome](https://fontawesome.com/) - Icon library
- [Google Fonts](https://fonts.google.com/) - Web fonts

## Converting from HTML to Svelte

This project is a conversion of a static HTML website to a component-based Svelte application. The conversion process involved:

1. Breaking down the HTML into reusable components
2. Converting inline JavaScript to Svelte reactive declarations and event handlers
3. Scoping CSS styles to their respective components
4. Implementing data flow between components

## License

This project is licensed under the MIT License - see the LICENSE file for details.
