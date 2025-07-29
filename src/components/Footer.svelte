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

    const socialLinks = [
        { icon: "fab fa-facebook-f", url: "#", label: "Facebook" },
        { icon: "fab fa-twitter", url: "#", label: "Twitter" },
        { icon: "fab fa-github", url: "#", label: "GitHub" },
        { icon: "fab fa-telegram", url: "#", label: "Telegram" },
    ];
</script>

<footer id="contact">
    <div class="container">
        <div class="footer-content">
            <div class="footer-column">
                <h3>AndroidCustomizer</h3>
                <p>
                    Making Android customization accessible to everyone by
                    eliminating the confusion and wasted time.
                </p>
                <div class="social-links">
                    {#each socialLinks as social}
                        <a href={social.url} aria-label={social.label}
                            ><i class={social.icon}></i></a
                        >
                    {/each}
                </div>
            </div>

            {#each footerColumns as column}
                <div class="footer-column">
                    <h3>{column.title}</h3>
                    <ul>
                        {#each column.links as link}
                            <li><a href={link.url}>{link.text}</a></li>
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
    footer {
        background-color: var(--dark-color);
        color: white;
        padding: 3rem 0 1.5rem;
    }

    .footer-content {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 2rem;
        margin-bottom: 2rem;
    }

    .footer-column h3 {
        margin-bottom: 1.5rem;
        position: relative;
    }

    .footer-column h3::after {
        content: "";
        position: absolute;
        left: 0;
        bottom: -0.5rem;
        width: 50px;
        height: 2px;
        background-color: var(--accent-color);
    }

    .footer-column ul {
        list-style: none;
    }

    .footer-column ul li {
        margin-bottom: 0.8rem;
    }

    .footer-column ul li a {
        color: #ccc;
        text-decoration: none;
        transition: color 0.3s;
    }

    .footer-column ul li a:hover {
        color: white;
    }

    .social-links {
        display: flex;
        gap: 1rem;
        margin-top: 1rem;
    }

    .social-links a {
        color: white;
        background-color: rgba(255, 255, 255, 0.1);
        width: 40px;
        height: 40px;
        border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: background-color 0.3s;
    }

    .social-links a:hover {
        background-color: var(--primary-color);
    }

    .footer-bottom {
        text-align: center;
        padding-top: 2rem;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
    }
</style>
