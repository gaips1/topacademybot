import type { NextFunction } from "grammy";
import type { MyConversationContext, MyContext } from "./types.js";
import { getUser, updateUserAccessToken } from "./db/services/users.service.js";
import { ApiClient } from "./api/apiclient.js";
import { LRUCache } from "lru-cache";
import { checkEvaluates } from "./handlers/evaluates.js";

export const clientsCache = new LRUCache<number, ApiClient>({
    max: 1000,
    ttl: 21_600_000,
    ttlResolution: 60_000,
    ttlAutopurge: true,
    updateAgeOnGet: true,
    updateAgeOnHas: false
});

export async function userLoader(ctx: MyConversationContext, next: NextFunction) {
    if (!ctx.from) {
        return await next();
    }
    
    const userId = ctx.from.id;
    let client = clientsCache.get(userId);

    if (!client) {
        const data = await getUser(userId);

        if (!data) {
            if ("conversation" in ctx) {
                const fullCtx = ctx as MyContext;
                await fullCtx.conversation.enter("auth");
            }
            return
        }

        client = new ApiClient(
            data.username!,
            data.password!,
            data.accessToken!,
            async (newToken) => {await updateUserAccessToken(userId, newToken);}
        );
        clientsCache.set(userId, client);
    }
    
    ctx.ApiClient = client;

    if ("conversation" in ctx && !ctx.callbackQuery?.data?.startsWith("evaluates/")) {
        const fullCtx = ctx as MyContext;
        const now = Date.now();
        const lastCheck = fullCtx.session.evaluateCooldown;
        if (now - lastCheck < 300_000) {
            await next();
            return;
        }
        fullCtx.session.evaluateCooldown = now;
        await checkEvaluates(fullCtx)
    }
    
    await next();
}