import type { 
    AuthResponse, 
    UserData, 
    StudentRatingList, 
    RewardsHistory, 
    ActivityList, 
    HomeworkList, 
    HomeworkCounterList, 
    EvaluationList, 
} from "./types/index.js";

type TokenUpdateCallback = (newToken: string) => Promise<void> | void;

export class ApiClient {
    private token: string | null = null;
    private baseUrl: string = 'https://msapi.top-academy.ru/api/v2/';
    private loginPromise: Promise<string> | null = null;

    public ErrorMessage = "Не удалось получить ваши данные с серверов Академии ТОП. Пожалуйста, проверьте введеные вами данные аккаунта или повторите попытку позже."

    constructor(
        private username: string,
        private password: string,
        existingToken?: string,
        private onTokenUpdate?: TokenUpdateCallback
    ) {
        if (existingToken) this.token = existingToken;
    }
    
    async login(): Promise<string> {
        if (this.loginPromise) return this.loginPromise;

        this.loginPromise = (async () => {
            try {
                const response = await fetch(`${this.baseUrl}auth/login`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json', "Referer": "https://journal.top-academy.ru/"},
                    body: JSON.stringify({
                        application_key: "6a56a5df2667e65aab73ce76d1dd737f7d1faef9c52e8b8c55ac75f565d8e8a6",
                        id_city: null,
                        password: this.password,
                        username: this.username
                    })
                });

                if (!response.ok) {
                    throw new Error(`Login failed with status: ${response.status}`);
                }

                const data: AuthResponse = await response.json();
                if (!data.access_token) {
                    throw new Error("No access token received");
                }

                this.token = data.access_token;

                if (this.onTokenUpdate) {
                    await this.onTokenUpdate(this.token);
                }

                return this.token;
            } finally {
                this.loginPromise = null;
            }
        })();
        
        return this.loginPromise;
    }
    private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
        if (!this.token) {
            await this.login();
        }

        const sendRequest = () => fetch(`${this.baseUrl}${endpoint}`, {
            ...options,
            headers: {
                ...options.headers,
                'Authorization': `Bearer ${this.token}`,
                'Accept': "application/json, text/plain, */*",
                'Content-Type': 'application/json',
                "Referer": "https://journal.top-academy.ru/"
            }
        });

        let response = await sendRequest();
        if (response.status === 401) {
            await this.login();
            response = await sendRequest();
        }

        if (!response.ok) {
            throw new Error(`Request to ${endpoint} failed with status: ${response.status}`);
        }

        const text = await response.text();
        const trimmedText = text.trim();
        
        if (!trimmedText) {
            return null as unknown as T;
        }

        try {
            return JSON.parse(trimmedText) as T;
        } catch (e) {
            console.error(`Failed to parse JSON from ${endpoint}. Response text:`, text);
            throw e;
        }
    }


    async getUserData(): Promise<UserData | null> {
        try {
            return await this.request<UserData>('settings/user-info');
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getLeaderboard(isGroup: boolean): Promise<StudentRatingList | null> {
        try {
            const endpoint = isGroup ? "dashboard/progress/leader-group" : "dashboard/progress/leader-stream";
            return await this.request<StudentRatingList>(endpoint);
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getRewards(): Promise<RewardsHistory | null> {
        try {
            return await this.request<RewardsHistory>('dashboard/progress/activity');
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getActivity(): Promise<ActivityList | null> {
        try {
            return await this.request<ActivityList>('progress/operations/student-visits');
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getHomeworks(type: number, page: number, groupId: number = 8): Promise<HomeworkList | null> {
        try {
            const query = `page=${page}&status=${type}&type=0&group_id=${groupId}`;
            return await this.request<HomeworkList>(`homework/operations/list?${query}`);
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getHomeworkCount(groupId: number = 8): Promise<HomeworkCounterList | null> {
        try {
            return await this.request<HomeworkCounterList>(`count/homework?type=0&group_id=${groupId}`);
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async getLessonEvaluations(): Promise<EvaluationList | null> {
        try {
            return await this.request<EvaluationList>("feedback/students/evaluate-lesson-list");
        } catch (e) {
            console.error(e);
            return null;
        }
    }

    async evaluateLesson(
        key: string,
        mark_lesson: number,
        mark_teach: number,
        tags_lesson: string[] = [],
        tags_teach: string[] = [],
        comment_lesson: string = "",
        comment_teach: string = ""
    ): Promise<boolean> {
        try {
            await this.request<any>("feedback/students/evaluate-lesson", {
                method: "POST",
                body: JSON.stringify({
                    comment_teach,
                    comment_lesson,
                    key,
                    mark_lesson,
                    mark_teach,
                    tags_lesson,
                    tags_teach
                })
            });
            return true;
        } catch (e) {
            console.error(e);
            return false;
        }
    }
}