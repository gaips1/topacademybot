export interface LessonEvaluation {
  key: string;
  date_visit: string;
  fio_teach: string;
  spec_name: string;
  teach_photo: string | null;
}

export type EvaluationList = LessonEvaluation[];