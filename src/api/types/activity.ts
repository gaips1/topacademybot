export interface Activity {
  date_visit: string;
  lesson_number: number;
  status_was: number;
  spec_id: number;
  teacher_name: string;
  spec_name: string;
  lesson_theme: string;
  control_work_mark: number | null;
  home_work_mark: number | null;
  lab_work_mark: number | null;
  class_work_mark: number | null;
  practical_work_mark: number | null;
}

export type ActivityList = Activity[];