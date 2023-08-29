# Bluesky Worker

Bluesky Worker is a Cloudflare Workers application that allows seamless posting of messages to Bluesky using a webhook-like approach.

## Features

- **Webhook-like functionality**: Bluesky Worker acts as a Webhook, allowing you to send POST requests with the desired content to a specified URL, which will then be posted to Bluesky.
- **Cloudflare Workers**: Powered by Cloudflare Workers, Bluesky Worker ensures scalability, reliability, and low-latency performance.
- **Rust implementation**: Bluesky Worker is implemented in Rust, a fast and memory-safe programming language, ensuring high performance and security.
- **Post-only functionality**: Bluesky Worker focuses solely on the posting feature, making it lightweight and straightforward.

## Setup

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/bridge-y/bluesky-worker)

1. Create your project by clicking on `Use this template` button (or fork this repository) and clone into your local host.

2. Execute the following commands.

   ```bash
   npx wrangler dev

   # deploy your worker globally to Cloudflare Workers.
   npx wrangler publish
   ```

3. Set up the required secrets.

   Execute the following command:

   ```bash
   npx wrangler put <secret>
   ```

   Secrets:

   - `REQUEST_PATH`: A randomly generated value.
   - `FULL_USERNAME`: Your Bluesky handle (e.g. \<hoge\>.bluesky.social).
   - `PASSWORD`: Your account password.  
     I recommend using App passwords for added security.

## Usage

```bash
curl -X POST -d '{"text": "test"}' -H "Content-Type: application/json" https://<your worker domain>/<REQUEST_PATH>
```

## License

This project is licensed under the MIT License.

## Disclaimer

Bluesky Worker is a third-party application and is not affiliated with Bluesky or Cloudflare. Use it at your own risk.
