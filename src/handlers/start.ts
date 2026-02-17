import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";

export const composer = new Composer<MyContext>();

const kb = new InlineKeyboard()
    .text("Домашние задания", "homework/3/1")
    .style("primary")
    .row()
    .text("Посещаемость/оценки", "activity/0")
    .style("primary")
    .row()
    .text("Таблица лидеров", "leaderboard/stream")
    .style("primary")
    .row()
    .text("Ваши награды", "rewards/0")
    .style("primary")

export const start_command_handler = async (ctx: MyContext) => {
    const data = await ctx.ApiClient.getUserData();
    if (!data) {
        await ctx.reply(ctx.ApiClient.ErrorMessage)
        return
    }
    
    const coins = data.gaming_points[0]!.points
    const gems = data.gaming_points[1]!.points
    const text = 
        `Привет, ${data.full_name}!\n\n` +

        `Ваш ID: ${data.student_id}\n` +
        `Дата регистрации: ${new Date(data.registration_date).toLocaleDateString("ru-RU")}\n` +
        `Группа: ${data.group_name}\n\n` +

        `Количество топкоинов: ${coins}\n` +
        `Количество топгемов:  ${gems}\n` +
        `Количество топмани:  ${coins + gems}\n\n` +

        `Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.`

    if (ctx.callbackQuery) {
        await ctx.editMessageText(text, { reply_markup: kb })
    } else {
        await ctx.reply(text, { reply_markup: kb })
    }
};

composer.command("start", start_command_handler)
composer.callbackQuery("mm", start_command_handler)