use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;

// Structs for building the JSON request body, matching the OpenAI multimodal format.
#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: Vec<ContentPart>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

// Structs for deserializing the JSON response.
#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Configuration ---
    let api_key = "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";
    let image_path = "Screenshot from 2025-06-23 19-08-32.png";
    // Using a Vision-Language (VL) model capable of processing images.
    let model_name = "qwen/qwen-vl-plus"; 
    let prompt_text = "Transcribe the text from this screenshot exactly as it appears. Pay close attention to formatting, including line breaks, symbols, and filenames.";

    println!("Reading and encoding image: {}...", image_path);

    // 1. Read the image file into bytes.
    let image_bytes = fs::read(image_path)?;

    // 2. Encode the bytes into a Base64 string.
    let base64_image = base64::encode(&image_bytes);

    // 3. Format as a data URI.
    let image_uri = format!("data:image/png;base64,{}", base64_image);

    println!("Image encoded successfully. Preparing API request...");

    // 4. Construct the request body.
    let request_body = RequestBody {
        model: model_name.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![
                ContentPart::Text {
                    text: prompt_text.to_string(),
                },
                ContentPart::ImageUrl {
                    image_url: ImageUrl { url: image_uri },
                },
            ],
        }],
        max_tokens: 2048,
    };

    // 5. Make the API call using the reqwest client.
    let client = reqwest::Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;
    
    println!("Request sent. Waiting for response...");

    if response.status().is_success() {
        let response_body = response.json::<ChatCompletionResponse>().await?;
        if let Some(choice) = response_body.choices.get(0) {
            println!("\n--- Qwen Vision Model Response ---");
            println!("{}", choice.message.content);
            println!("------------------------------------");
        } else {
            println!("API call successful, but no choices were returned.");
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("Error: API call failed with status {}", status);
        eprintln!("Response: {}", error_text);
    }

    Ok(())
} 