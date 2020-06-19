import Discord from 'discord.js';
import { resolve as resolvePath } from 'path';

import { spawn } from './async-process';

type MessageListener = (client: Discord.Client, message: Discord.Message) => unknown;

export async function login(token: string, listener: MessageListener) {
  const client = new Discord.Client();
  client.on('message', (message: Discord.Message) => listener(client, message));
  await client.login(token);
}

const prefix = 'dc!';

const help = `
  \`\`\`
  Dischord
  ${prefix}help Dischordのヘルプを表示
  ${prefix}play [MML] MMLを音声ファイルに書き出し
  Dischord MML 文法
  以下の文字列を連ねて記述します。小文字のアルファベット部分はパラメータとして整数を入れます。
  CDEFGABRn ドレミファソラシと休符に対応しています。数字を後ろにつけるとn分音符を表現します。
  "."は付点音符を表現します。"&"で長さを連結すると2つの長さをタイで接続します。
  <> オクターブを上げ(下げ)ます。
  Tn テンポを後ろに表記された値に変更します。
  Vn 音量を変更します。デフォルトは50です。
  Ln デフォルトの音符の長さを変更します。
  @n 音色を変更します。以下は指定できる波形の一覧です。
  0: 矩形波(デューティ比50%), 1: 矩形波(25%), 2: 矩形波(12.5%), 3: 三角波, 4: ノコギリ波, 5: サイン波, 6: ホワイトノイズ, 7: 矩形波2(50%)
  Na,d,s,r ADSRエンベロープを設定します。
  Un,d n個の音をd%のデチューンで重ねて出力します。
  []n 括弧で囲んだ範囲をn回繰り返します。
  ()n 括弧で囲んだ範囲の音を同時に発音します。
  ; 複数の音を重ねるために、書き込み位置を先頭に戻します。
  \`\`\`
`
  .split('\n')
  .map(x => x.trim())
  .filter(x => x)
  .join('\n');

export async function ready(token: string) {
  await login(token, async (client, message) => {
    if (!client.user || client.user.id === message.author.id || message.author.bot) {
      return;
    }

    if (message.content.slice(0, prefix.length) !== prefix) {
      return;
    }

    const content = message.content.slice(prefix.length);
    const command = content.split(' ', 1)[0];
    const parameter = content.slice(command.length + 1);

    switch (command) {
      case 'play': {
        await message.channel.send('生成しています…');
        const child = spawn(resolvePath(process.cwd(), 'composer/target/debug/dischord_rust'));
        await child.writeStdin(parameter);
        const [result, err] = await child.end();

        if (err.length) {
          await message.channel.send(err.toString('utf-8'));
        } else {
          await message.channel.send('', { files: [{ name: 'result.wav', attachment: result }] });
        }
        break;
      }

      case 'help': {
        message.channel.send(help);
        break;
      }

      default: {
        message.channel.send('未知のコマンドです');
        break;
      }
    }
  });
}
