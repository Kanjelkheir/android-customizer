<script>
  import { onMount } from 'svelte';

  let isMobile = false;

  // Detect device type based on viewport width
  function checkDevice() {
    const width = window.innerWidth;
    isMobile = width <= 768; // phone or tablet breakpoint
  }

  function smoothScrollTo(event) {
    event.preventDefault();
    const targetId = event.currentTarget.getAttribute('href');
    const targetElement = document.querySelector(targetId);
    if (targetElement) {
      window.scrollTo({
        top: targetElement.offsetTop,
        behavior: 'smooth'
      });
    }
  }

  onMount(() => {
    checkDevice();
    window.addEventListener('resize', checkDevice);
    return () => window.removeEventListener('resize', checkDevice);
  });
</script>

<header>
  <div class="container header-container" class:is-mobile={isMobile}>
    <div class="logo" role="banner">
      <i class="fas fa-mobile-alt" aria-hidden="true"></i>
      <span>AndroidCustomizer</span>
    </div>
    <nav role="navigation" aria-label="Primary Navigation">
      <ul>
        <li><a href="https://droidify.netlify.app" on:click={smoothScrollTo}>Home</a></li>
        <li><a href="https://droidify.netlify.app/features" on:click={smoothScrollTo}>Features</a></li>
        <li><a href="https://droidify.netlify.app/devices" on:click={smoothScrollTo}>Devices</a></li>
        <li><a href="https://droidify.netlify.app/guide" on:click={smoothScrollTo}>Guides</a></li>
        <li><a href="https://droidify.netlify.app/forum" on:click={smoothScrollTo}>Community</a></li>
        <li><a href="https://droidify.netlify.app/contact" on:click={smoothScrollTo}>Contact</a></li>
      </ul>
    </nav>
  </div>
</header>

<style>
  header {
    background-color: var(--dark-color);
    color: white;
    padding: 1rem 0;
    position: sticky;
    top: 0;
    z-index: 100;
    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
  }

  .header-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  /* Layout change when mobile */
  .header-container.is-mobile {
    flex-direction: column;
    text-align: center;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: bold;
    display: flex;
    align-items: center;
  }

  .logo i {
    color: var(--accent-color);
    margin-right: 0.5rem;
  }

  nav ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
  }

  /* For mobile, center and wrap nav links */
  .header-container.is-mobile nav ul {
    justify-content: center;
    flex-wrap: wrap;
    margin-top: 1rem;
  }

  nav ul li {
    margin-left: 1.5rem;
  }

  /* For first item no margin */
  nav ul li:first-child {
    margin-left: 0;
  }

  nav ul li a {
    color: white;
    text-decoration: none;
    transition: color 0.3s;
    /* Minimum tap target size */
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 48px;
    min-height: 48px;
    padding: 0 0.5rem;
    font-size: 1rem;
    user-select: none;
  }

  nav ul li a:hover,
  nav ul li a:focus {
    color: var(--accent-color);
    outline: none;
  }

  @media (max-width: 768px) {
    nav ul li {
      margin: 0 0.75rem;
    }
  }
</style>
