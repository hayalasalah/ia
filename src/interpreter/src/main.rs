use std::{error::Error, io};

use async_openai::{
    types::{
        ChatCompletionRequestMessageContentPartImageArgs,
        ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ImageUrlArgs, ImageUrlDetail,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let mut b64_img = String::new();
    io::stdin().read_line(&mut b64_img)?;

    let img_part = ChatCompletionRequestMessageContentPartImageArgs::default()
        .image_url(
            ImageUrlArgs::default()
                .url(format!("data:image/png;base64,{}", b64_img))
                .detail(ImageUrlDetail::High)
                .build()?,
        )
        .build()?;

    let text_part = ChatCompletionRequestMessageContentPartTextArgs::default()
        .text(r#"
        Based on the image, can you tell me today's times for Fajr adhan and iqamah, Zuhr adhan and iqamah, Asr adhan and iqamah, Maghrib adhan and iqamah, and Isha adhan and iqamah?
        Please format the output as a JSON object, where the top level keys are prayer names mapped to objects with type and time key-value pairs. Do not say anything else. Do not include any formatting. All the output should be lowercase. Include am and pm on all the times.  
        Stop and think it through and ONLY BASE IT ON THE IMAGE. If there's no times in the image then say so"#)
        .build()?;

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .max_tokens(2000_u16)
        .temperature(0.2)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![img_part.into(), text_part.into()])
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await?;
    let result = response
        .choices
        .first()
        .as_ref()
        .expect("No response from OpenAI")
        .message
        .content
        .as_ref()
        .expect("OpenAI returned None message");

    println!("{}", result);

    Ok(())
}
