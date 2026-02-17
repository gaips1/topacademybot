import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";
import { ACHIEVEMENTS_TRANSLATIONS, POINT_TYPES_TRANSLATIONS } from "../api/types/rewards.js";

export const composer = new Composer<MyContext>();

composer.callbackQuery(/^rewards\//, async (ctx) => {
    const page = parseInt(ctx.callbackQuery.data.split("/")[1]!)
    const data = await ctx.ApiClient.getRewards()
    if (!data) {
        return await ctx.answerCallbackQuery({ text: "Произошла ошибка, попробуйте позже.", show_alert: true })
    }
    
    const header = "<b>🏆 Ваши награды 🏆</b>\n\n"
    const text =
        header +
        data.slice(page * 10, (page + 1) * 10)
            .map(
                (entry) => 
                `${new Date(entry.date).toLocaleString('ru-RU')}\n` +
                `${ACHIEVEMENTS_TRANSLATIONS[entry.achievements_name] || "Неизвестно"}: +${entry.current_point} ` +
                `${POINT_TYPES_TRANSLATIONS[entry.point_types_name]}\n`
            )
                .join("----------------------------\n")

    const kb = new InlineKeyboard()
        .text("Вперёд", `rewards/${page+1}`)

    if (page > 0) {
        kb.row().text("Назад", `rewards/${page-1}`)
    }

    kb.row().text("Главное меню", "mm")

    await ctx.answerCallbackQuery()
    await ctx.editMessageText(text, { reply_markup: kb, parse_mode: "HTML" })
})