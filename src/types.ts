import { Context, Api, type SessionFlavor } from "grammy";
import type { HydrateFlavor, HydrateApiFlavor } from "@grammyjs/hydrate";
import { ApiClient } from "./api/apiclient.js";
import type { ConversationFlavor } from "@grammyjs/conversations";
import type { EvaluationList } from "./api/types/lesson_evaluation.js";

export interface SessionData {
  evaluations: EvaluationList
  evaluateCooldown: number
}

export type MyContext = HydrateFlavor<Context> & { ApiClient: ApiClient } & ConversationFlavor<Context> & SessionFlavor<SessionData>;
export type MyConversationContext = MyContext;
export type MyApi = HydrateApiFlavor<Api>;