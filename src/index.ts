import { ready } from './bot';

async function main() {
  console.log('Dischord-rust v0.0.1');

  const token = process.env.DISCHORD_TOKEN;

  if (!token) {
    console.log('DISCHORD_TOKEN is not present');
    process.exit(1);
  }

  await ready(token);
}

main();
