// Card interactions and navigation
document.addEventListener('DOMContentLoaded', function() {
    // Handle card clicks for navigation
    const cards = document.querySelectorAll('.card[data-url]');
    
    cards.forEach(card => {
        card.addEventListener('click', function(e) {
            e.preventDefault();
            const url = this.getAttribute('data-url');
            
            if (url) {
                // Check if it's an external URL
                if (url.startsWith('http')) {
                    window.open(url, '_blank', 'noopener,noreferrer');
                } else {
                    // Internal navigation
                    window.location.href = url;
                }
            }
        });
        
        // Add keyboard support
        card.addEventListener('keydown', function(e) {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                this.click();
            }
        });
        
        // Make cards focusable
        card.setAttribute('tabindex', '0');
        card.setAttribute('role', 'button');
    });
    
    // Mobile navigation toggle
    const navToggle = document.getElementById('nav-toggle');
    const navMenu = document.getElementById('nav-menu');
    
    if (navToggle && navMenu) {
        navToggle.addEventListener('click', function() {
            navMenu.classList.toggle('nav-menu--open');
            this.classList.toggle('nav-toggle--open');
        });
    }
    
    // Smooth scroll for anchor links
    const anchorLinks = document.querySelectorAll('a[href^="#"]');
    
    anchorLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                const navHeight = document.querySelector('.nav').offsetHeight;
                const targetPosition = targetElement.offsetTop - navHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });
    
    // Add scroll effect to navigation
    let lastScrollTop = 0;
    const nav = document.querySelector('.nav');
    
    window.addEventListener('scroll', function() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        
        if (scrollTop > lastScrollTop && scrollTop > 100) {
            // Scrolling down
            nav.style.transform = 'translateY(-100%)';
        } else {
            // Scrolling up
            nav.style.transform = 'translateY(0)';
        }
        
        lastScrollTop = scrollTop <= 0 ? 0 : scrollTop;
    }, false);
    
    // Intersection Observer for animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animate-in');
            }
        });
    }, observerOptions);
    
    // Observe cards and sections for animation
    const animatedElements = document.querySelectorAll('.card, .section');
    animatedElements.forEach(el => observer.observe(el));
});

// Add CSS for mobile navigation
const mobileNavStyles = `
@media (max-width: 767px) {
    .nav-menu {
        position: fixed;
        top: 64px;
        left: 0;
        right: 0;
        background-color: var(--color-bg);
        border-bottom: 1px solid var(--color-border);
        padding: var(--space-4);
        transform: translateY(-100%);
        opacity: 0;
        visibility: hidden;
        transition: all var(--transition-base);
        flex-direction: column;
        align-items: flex-start;
        gap: var(--space-4);
    }
    
    .nav-menu--open {
        transform: translateY(0);
        opacity: 1;
        visibility: visible;
        display: flex;
    }
    
    .nav-toggle--open .nav-toggle-line:nth-child(1) {
        transform: rotate(45deg) translate(5px, 5px);
    }
    
    .nav-toggle--open .nav-toggle-line:nth-child(2) {
        opacity: 0;
    }
    
    .nav-toggle--open .nav-toggle-line:nth-child(3) {
        transform: rotate(-45deg) translate(7px, -6px);
    }
}

.animate-in {
    animation: fadeInUp 0.6s ease forwards;
}

@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
`;

// Inject mobile navigation styles
const styleSheet = document.createElement('style');
styleSheet.textContent = mobileNavStyles;
document.head.appendChild(styleSheet);
