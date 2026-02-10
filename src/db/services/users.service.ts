import { eq } from "drizzle-orm";
import { db } from "../index.js";
import { users } from "../schema.js";
import type { NewUser } from "../schema.js";

export const getUser = async (id: number | string) => {
  const user = await db.query.users.findFirst({
    where: eq(users.id, BigInt(id)),
  });
  return user ?? null;
};

export const insertUser = async (id: number | string, username: string, password: string, token: string | null) => {
  const newUser: NewUser = { id: BigInt(id), username: username, password: password, accessToken: token };
  const result = await db.insert(users).values(newUser);
  return result;
};

export const updateUserAccessToken = async (
  id: bigint | number | string,
  token: string
) => {
  console.log("обновил токен!")
  const result = await db.update(users).set({ accessToken: token }).where(
    eq(users.id, BigInt(id))
  );
  return result;
};

