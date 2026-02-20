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

bot.catch((err) => {
  console.error('Bot error:', err);
});

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

if (process.env.NODE_ENV === 'prod') {
  const handleUpdate = webhookCallback<MyContext, "http">(bot, "http");

  const server = http.createServer((req, res) => {
    if (req.method === "GET" && (req.url === "/" || req.url === "/health")) {
      res.writeHead(200, { "Content-Type": "text/plain" });
      res.end("OK");
      return;
    }

    if (req.method !== "POST") {
      res.writeHead(405, { "Content-Type": "text/plain" });
      res.end("Method Not Allowed");
      return;
    }

    const contentType = req.headers["content-type"];
    if (!contentType || !contentType.includes("application/json")) {
      res.writeHead(400, { "Content-Type": "text/plain" });
      res.end("Bad Content-Type");
      return;
    }

    handleUpdate(req, res).catch((err) => {
      console.error("Webhook handler error:", err);
      if (!res.headersSent) {
        res.writeHead(500);
        res.end("Internal Server Error");
      }
    });
  });

  server.on("error", (error) => {
    console.error("Server error:", error);
  });

  server.listen(4000, "127.0.0.1", () => {
    console.log(`Webhook server listening on port 4000`);
  });
} else {
  bot.start();
}

console.log("Запущено!")