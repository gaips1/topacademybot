import { Composer, InlineKeyboard } from "grammy";
import type { MyContext } from "../types.js";
import { STATUS_WAS_TRANSLATED } from "../api/types/activity.js";
import { splitText } from "../utils.js";

export const composer = new Composer<MyContext>();

const fmt = (d: Date | string | number) => new Date(d).toLocaleDateString("ru-RU");
const getMarks = (a: any) => 
    [a.control_work_mark, a.home_work_mark, a.lab_work_mark, a.class_work_mark, a.practical_work_mark]
    .filter(v => v != null)
    .join(", ");


composer.callbackQuery(/^activity\//, async (ctx) => {
    const page = parseInt(ctx.callbackQuery.data.split("/")[1]!)
    const data = await ctx.ApiClient.getActivity();
    
    if (!data?.length) {
        return await ctx.answerCallbackQuery({ 
            text: !data ? "Ошибка получения данных" : "Вы не были на парах", 
            show_alert: true 
        });
    }

    const base = new Date(data[0]!.date_visit);
    const start = new Date(base.setDate(base.getDate() - (base.getDay() || 7) + 1 - (page * 7)));
    start.setHours(0, 0, 0, 0);
    const end = new Date(start); 
    end.setDate(end.getDate() + 5);

    const scheduleByDate = data
        .filter(a => { 
            const d = new Date(a.date_visit); 
            return d >= start && d < end; 
        })
        .sort((a, b) => {
            const dateDiff = new Date(a.date_visit).getTime() - new Date(b.date_visit).getTime();
            if (dateDiff !== 0) return dateDiff;
            return a.lesson_number - b.lesson_number;
        })
        .reduce((acc, ac) => {
            const dateKey = fmt(ac.date_visit);
            const marks = getMarks(ac);
            
            const row = `\n🎓 <b>${ac.spec_name}</b>\n` +
                        `    - <b>Пара:</b> ${ac.lesson_number}\n` +
                        `    - <b>Статус:</b> ${STATUS_WAS_TRANSLATED[ac.status_was] || ac.status_was}\n` +
                        `    - <b>Преподаватель:</b> ${ac.teacher_name}\n` +
                        `    - <b>Тема:</b> ${ac.lesson_theme}\n` +
                        `    - <b>Оценки:</b> ${marks || "нет"}\n`;
            
            (acc[dateKey] ||= []).push(row);
            return acc;
        }, {} as Record<string, string[]>);

    const content = Object.entries(scheduleByDate)
        .map(([date, rows]) => `\n<b><u>${date}</u></b>${rows.join("")}`)
        .join("");

    const fullText = `<b>🗓️ Учебная неделя ${fmt(start)} - ${fmt(end)}</b>\n` + 
                     (content || "\n<i>В эту неделю пар не было</i>");

    const kb = new InlineKeyboard().text("Вперёд", `activity/${page + 1}`);
    if (page > 0) kb.row().text("Назад", `activity/${page - 1}`);
    kb.row().text("Главное меню", "mm");

    const textParts = splitText(fullText, 4096);
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
});