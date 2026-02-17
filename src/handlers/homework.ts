import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";
import { splitText } from "../utils.js";

export const composer = new Composer<MyContext>();

composer.callbackQuery(/^homework\//, async (ctx) => {
    const type = parseInt(ctx.callbackQuery.data.split("/")[1]!)
    const page = parseInt(ctx.callbackQuery.data.split("/")[2]!)

    const profile = await ctx.ApiClient.getUserData()
    const homeworks = await ctx.ApiClient.getHomeworks(type, page, profile?.current_group_id)
    const counters = await ctx.ApiClient.getHomeworkCount(profile?.current_group_id)

    if (!profile || !homeworks || !counters) {
        return await ctx.answerCallbackQuery({ text: "Произошла ошибка, попробуйте позже.", show_alert: true })
    }

    const counter = counters.find(item => item.counter_type === type)!.counter;
    const kb = new InlineKeyboard()
    if (page > 1) kb.text("Назад", `homework/${type}/${page-1}`).style("danger")
    if ((counter - (page * 6)) > 0) kb.text("Вперёд", `homework/${type}/${page+1}`).style("success")
    if (type === 3 || type === 0) kb.row().text("Загрузить выполненное ДЗ", `upload_homework/${type}/${page}`)

    kb.row().text(`Текущие дз ${type === 3 ? "(Вы здесь)" : ""}`, "homework/3/1").style(type === 3 ? "success" : "primary")
    kb.row().text(`На проверке ${type === 2 ? "(Вы здесь)" : ""}`, "homework/2/1").style(type === 2 ? "success" : "primary")
    kb.row().text(`Проверено ${type === 1 ? "(Вы здесь)" : ""}`, "homework/1/1").style(type === 1 ? "success" : "primary")
    kb.row().text(`Просрочено ${type === 0 ? "(Вы здесь)" : ""}`, "homework/0/1").style(type === 0 ? "success" : "primary")

    kb.row().text("Главное меню", "mm").style("danger")

    if (homeworks.length === 0) {
        await ctx.answerCallbackQuery()
        return await ctx.editMessageText("В данный момент выбранных домашних заданий нет.", { reply_markup: kb });
    }

    const text = homeworks
        .map((hw) => {
            const parts = [];
            parts.push(`📚 <b>${hw.name_spec}</b>`);

            parts.push(`  📌 <b>Тема:</b> ${hw.theme}`);
            parts.push(`  👨‍🏫 <b>Преподаватель:</b> ${hw.fio_teach}`);
            parts.push(`  📅 <b>Выдано:</b> ${new Date(hw.creation_time).toLocaleDateString("ru-RU")}`);
            parts.push(`  📅 <b>Срок:</b> ${new Date(hw.completion_time).toLocaleDateString("ru-RU")}`);
            parts.push(`  ℹ️ <b>Комментарий:</b> ${hw.comment || "нет"}`);
            parts.push(`  📩 <a href="${hw.file_path}">Скачать назначенное дз</a>`);

            if (hw.homework_stud) {
                parts.push(`\n  📅 <b>Сдано:</b> ${new Date(hw.homework_stud.creation_time).toLocaleDateString("ru-RU")}`);

                if (hw.homework_stud.stud_answer != null) {
                    parts.push(`  ✅ <b>Твой ответ:</b> ${hw.homework_stud.stud_answer}`);
                }

                if (hw.homework_stud.mark != null) {
                    parts.push(`  ⭐ <b>Оценка:</b> ${hw.homework_stud.mark}`);
                }

                if (hw.homework_stud.file_path != null) {
                    parts.push(`  📩 <a href="${hw.homework_stud.file_path}">Скачать выполненное дз</a>`);
                }
            }

            if (hw.homework_comment?.text_comment) {
                parts.push(`  💬 <b>Комментарий преподавателя:</b> ${hw.homework_comment.text_comment}`);
            }

            return parts.join("\n");
        })
        .join("\n\n")

    const textParts = splitText(text, 4096);
    await ctx.answerCallbackQuery();

    if (textParts.length === 1) {
        await ctx.editMessageText(textParts[0]!, { 
            parse_mode: "HTML", 
            reply_markup: kb 
        });
    } 
    else {
        await ctx.editMessageText(textParts[0]!, { parse_mode: "HTML" });
        for (let i = 1; i < textParts.length; i++) {
            const isLastPart = i === textParts.length - 1;
            await ctx.reply(textParts[i]!, { 
                parse_mode: "HTML",
                reply_markup: isLastPart ? kb : undefined 
            });
        }
    }
})