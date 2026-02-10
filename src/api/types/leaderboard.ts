export interface StudentRating {
  id: number | null;
  full_name: string | null;
  photo_path: string | null;
  position: number | null;
  amount: number | null;
}

export type StudentRatingList = StudentRating[];