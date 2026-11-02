use serde::de::IgnoredAny;

use super::{types::*, *};

impl User {
    pub async fn create_homework(
        &self,
        homework_id: u64,
        file_data: Option<Vec<u8>>,
        filename: Option<String>,
        spent_hour: u8,
        spent_min: u8,
        mark: u8,
        answer_text: Option<String>,
    ) -> Result<(), ApiError> {
        let mut form_data = reqwest::multipart::Form::new().text("id", homework_id.to_string());

        if let (Some(data), Some(name)) = (file_data, filename) {
            let mime = mime_guess::from_path(&name).first_or_octet_stream();
            form_data = form_data.part(
                "file",
                reqwest::multipart::Part::bytes(data)
                    .file_name(name)
                    .mime_str(mime.as_ref())?,
            );
        }

        form_data = form_data
            .text("answerText", answer_text.clone().unwrap_or_default())
            .text("spentTimeHour", format!("{:02}", spent_hour))
            .text("spentTimeMin", format!("{:02}", spent_min));

        let request_builder = REQWEST_CLIENT
            .post(format!("{BASE_URL}/homework/operations/create"))
            .multipart(form_data);

        self.request_with_options::<IgnoredAny>(request_builder)
            .await?;

        let form_data = reqwest::multipart::Form::new().text(
            "EvaluationHomeworkForm",
            format!(
                "{{\
                    \"id\": null, \"idDomZad\": {homework_id},\
                    \"mark\": {mark}, \"comment\": \"{}\"\
                }}",
                answer_text.unwrap_or_default()
            ),
        );

        let request_builder = REQWEST_CLIENT
            .post(format!("{BASE_URL}/homework/evaluation/operations/save"))
            .multipart(form_data);

        self.request_with_options::<IgnoredAny>(request_builder)
            .await?;

        Ok(())
    }
}
