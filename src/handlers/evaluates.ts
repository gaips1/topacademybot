import { Composer, InlineKeyboard } from "grammy";
import type { MyContext, MyConversationContext } from "../types.js";

export const composer = new Composer<MyContext>();

export async function checkEvaluates(ctx: MyConversationContext) {
    let evaluations = ctx.session.evaluations
    if (evaluations.length === 0) {
        const data = await ctx.ApiClient.getLessonEvaluations()
        if (!data || data.length === 0) return false
        evaluations = data
    }
    ctx.session.evaluations = evaluations

    const evaluation = evaluations[0]!
    const kb = new InlineKeyboard()
        .text("⭐", `evaluates/${evaluation.key}/1`)
        .text("⭐⭐⭐", `evaluates/${evaluation.key}/3`)
        .row()
        .text("⭐⭐", `evaluates/${evaluation.key}/2`)
        .text("⭐⭐⭐⭐", `evaluates/${evaluation.key}/4`)
        .row()
        .text("⭐⭐⭐⭐⭐", `evaluates/${evaluation.key}/5`)

    await ctx.reply(
        "Оцените занятие: " +
        `${evaluation.spec_name.toUpperCase()} ` +
        `${new Date(evaluation.date_visit).toLocaleDateString('ru-RU')}\n` +
        `Оцените работу преподавателя ${evaluation.fio_teach}:`,
        { reply_markup: kb }
    )
}

composer.callbackQuery(/^evaluates\//, async (ctx) => {
    const [_, key, evl1, evl2 = null] = ctx.callbackQuery.data.split("/")
    await ctx.answerCallbackQuery();

    if (evl2 === null) {
        const kb = new InlineKeyboard()
            .text("⭐", `evaluates/${key}/${evl1}/1`)
            .text("⭐⭐⭐", `evaluates/${key}/${evl1}/3`)
            .row()
            .text("⭐⭐", `evaluates/${key}/${evl1}/2`)
            .text("⭐⭐⭐⭐", `evaluates/${key}/${evl1}/4`)
            .row()
            .text("⭐⭐⭐⭐⭐", `evaluates/${key}/${evl1}/5`)

        const currentText = ctx.callbackQuery.message?.text || "";
        const header = currentText.split('\n')[0];

        await ctx.editMessageText(`${header}\n\nТеперь оцените качество урока:`, { reply_markup: kb })

    } else {
        const ok = await ctx.ApiClient.evaluateLesson(key!, Number(evl1), Number(evl2))
        if (!ok) {
            return await ctx.editMessageText("Произошла серверная ошибка при отправке оценки! Попробуйте позже.")
        }

        ctx.session.evaluations.shift();

        await ctx.editMessageText("Спасибо за ваши оценки!")
        return await checkEvaluates(ctx)
    }
})