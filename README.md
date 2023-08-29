# Bluesky Worker

Bluesky Worker is a Cloudflare Workers application that allows seamless posting of messages to Bluesky using a webhook-like approach.

## Features

- **Webhook-like functionality**: Bluesky Worker acts as a Webhook, allowing you to send POST requests with the desired content to a specified URL, which will then be posted to Bluesky.
- **Cloudflare Workers**: Powered by Cloudflare Workers, Bluesky Worker ensures scalability, reliability, and low-latency performance.
- **Rust implementation**: Bluesky Worker is implemented in Rust, a fast and memory-safe programming language, ensuring high performance and security.
- **Post-only functionality**: Bluesky Worker focuses solely on the posting feature, making it lightweight and straightforward.

## Setup

1. Click the button below and follow the on-screen instructions to proceed with the setup.

   [![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/bridge-y/bluesky-worker)

2. Within the Cloudflare Workers dashboard, register the following secrets as variables.

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
