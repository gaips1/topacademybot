import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";

export const composer = new Composer<MyContext>();

composer.callbackQuery(/^leaderboard\//, async (ctx) => {
    const [_, type] = ctx.callbackQuery.data.split("/")
    const isGroupLeaderboard = (type == "group")
    const data = await ctx.ApiClient.getLeaderboard(isGroupLeaderboard)

    if (!data) {
        return await ctx.answerCallbackQuery({ text: "Произошла ошибка, попробуйте позже.", show_alert: true })
    }

    const header = "<b>🏆 Таблица лидеров 🏆</b>\n\n"
    const text =
        header +
        data.map((entry, i) => `${i + 1}. ${entry.full_name} - ${entry.amount} топмани`)
            .join("\n");
    
    const kb = new InlineKeyboard()
        .text(
            isGroupLeaderboard ? "Таблица лидеров (в потоке)" : "Таблица лидеров (в группе)",
            isGroupLeaderboard ? "leaderboard/stream" : "leaderboard/group"
        )
        .row()
        .text("Главное меню", "mm")

    await ctx.answerCallbackQuery()
    await ctx.editMessageText(text, { reply_markup: kb, parse_mode: "HTML" })
})