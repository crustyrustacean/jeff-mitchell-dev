# Build with taxus, then deploy to Cloudflare Pages
taxus build
npx wrangler pages deploy dist/ --project-name=jeff-mitchell-dev
