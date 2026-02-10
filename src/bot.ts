import { Bot, webhookCallback, session } from "grammy";
import {hydrate, hydrateApi, hydrateContext} from "@grammyjs/hydrate";
import http from "node:http"
import { autoRetry } from "@grammyjs/auto-retry";
import { apiThrottler } from "@grammyjs/transformer-throttler";
import {
  conversations,
  createConversation,
} from "@grammyjs/conversations";

import type { MyApi, MyContext, SessionData } from "./types.js";
import { userLoader } from "./middlewares.js";
import { composer as startHandlers } from "./handlers/start.js";
import { composer as evaluatesHandlers } from "./handlers/evaluates.js";
import { composer as leaderboardsHandlers } from "./handlers/leaderboards.js";
import { auth } from "./handlers/auth.js";

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

bot.use(createConversation(auth, { plugins: [hydrate()]} ));

bot.on(["message:entities:bot_command", "callback_query"], userLoader);
bot.use(startHandlers);
bot.use(evaluatesHandlers);
bot.use(leaderboardsHandlers);

if (process.env.NODE_ENV === 'prod') {
  const server = http.createServer(webhookCallback(bot, 'http'));
  server.listen(4000);
  } else {
  bot.start(); 
}
console.log("Запущено!")