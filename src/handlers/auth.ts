import type { Conversation } from "@grammyjs/conversations";
import { ApiClient } from "../api/apiclient.js";
import { insertUser,  updateUser } from "../db/services/users.service.js";
import { clientsCache } from "../middlewares.js";
import type { MyContext, MyConversationContext } from "../types.js";
import { start_command_handler } from "./start.js";
import { setTimeout } from 'timers/promises';
import { except } from "drizzle-orm/gel-core";

export async function auth(conversation: Conversation<MyContext, MyConversationContext>, ctx: MyContext, update: boolean = false) {
    await ctx.reply(
        "Введите ваш логин и пароль от Top Academy в формате: <code>логин пароль</code>.\n" +
        "Для отмены введите <code>отмена</code>\n\n" +
        "Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.",
        { parse_mode: "HTML" }
    );

    while (true) {
        let msg
        try {
            const newCtx = await conversation.waitFor("message:text", { 
                maxMilliseconds: 300_000 
            });
            msg = newCtx.msg
        }
        catch {
            return
        }

        if (msg.text.toLowerCase() === "отмена") {
            await ctx.reply("Отменено!", { reply_parameters: { message_id: msg.message_id } });
            return;
        }

        const [login, password] = msg.text.split(/\s+/);
        if (!login || !password) {
            await ctx.reply("Формат: <code>логин пароль</code>. Попробуйте еще раз:", { parse_mode: "HTML", reply_parameters: { message_id: msg.message_id } });
            continue;
        }

        let token
        try {
            const client = new ApiClient(login, password);
            token = await conversation.external(() => client.login());
        } catch {
            await ctx.reply("Не удалось войти. Проверьте данные и попробуйте снова:", { reply_parameters: { message_id: msg.message_id } });
            continue;
        }

        const client = new ApiClient(login, password, token)
        ctx.ApiClient = client

        await conversation.external(async () => {
            if (update) {
                await updateUser(msg.from.id, login, password, token)
            } else {
                await insertUser(msg.from.id, login, password, token)
            }
            clientsCache.set(msg.from.id, client);
        });

        await ctx.reply("Вы успешно авторизованы! Перехожу в главное меню...", { reply_parameters: { message_id: msg.message_id } });
        await setTimeout(1500);
        await start_command_handler(ctx);
        return;
    }
}