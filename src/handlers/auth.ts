import { ApiClient } from "../api/apiclient.js";
import { insertUser,  updateUser } from "../db/services/users.service.js";
import { clientsCache } from "../middlewares.js";
import type { MyContext, MyConversation, MyConversationContext } from "../types.js";
import { start_command_handler } from "./start.js";
import { setTimeout } from 'timers/promises';

export async function auth(conversation: MyConversation, ctx: MyConversationContext, update: boolean = false) {
    await ctx.reply(
        "Введите ваш логин и пароль от Top Academy в формате: <code>логин пароль</code>.\n" +
        "Для отмены введите <code>отмена</code>\n\n" +
        "Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.",
        { parse_mode: "HTML" }
    );

    while (true) {
        try {
            const { msg } = await conversation.waitFor("message:text", { 
                maxMilliseconds: 300_000 
            });
            const text = msg.text;

            if (text.toLowerCase() === "отмена") {
                await ctx.reply("Отменено!", { reply_parameters: { message_id: msg.message_id } });
                return;
            }

            const [login, password] = text.split(/\s+/);
            if (!login || !password) {
                await ctx.reply("Формат: <code>логин пароль</code>. Попробуйте еще раз:", { parse_mode: "HTML", reply_parameters: { message_id: msg.message_id } });
                continue;
            }

            let client = new ApiClient(login, password);
            const token = await conversation.external(() => client.login());
            
            if (!token) {
                await ctx.reply("Не удалось войти. Проверьте данные и попробуйте снова:", { reply_parameters: { message_id: msg.message_id } });
                continue;
            }

            client = new ApiClient(login, password, token)
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
            await start_command_handler(ctx as MyContext);
            return;

        } catch (error: any) {
            return;
        }
    }
}