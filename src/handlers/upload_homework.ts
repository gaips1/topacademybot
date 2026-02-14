import { Composer, InlineKeyboard } from "grammy";
import type { MyContext, MyConversationContext } from "../types.js";
import { splitText } from "../utils.js";
import type { Conversation } from "@grammyjs/conversations";

export const composer = new Composer<MyContext>();

composer.callbackQuery(/^upload_homework\//, async (ctx) => {
    const [_, page, homework_id = null, homework_evaluate = null] = ctx.callbackQuery.data.split("/")
    
    if (homework_id !== null && homework_evaluate === null) {
        const kb = new InlineKeyboard()
            .text("⭐", `upload_homework/${page}/${homework_id}/1`)
            .text("⭐⭐⭐", `upload_homework/${page}/${homework_id}/3`)
            .row()
            .text("⭐⭐", `upload_homework/${page}/${homework_id}/2`)
            .text("⭐⭐⭐⭐", `upload_homework/${page}/${homework_id}/4`)
            .row()
            .text("⭐⭐⭐⭐⭐", `upload_homework/${page}/${homework_id}/5`)
            .row()
            .text("Отменить", "cancel")

        await ctx.answerCallbackQuery()
        return await ctx.editMessageText("Оцените, насколько знания, полученные на уроке, были достаточными для выполнения ДЗ:", { reply_markup: kb });
    
    } else if (homework_id !== null && homework_evaluate !== null) {
        await ctx.answerCallbackQuery()
        return await ctx.conversation.enter("upload_homework", homework_id, homework_evaluate)
    }

    const profile = await ctx.ApiClient.getUserData()
    const homeworks = await ctx.ApiClient.getHomeworks(3, parseInt(page!), profile?.current_group_id)

    if (!profile || !homeworks) {
        return await ctx.answerCallbackQuery({ text: "Произошла ошибка, попробуйте позже.", show_alert: true })
    }

    const kb = new InlineKeyboard()
    homeworks.map((hw) => {
        kb.text(`${hw.name_spec}: ${hw.theme.slice(0, 6)}...`, `upload_homework/${page}/${hw.id}`).row()
    })
    kb.text("Назад к списку дз", `homework/3/1`)

    await ctx.answerCallbackQuery();
    await ctx.editMessageReplyMarkup({ reply_markup: kb })
})

const cancelKb = new InlineKeyboard().text("Отменить", "cancel")

export async function upload_homework(
    conversation: Conversation<MyContext, MyConversationContext>,
    ctx: MyContext,
    homework_id: number,
    homework_evaluate: number
) {
    await ctx.editMessageText("Потрачено времени на выполнение ДЗ (чч:мм):", { reply_markup: cancelKb })

    let hours: number
    let minutes: number

    try {
        while (true) {
            const { msg } = await conversation.waitFor("message:text", { 
                maxMilliseconds: 300_000 
            })
            const match = msg.text.trim().match(/^(\d{1,2}):(\d{2})$/)
            
            if (!match) {
                await ctx.reply("Формат: чч:мм (например 1:45, 09:05):", { reply_parameters: { message_id: msg.message_id } })
                continue
            }

            const h = Number(match[1])
            const m = Number(match[2])

            if (h > 23 || m > 59) {
                await ctx.reply("Часы 0–23, минуты 0–59:", { reply_parameters: { message_id: msg.message_id } });
                continue
            }

            hours = h
            minutes = m

            await ctx.reply(
                "Отправьте ваш текстовый ответ (или <code>пропустить</code>, чтобы пропустить этот этап):",
                { reply_markup: cancelKb, parse_mode: "HTML", reply_parameters: { message_id: msg.message_id }}
            )
            break
        }
    } catch {
        return await ctx.editMessageText("Превышено время ожидания.")
    }

    let comment: string | null
    try {
        const { msg } = await conversation.waitFor("message:text", { 
            maxMilliseconds: 300_000 
        })
        comment = msg.text.toLowerCase() === "пропустить" ? null : msg.text.trim()
        await ctx.reply(
            "Отправьте документ (не .txt и не .csv), фото, или видео с выполненной домашней работой:",
            { reply_markup: cancelKb, reply_parameters: { message_id: msg.message_id }}
        )
    } catch {
        return await ctx.editMessageText("Превышено время ожидания.")
    }

    try {
        while (true) {
            const { msg } = await conversation.waitFor([":document" , ":photo", ":video"], { 
                maxMilliseconds: 300_000, otherwise: ctx => ctx.reply("Отправьте документ (не .txt и не .csv), фото, или видео:")
            })
        }
    } catch {
        return await ctx.editMessageText("Превышено время ожидания.")
    }
}