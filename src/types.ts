import { Context, Api, type SessionFlavor } from "grammy";
import type { HydrateFlavor, HydrateApiFlavor } from "@grammyjs/hydrate";
import { ApiClient } from "./api/apiclient.js";
import type { Conversation, ConversationFlavor } from "@grammyjs/conversations";
import type { EvaluationList } from "./api/types/lesson_evaluation.js";

export interface SessionData {
  evaluations: EvaluationList
  evaluateCooldown: number
}

export type MyContext = ConversationFlavor<
  HydrateFlavor<Context> &
  SessionFlavor<SessionData> &
  { ApiClient: ApiClient }
>;

export type MyConversationContext =
  HydrateFlavor<Context> &
  SessionFlavor<SessionData> &
  { ApiClient: ApiClient };

export type MyConversation = Conversation<MyContext, MyConversationContext>;
export type MyApi = HydrateApiFlavor<Api>;