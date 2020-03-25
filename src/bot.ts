import Discord from 'discord.js';

type MessageListener = (client: Discord.Client, message: Discord.Message) => any;

export async function login(token: string, listener: MessageListener) {
  const client = new Discord.Client();
  client.on('message', (message: Discord.Message) => listener(client, message));
  await client.login(token);
}

export async function ready(token: string) {
  await login(token, (client, message) => {
    if (!client.user || client.user.id === message.author.id || message.author.bot) {
      return;
    }

    if (message.content.toLowerCase().includes('ping')) {
      message.channel.send('Pong!');
    }
  });
}
