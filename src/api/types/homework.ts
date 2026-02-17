export interface HomeworkStud {
  id: number;
  filename: string | null;
  file_path: string | null;
  tmp_file: string | null;
  mark: number | null;
  creation_time: string;
  stud_answer: string | null;
  auto_mark: boolean;
}

export interface HomeworkComment {
  text_comment: string | null;
  attachment: string | null;
  attachment_path: string | null;
  date_updated: string;
}

export interface Homework {
  id: number;
  id_spec: number;
  id_teach: number;
  id_group: number;
  fio_teach: string;
  theme: string;
  completion_time: string;
  creation_time: string;
  overdue_time: string;
  filename: string | null;
  file_path: string;
  comment: string;
  name_spec: string;
  status: number;
  common_status: number | null;
  homework_stud: HomeworkStud | null;
  homework_comment: HomeworkComment | null;
  cover_image: string | null;
}

export interface HomeworkCounter {
  counter_type: number;
  counter: number;
}

export interface CreatedHomework {
  id: number,
  filename: string | null,
  file_path: string | null,
  tmp_file: string | null,
  mark: number | null,
  creation_time: string,
  stud_answer: string | null,
  auto_mark: boolean
}

export type HomeworkCounterList = HomeworkCounter[];
export type HomeworkList = Homework[];