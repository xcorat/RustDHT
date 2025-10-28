# RustDHT Website

This directory contains the source code for the RustDHT project website, built with [11ty (Eleventy)](https://www.11ty.dev/) static site generator.

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run serve

# Build for production
npm run build
```

## Structure

```
pages/
├── .eleventy.js           # 11ty configuration
├── package.json           # Dependencies and scripts
├── _includes/             # Templates and layouts
│   ├── base.liquid        # Base HTML template
│   ├── nav.liquid         # Navigation component
│   ├── footer.liquid      # Footer component
│   └── card.liquid        # Reusable card component
├── _data/                 # Site data
│   ├── site.json          # Site metadata
│   ├── projects.json      # External projects data
│   └── roadmap.json       # Roadmap milestones
├── assets/                # Static assets
│   ├── css/main.css       # Main stylesheet
│   └── js/cards.js        # Card interactions
├── docs/                  # Build output (GitHub Pages)
├── index.md               # Homepage
├── why-decentralized.md   # Why Decentralized page
├── technical.md           # Technical Overview page
├── use-cases.md           # Use Cases page
├── community.md           # Community page
└── roadmap.md             # Roadmap page
```

## Development

### Local Development

1. Install Node.js (v16 or later)
2. Run `npm install` to install dependencies
3. Run `npm run serve` to start the development server
4. Open `http://localhost:8080` in your browser

### Building for Production

```bash
npm run build
```

This generates the static site in the `docs/` directory, which is configured for GitHub Pages deployment.

### Adding Content

- **Pages**: Create new `.md` files in the root directory
- **Data**: Add JSON files to `_data/` directory
- **Assets**: Add CSS, JS, images to `assets/` directory
- **Templates**: Add Liquid templates to `_includes/` directory

### Styling

The site uses a custom CSS design system with:
- CSS custom properties for theming
- Mobile-first responsive design
- Dark theme with rust orange accents
- Smooth animations and hover effects

### JavaScript

Minimal JavaScript for:
- Card click navigation
- Mobile menu toggle
- Smooth scrolling
- Intersection Observer animations

## Deployment

The site is configured to deploy to GitHub Pages:

1. Build output goes to `docs/` directory
2. `.nojekyll` file prevents Jekyll processing
3. GitHub Pages serves from `docs/` folder
4. Custom domain can be configured via CNAME file

### GitHub Actions (Optional)

For automatic deployment on push:

```yaml
name: Build and Deploy
on:
  push:
    branches: [ main ]
jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions/setup-node@v2
      with:
        node-version: '16'
    - run: npm ci
    - run: npm run build
    - uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./docs
```

## Content Strategy

### Homepage
- Hero section with clear value proposition
- Feature cards with hover effects and navigation
- Projects showcase with external links
- Roadmap overview and call-to-action

### Subpages
- **Why Decentralized**: Vision and principles
- **Technical**: Architecture and implementation
- **Use Cases**: Real-world applications
- **Community**: How to get involved
- **Roadmap**: Development timeline

### Footnotes
- Markdown footnotes link to detailed documentation
- References to wiki/main docs for technical details
- External links to related projects and resources

## Design System

### Colors
- Background: Dark (#0a0a0a, #1a1a1a, #2a2a2a)
- Text: Light (#f5f5f5, #a0a0a0, #666666)
- Accent: Rust orange (#ff6b35, #ff8c42)
- Borders: Gray (#333333, #555555)

### Typography
- System font stack for performance
- Clear hierarchy with consistent spacing
- Readable line heights and font sizes
- Responsive typography scaling

### Components
- Cards with hover effects and smooth transitions
- Navigation with mobile-friendly hamburger menu
- Footer with organized link sections
- Buttons with consistent styling

## Performance

- Minimal JavaScript and CSS
- System fonts (no web font loading)
- Optimized images and assets
- Fast static site generation
- CDN-friendly output

## Accessibility

- Semantic HTML structure
- Proper heading hierarchy
- Alt text for images
- Keyboard navigation support
- Color contrast compliance
- Screen reader friendly

## Browser Support

- Modern browsers (Chrome, Firefox, Safari, Edge)
- Mobile browsers (iOS Safari, Chrome Mobile)
- Progressive enhancement approach
- Graceful degradation for older browsers

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `npm run serve`
5. Build with `npm run build`
6. Submit a pull request

### Content Guidelines

- Write in clear, accessible language
- Include examples and use cases
- Link to relevant documentation
- Use consistent formatting and style
- Test on mobile devices

### Technical Guidelines

- Follow existing code style
- Test changes across browsers
- Optimize for performance
- Maintain accessibility standards
- Document significant changes

## License

This website is part of the RustDHT project and shares its open-source license.
