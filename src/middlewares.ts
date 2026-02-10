import type { NextFunction } from "grammy";
import type { MyContext } from "./types.js";
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

const EVALUATE_COOLDOWN_MS = 300_000

export async function userLoader(ctx: MyContext, next: NextFunction) {
    const userId = ctx.from!.id;
    let client = clientsCache.get(userId);

    if (!client) {
        const data = await getUser(userId);

        if (!data) {
            await ctx.conversation.enter("auth");
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

    if (!ctx.callbackQuery?.data?.startsWith("evaluates/")) {
        const now = Date.now();
        const lastCheck = ctx.session.evaluateCooldown;
        if (now - lastCheck < EVALUATE_COOLDOWN_MS) {
            await next();
            return;
        }
        ctx.session.evaluateCooldown = now;
        await checkEvaluates(ctx)
    }
    
    await next();
}