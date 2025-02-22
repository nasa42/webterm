# Deploy Webterm Frontend

## Build the frontend

Webterm frontend is a static site built using [Astro](https://github.com/withastro/astro). Ensure you've Node installed
or install it
with [these instructions](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). Node version should
match the version mentioned in [.node-version](../frontend/.node-version) file.

```bash
git clone https://github.com/nasa42/webterm
cd frontend
npm install
# if you don't want the frontend to use default relays, set custom ones before running the build
export PUBLIC_DEFAULT_RELAYS=relay1.example.com,relay2.example.com
npm run build
```

## Deploy the frontend

1. Upload the static files produced in `frontend/dist` directory to your favourite static site provider.
