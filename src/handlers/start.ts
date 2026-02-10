import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";

export const composer = new Composer<MyContext>();

const kb = new InlineKeyboard()
    .text("Домашние задания", "homework/3/1")
    .row()
    .text("Посещаемость/оценки", "activity/0")
    .row()
    .text("Таблица лидеров", "leaderboard/stream")
    .row()
    .text("Ваши награды", "rewards/0")

export const start_command_handler = async (ctx: MyContext) => {
    const data = await ctx.ApiClient.getUserData();
    if (!data) {
        await ctx.reply(ctx.ApiClient.ErrorMessage)
        return
    }
    
    const coins = data.gaming_points[0]!.points
    const gems = data.gaming_points[1]!.points
    const text = `
Привет, ${data.full_name}!

Ваш ID: ${data.student_id}
Дата регистрации: ${new Date(data.registration_date).toLocaleDateString("ru-RU")}
Группа: ${data.group_name}

Количество топкоинов: ${coins}
Количество топгемов:  ${gems}
Количество топмани:  ${coins + gems}

Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.`

    if (ctx.callbackQuery) {
        await ctx.editMessageText(text, { reply_markup: kb })
    } else {
        await ctx.reply(text, { reply_markup: kb })
    }
};

composer.command("start", start_command_handler)
composer.callbackQuery("mm", start_command_handler)