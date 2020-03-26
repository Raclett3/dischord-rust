import Discord from 'discord.js';
import { resolve as resolvePath } from 'path';

import { spawn } from './async-process';

type MessageListener = (client: Discord.Client, message: Discord.Message) => any;

export async function login(token: string, listener: MessageListener) {
  const client = new Discord.Client();
  client.on('message', (message: Discord.Message) => listener(client, message));
  await client.login(token);
}

export async function ready(token: string) {
  await login(token, async (client, message) => {
    if (!client.user || client.user.id === message.author.id || message.author.bot) {
      return;
    }

    const prefix = 'dc!';

    if (message.content.slice(0, prefix.length) !== prefix) {
      return;
    }

    const command = message.content.slice(prefix.length);
    const child = spawn(resolvePath(process.cwd(), 'composer/target/debug/dischord_rust'));
    await child.writeStdin(command);
    const result = await child.end();

    await message.channel.send('', { files: [{ name: 'result.txt', attachment: result }] });
  });
}
