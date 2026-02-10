import { Composer } from "grammy";
import type { MyContext } from "../types.js";

import { composer as startHandlers } from "./start.js";
import { composer as evaluatesHandlers } from "./evaluates.js";
import { composer as leaderboardsHandlers } from "./leaderboards.js";
import { composer as rewardsHandlers } from "./rewards.js";

export const handlers = new Composer<MyContext>();

handlers.use(startHandlers);
handlers.use(evaluatesHandlers);
handlers.use(leaderboardsHandlers);
handlers.use(rewardsHandlers);