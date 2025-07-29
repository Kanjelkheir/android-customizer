<script>
    const footerColumns = [
        {
            title: "Resources",
            links: [
                { text: "Custom ROMs", url: "/Custom_Rom" },
                { text: "Recovery Images", url: "#" },
                { text: "Kernels", url: "#" },
                { text: "Root Tools", url: "#" },
                { text: "Mods & Tweaks", url: "#" },
            ],
        },
        {
            title: "Guides",
            links: [
                { text: "Rooting Guide", url: "#" },
                { text: "Custom ROM Installation", url: "#" },
                { text: "Backup & Restore", url: "#" },
                { text: "Troubleshooting", url: "#" },
                { text: "FAQ", url: "#" },
            ],
        },
        {
            title: "Company",
            links: [
                { text: "About Us", url: "#" },
                { text: "Our Team", url: "#" },
                { text: "Privacy Policy", url: "#" },
                { text: "Terms of Service", url: "#" },
                { text: "Contact Us", url: "#" },
            ],
        },
    ];
=======
  // Just data: no HTML tags inside the array
  const footerColumns = [
    {
      title: "Resources",
      links: [
        { text: "Custom Rom", url: "https://droidify.netlify.app/Custom_Rom" },
        { text: "Recovery Images", url: "https://droidify.netlify.app/Recovery_Images" },
        { text: "Kernels", url: "https://droidify.netlify.app/Kernels" },
        { text: "Root Tools", url: "https://droidify.netlify.app/Root_Tools" },
        { text: "Mods & Tweaks", url: "https://droidify.netlify.app/Mods_and_Tweaks" },
      ],
    },
    {
      title: "Guides",
      links: [
        { text: "Rooting Guide", url: "https://droidify.netlify.app/Rooting_Guide" },
        { text: "Custom ROM Installation", url: "https://droidify.netlify.app/Custom_ROM_Installation" },
        { text: "Backup & Restore", url: "https://droidify.netlify.app/Backup_and_Restore" },
        { text: "Troubleshooting", url: "https://droidify.netlify.app/Troubleshooting" },
        { text: "FAQ", url: "https://droidify.netlify.app/FAQ" },
      ],
    },
    {
      title: "Company",
      links: [
        { text: "About Us", url: "https://droidify.netlify.app/About_Us" },
        { text: "Our Team", url: "https://droidify.netlify.app/Our_Team" },
        { text: "Privacy Policy", url: "https://droidify.netlify.app/Privacy_Policy" },
        { text: "Terms of Service", url: "https://droidify.netlify.app/Terms_of_Service" },
        { text: "Contact Us", url: "https://droidify.netlify.app/Contact_Us" },
      ],
    },
  ];

  const socialLinks = [
    { icon: "fab fa-facebook-f", url: "https://www.facebook.com/eliekh05", label: "Facebook" },
    { icon: "fab fa-x", url: "https://x.com/eliekh05", label: "Twitter" },
    { icon: "fab fa-github", url: "https://github.com/eliekh05/android-customizer", label: "GitHub" },
    { icon: "fab fa-telegram", url: "https://t.me/eliekh05", label: "Telegram" },
    { icon: "fab fa-whatsapp", url: "https://wa.me/+96179140038", label: "WhatsApp" }
  ];

  // Example smooth scroll function (adjust as needed)
  function smoothScrollTo(event) {
    event.preventDefault();
    const href = event.currentTarget.getAttribute('href');
    // Your smooth scroll logic here, or just navigate:
    window.open(href, "_blank"); // If external links open in new tab
  }
</script>

<footer id="contact" role="contentinfo" aria-label="Footer">
  <div class="container">
    <div class="footer-content">
      <div class="footer-column" role="region" aria-labelledby="footer-brand">
        <h3 id="footer-brand">AndroidCustomizer</h3>
        <p>
          Making Android customization accessible to everyone by
          eliminating the confusion and wasted time.
        </p>
        <div class="social-links">
          {#each socialLinks as social}
            <a href={social.url} aria-label={social.label} target="_blank" rel="noopener noreferrer" class="social-link">
              <i class={social.icon} aria-hidden="true"></i>
            </a>
          {/each}
        </div>
      </div>

      {#each footerColumns as column}
        <div class="footer-column" role="region" aria-labelledby={"footer-" + column.title.toLowerCase().replace(/\s+/g, '-')}>
          <h3 id={"footer-" + column.title.toLowerCase().replace(/\s+/g, '-')}>{column.title}</h3>
          <ul>
            {#each column.links as link}
              <li>
                <a href={link.url} class="footer-link" on:click={smoothScrollTo}>
                  {link.text}
                </a>
              </li>
            {/each}
          </ul>
        </div>
      {/each}
    </div>
    <div class="footer-bottom">
      <p>
        &copy; {new Date().getFullYear()} AndroidCustomizer. All rights reserved.
      </p>
    </div>
  </div>
</footer>

<style>
  /* Your styles remain unchanged */
</style>
