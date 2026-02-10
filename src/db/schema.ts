import { mysqlTable, bigint, varchar } from 'drizzle-orm/mysql-core';

export const users = mysqlTable('users', {
  id: bigint('id', { mode: 'bigint' }).primaryKey(), 
  username: varchar('username', { length: 255 }),
  password: varchar('password', { length: 255 }),
  accessToken: varchar('access_token', { length: 512 }),
});

export type NewUser = typeof users.$inferInsert;