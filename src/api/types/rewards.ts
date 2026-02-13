export interface RewardRecord {
  date: string;
  action: number;
  current_point: number;
  point_types_id: number;
  point_types_name: string;
  achievements_id: number;
  achievements_name: string;
  achievements_type: number;
  badge: number;
  old_competition: boolean;
}

export type RewardsHistory = RewardRecord[];

export const POINT_TYPES_TRANSLATIONS: Record<string, string> = {
  "DIAMOND": "Топкоины",
  "COIN": "Топгемы"
};

export const ACHIEVEMENTS_TRANSLATIONS: Record<string, string> = {
  "EVALUATION_LESSON_MARK": "Оценка занятия",
  "PAIR_VISIT": "Посещение пары",
  "WORK_IN_CLASS": "Поощрение от преподавателя за работу на уроке",
  "ASSESMENT": "Оценка",
  "HOMETASK_INTIME": "Своевременная сдача ДЗ",
  "20_VISITS_WITHOUT_DELAY": "20 посещений без опозданий",
  "20_VISITS_WITHOUT_GAP": "20 посещений без пропусков",
  "10_VISITS_WITHOUT_DELAY": "10 посещений без опозданий",
  "10_VISITS_WITHOUT_GAP": "10 посещений без пропусков",
  "5_VISITS_WITHOUT_DELAY": "5 посещений без опозданий",
  "5_VISITS_WITHOUT_GAP": "5 посещений без пропусков",
  "FILL_IN_PROFILE": "Заполнение профиля",
  "EMAIL_CONFIRMATION": "Подтверждение почты",
  "AUTO_MARK_EXPIRED_HOMEWORK": "Не выполнение сроков сдачи домашнего задания"
};