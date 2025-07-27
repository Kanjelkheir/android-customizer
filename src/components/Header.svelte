<script>
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
</script>

<header>
  <div class="container header-container">
    <div class="logo">
      <i class="fas fa-mobile-alt"></i>
      <span>AndroidCustomizer</span>
    </div>
    <nav>
      <ul>
        <li><a href="https://droidify.netlify.app">Home</a></li>
        <li><a href="https://droidify.netlify.app/features">Features</a></li>
        <li><a href="https://droidify.netlify.app/devices">Devices</a></li>
        <li><a href="https://droidify.netlify.app/guide">Guides</a></li>
        <li><a href="https://droidify.netlify.app/forum">Community</a></li>
        <li><a href="https://droidify.netlify.app/contact">Contact</a></li>
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
  }

  nav ul li {
    margin-left: 1.5rem;
  }

  nav ul li a {
    color: white;
    text-decoration: none;
    transition: color 0.3s;
  }

  nav ul li a:hover {
    color: var(--accent-color);
  }

  @media (max-width: 768px) {
    .header-container {
      flex-direction: column;
      text-align: center;
    }

    nav ul {
      margin-top: 1rem;
      justify-content: center;
    }

    nav ul li {
      margin: 0 0.75rem;
    }
  }
</style>
