import { Bot, webhookCallback, session } from "grammy";
import { hydrate, hydrateApi, hydrateContext } from "@grammyjs/hydrate";
import http from "node:http"
import { autoRetry } from "@grammyjs/auto-retry";
import { apiThrottler } from "@grammyjs/transformer-throttler";
import { conversations, createConversation } from "@grammyjs/conversations";

import type { MyApi, MyContext, SessionData } from "./types.js";
import { userLoader } from "./middlewares.js";
import { handlers } from "./handlers/index.js"; 
import { auth } from "./handlers/auth.js";
import { cancelHandler } from "./utils.js";
import { limit } from "@grammyjs/ratelimiter";

const bot = new Bot<MyContext, MyApi>(process.env.TOKEN!,);
const throttler = apiThrottler();

function initial(): SessionData {
  return { evaluations: [], evaluateCooldown: 0 };
}
bot.use(session({ initial }));
bot.use(hydrateContext());
bot.use(conversations());
bot.api.config.use(hydrateApi());
bot.api.config.use(autoRetry());
bot.api.config.use(throttler);
bot.use(limit({onLimitExceeded: async (ctx) => {
  if (ctx.callbackQuery) {
    await ctx.answerCallbackQuery({ text: "Тише будь", show_alert: true })
  } else {
    await ctx.reply("Тише будь")
  }
}, timeFrame: 1500, limit: 1}));

bot.use(createConversation(auth, { plugins: [hydrate()]} ));
bot.on(["message:entities:bot_command", "callback_query"], userLoader);
bot.command("relogin", async (ctx) => {await ctx.conversation.enter("auth", true)})
bot.callbackQuery("cancel", cancelHandler)
bot.use(handlers);

bot.catch((err) => {
  console.error('Bot error:', err);
});

if (process.env.NODE_ENV === 'prod') {
  const server = http.createServer(webhookCallback(bot, 'http'));
  server.on('error', (error) => {
    console.error('Server error:', error);
  });
  server.listen(4000, () => {
    console.log('Webhook server listening on port 4000');
  });
} else {
  bot.start(); 
}
console.log("Запущено!")