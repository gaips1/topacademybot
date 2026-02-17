import { Composer, InlineKeyboard } from "grammy";
import type { MyConversation, MyConversationContext, MyContext } from "../types.js";
import { File } from "node:buffer"

export const composer = new Composer<MyContext>();
const cancelKb = new InlineKeyboard().text("Отменить", "cancel")

composer.callbackQuery(/^upload_homework\//, async (ctx) => {
    const [_, type, page, homework_id = null, homework_evaluate = null] = ctx.callbackQuery.data.split("/")
    
    if (homework_id !== null && homework_evaluate === null) {
        const kb = new InlineKeyboard()
            .text("⭐", `upload_homework/${type}/${page}/${homework_id}/1`)
            .text("⭐⭐⭐", `upload_homework/${type}/${page}/${homework_id}/3`)
            .row()
            .text("⭐⭐", `upload_homework/${type}/${page}/${homework_id}/2`)
            .text("⭐⭐⭐⭐", `upload_homework/${type}/${page}/${homework_id}/4`)
            .row()
            .text("⭐⭐⭐⭐⭐", `upload_homework/${type}/${page}/${homework_id}/5`)
            .row()
            .text("Отменить", "cancel")

        await ctx.answerCallbackQuery()
        return await ctx.editMessageText("Оцените, насколько знания, полученные на уроке, были достаточными для выполнения ДЗ:", { reply_markup: kb });
    
    } else if (homework_id !== null && homework_evaluate !== null) {
        await ctx.answerCallbackQuery()
        await ctx.editMessageText("Потрачено времени на выполнение ДЗ (чч:мм):", { reply_markup: cancelKb })
        return await ctx.conversation.enter("upload_homework", homework_id, homework_evaluate)
    }

    const profile = await ctx.ApiClient.getUserData()
    const homeworks = await ctx.ApiClient.getHomeworks(parseInt(type!), parseInt(page!), profile?.current_group_id)

    if (!profile || !homeworks) {
        return await ctx.answerCallbackQuery({ text: "Произошла ошибка, попробуйте позже.", show_alert: true })
    }

    const kb = new InlineKeyboard()
    homeworks.map((hw) => {
        kb.text(`${hw.name_spec}: ${hw.theme.slice(0, 6)}...`, `upload_homework/${type}/${page}/${hw.id}`).row()
    })
    kb.text("Назад к списку дз", `homework/${type}/1`)

    await ctx.answerCallbackQuery();
    await ctx.editMessageReplyMarkup({ reply_markup: kb })
})

export async function upload_homework(
    conversation: MyConversation,
    ctx: MyConversationContext,
    homework_id: number,
    homework_evaluate: number
) {
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
                await ctx.reply("Часы 0–23, минуты 0–59:", { reply_parameters: { message_id: msg.message_id } })
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
        return await ctx.reply("Превышено время ожидания.")
    }

    let comment: string | null
    try {
        const { msg } = await conversation.waitFor("message:text", { 
            maxMilliseconds: 300_000 
        })
        comment = msg.text.toLowerCase() === "пропустить" ? null : msg.text.trim()
        await ctx.reply(
            `Отправьте файл с выполненной домашней работой.\nРекомендуется загружать файлы в архиве. Файлы в формате .тхт и .csv не допускаются. ${comment === null ? "" : "\n\n(или <code>пропустить</code>, чтобы пропустить этот этап)"}`,
            { reply_markup: cancelKb, parse_mode: "HTML", reply_parameters: { message_id: msg.message_id }}
        )
    } catch {
        return await ctx.reply("Превышено время ожидания.")
    }

    let filename
    let fileBuffer

    try {
        while (true) {
            const { msg } = await conversation.waitFor([":document", ":photo", ":video", ":text"], {
                maxMilliseconds: 300_000
            })

            if (msg?.text?.toLowerCase() === "пропустить" && comment !== null) {
                await ctx.ApiClient.createHomework(homework_id, null, null, hours, minutes, comment)
                await ctx.reply("Домашнее задание успешно отправлено!")
                return

            } else if (msg?.text?.toLowerCase() === "пропустить" && comment === null) {
                await ctx.reply("Отправьте файл с выполненной домашней работой.", { reply_parameters: { message_id: msg.message_id } })
                continue
            }

            if (!msg?.document && !msg?.photo && !msg?.video) {
                await ctx.reply("Пожалуйста, отправьте файл, фото или видео.", { reply_parameters: { message_id: msg.message_id } })
                continue
            }

            let fileId: string;
            let mimeType: string;

            if (msg.document) {
                fileId = msg.document.file_id;
                filename = msg.document.file_name || `document_${Date.now()}`;
                mimeType = msg.document.mime_type || "application/octet-stream";
            } else if (msg.photo?.length) {
                fileId = msg.photo[msg.photo.length - 1]!.file_id;
                filename = `photo_${Date.now()}.jpg`;
                mimeType = "image/jpeg";
            } else if (msg.video) {
                fileId = msg.video.file_id;
                filename = msg.video.file_name || `video_${Date.now()}.mp4`;
                mimeType = msg.video.mime_type || "video/mp4";
            } else {
                await ctx.reply("Пожалуйста, отправьте файл, фото или видео.");
                continue;
            }

            await ctx.reply("Скачиваю и отправляю файл...", { reply_parameters: { message_id: msg.message_id } });
  
            const fileInfo = await ctx.api.getFile(fileId);
            const downloadUrl = `https://api.telegram.org/file/bot${process.env.TOKEN!}/${fileInfo.file_path}`;
            const res = await fetch(downloadUrl);

            if (!res.ok) {
                await ctx.reply("Ошибка при скачивании файла. Попробуйте снова.");
                continue;
            }

            const arrayBuffer = await res.arrayBuffer();
            fileBuffer = Buffer.from(arrayBuffer);
            break
        }
    } catch {
        return await ctx.reply("Превышено время ожидания.")
    }

    const data = await ctx.ApiClient.createHomework(homework_id, fileBuffer, filename, hours, minutes, comment)
    if (data) {
        await ctx.reply("Домашнее задание успешно отправлено!")
    } else {
        await ctx.reply("Произошла ошибка при отправке домашнего задания. Попробуйте позже.")
    }

    // TODO запихни все вызовы апишки в external + save, айдишник туда берется из какого то гета.
}