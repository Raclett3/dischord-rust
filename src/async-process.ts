import { ChildProcessWithoutNullStreams, spawn as spawnChild } from 'child_process';
import { Readable } from 'stream';

export class AsyncChildProcess {
  constructor(public stream: ChildProcessWithoutNullStreams) {}

  public async writeStdin(chunk: string | Buffer) {
    return await new Promise<void>((resolve, reject) => {
      this.stream.stdin.write(chunk, err => {
        if (err) {
          reject(err);
          return;
        }

        resolve();
      });
    });
  }

  public async end() {
    return await new Promise<[Buffer, Buffer]>(resolve => {
      this.stream.stdin.end(Buffer.alloc(0), async () => {
        resolve(await Promise.all([this.readStream(this.stream.stdout), this.readStream(this.stream.stderr)]));
      });
    });
  }

  private async readStream(stream: Readable) {
    return await new Promise<Buffer>((resolve, reject) => {
      let result = Buffer.alloc(0);
      stream.on('error', err => {
        reject(err);
      });

      stream.on('data', chunk => {
        if (typeof chunk === 'string') {
          result = Buffer.concat([result, Buffer.from(chunk)]);
        } else if (chunk instanceof Buffer) {
          result = Buffer.concat([result, chunk]);
        }
      });

      stream.on('end', () => {
        resolve(result);
      });
    });
  }
}

export function spawn(command: string, args: string[] = []): AsyncChildProcess {
  return new AsyncChildProcess(spawnChild(command, args));
}
