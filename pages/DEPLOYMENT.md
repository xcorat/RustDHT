# Website Deployment Guide

This guide covers deploying the RustDHT website to GitHub Pages.

## Quick Deployment

1. **Build the site:**
   ```bash
   cd pages/
   npm install
   npm run build
   ```

2. **Commit and push:**
   ```bash
   git add .
   git commit -m "Add website build"
   git push origin main
   ```

3. **Configure GitHub Pages:**
   - Go to repository Settings > Pages
   - Source: Deploy from a branch
   - Branch: main
   - Folder: /docs
   - Save

## File Structure

The build process creates:
```
docs/                    # GitHub Pages serves from here
├── index.html          # Homepage
├── why-decentralized/  # Subpages
├── technical/
├── use-cases/
├── community/
├── roadmap/
├── assets/             # CSS and JS
└── .nojekyll          # Prevents Jekyll processing
```

## Development Workflow

### Local Development
```bash
cd pages/
npm run serve           # Start dev server at localhost:8080
```

### Production Build
```bash
npm run build          # Generate static site in docs/
```

### Content Updates
1. Edit markdown files in `pages/`
2. Update data files in `pages/_data/`
3. Rebuild with `npm run build`
4. Commit and push changes

## GitHub Pages Configuration

### Repository Settings
- **Source**: Deploy from a branch
- **Branch**: main (or your default branch)
- **Folder**: /docs
- **Custom domain**: Optional (add CNAME file)

### Build Process
The site is built locally and committed to the repository. GitHub Pages serves the static files from the `docs/` directory without additional processing.

### .nojekyll File
The `.nojekyll` file in the `docs/` directory prevents GitHub from processing the site with Jekyll, allowing 11ty's output to be served directly.

## Custom Domain (Optional)

1. **Add CNAME file:**
   ```bash
   echo "yourdomain.com" > pages/CNAME
   npm run build
   ```

2. **Configure DNS:**
   - Add CNAME record pointing to `username.github.io`
   - Or A records pointing to GitHub Pages IPs

3. **Enable HTTPS:**
   - GitHub Pages automatically provides SSL certificates
   - May take a few minutes to provision

## Automated Deployment (Optional)

### GitHub Actions Workflow

Create `.github/workflows/deploy.yml`:

```yaml
name: Build and Deploy Website
on:
  push:
    branches: [ main ]
    paths: [ 'pages/**' ]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v3
      
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: pages/package-lock.json
        
    - name: Install dependencies
      run: |
        cd pages
        npm ci
        
    - name: Build website
      run: |
        cd pages
        npm run build
        
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./pages/docs
        cname: yourdomain.com  # Optional
```

This workflow:
- Triggers on pushes to main branch that modify `pages/` directory
- Installs dependencies and builds the site
- Deploys to GitHub Pages automatically

## Performance Optimization

### Build Optimization
- Minified CSS and JavaScript
- Optimized images and assets
- Efficient HTML structure
- Fast static site generation

### CDN Benefits
GitHub Pages provides global CDN distribution:
- Fast loading worldwide
- Automatic caching
- HTTPS by default
- High availability

### Monitoring
- Use GitHub Pages insights for traffic analytics
- Monitor Core Web Vitals with tools like PageSpeed Insights
- Set up uptime monitoring if using custom domain

## Troubleshooting

### Common Issues

**Build Fails:**
- Check Node.js version (requires v16+)
- Verify all dependencies are installed
- Check for syntax errors in templates

**Pages Not Updating:**
- Ensure changes are committed to correct branch
- Check GitHub Pages build status in repository settings
- Clear browser cache
- Verify .nojekyll file is present

**Styling Issues:**
- Check CSS file paths in generated HTML
- Verify assets are copied to docs/ directory
- Test locally with `npm run serve`

**404 Errors:**
- Check internal link paths
- Ensure all referenced files exist
- Verify GitHub Pages is serving from correct directory

### Debug Steps

1. **Local Testing:**
   ```bash
   cd pages/
   npm run build
   npx http-server docs/  # Test built site locally
   ```

2. **Check Generated Files:**
   - Verify HTML files are generated correctly
   - Check asset paths and file structure
   - Validate HTML and CSS

3. **GitHub Pages Status:**
   - Check repository Settings > Pages for build status
   - Review any error messages
   - Verify branch and folder settings

## Security Considerations

### Content Security
- All content is static (no server-side processing)
- No sensitive data in repository
- External links use `rel="noopener"`

### Dependencies
- Regular dependency updates via npm audit
- Use npm ci for reproducible builds
- Pin dependency versions in package-lock.json

### HTTPS
- GitHub Pages provides automatic HTTPS
- Custom domains get free SSL certificates
- All links should use HTTPS where possible

## Maintenance

### Regular Updates
- Update npm dependencies monthly
- Review and update content quarterly
- Monitor for broken external links
- Update roadmap and project status

### Content Management
- Keep documentation in sync with code
- Update project data and milestones
- Refresh external project information
- Maintain consistent style and tone

### Performance Monitoring
- Regular PageSpeed Insights checks
- Monitor loading times and Core Web Vitals
- Optimize images and assets as needed
- Review and minimize JavaScript usage

## Backup and Recovery

### Repository Backup
- GitHub provides built-in repository backup
- Consider additional backup to other Git providers
- Export important content to external formats

### Content Recovery
- All source files are in version control
- Build process is reproducible
- Documentation includes all necessary steps
- No external dependencies for core functionality

---

The website is now ready for deployment to GitHub Pages. The build process generates a complete static site that can be served from any web server or CDN.
