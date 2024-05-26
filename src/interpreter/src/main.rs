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
        .text("Based on the image, can you tell me the times for Fajr adhan and iqamah, Zuhr adhan and iqamah, Asr adhan and iqamah, Maghrib adhan and iqamah, and Isha adhan and iqamah? Please output each on a separate line with just the label and the time and nothing else. Do not say anything else. Stop and think it through")
        .build()?;

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .max_tokens(1000_u16)
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
