import * as Discord from 'discord.js';
import { ReadableStreamBuffer } from 'stream-buffers';

const queue: [Discord.VoiceChannel, Buffer][] = [];

export function playBufferOnChannel(channel: Discord.VoiceChannel, stream: Buffer) {
  async function processQueue() {
    if (queue.length === 0) {
      return;
    }

    const connection = await queue[0][0].join();
    const stream = new ReadableStreamBuffer();
    stream.put(queue[0][1]);
    stream.stop();
    const dispatcher = connection.play(stream, { type: 'converted', volume: 0.2 });
    dispatcher.on('finish', () => {
      connection.disconnect();
      queue.shift();
      setTimeout(processQueue, 1000);
    });
  }

  queue.push([channel, stream]);
  if (queue.length === 1) {
    processQueue();
  }
}
