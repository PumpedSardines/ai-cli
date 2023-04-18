use dotenv_codegen::dotenv;
use serde_json::json;

fn main() {
    const PROMPT: &str = "You're a cli tool that takes a description of a cli command and returns said cli commend. If you dont understand the description, respond with \"isgfoivbuwivodsubfahioabrfk\"";

    let arguments = std::env::args().collect::<Vec<String>>();

    let description = if arguments.len() > 1 {
        arguments[1..].join(" ")
    } else {
        println!("You need to provide a description");
        std::process::exit(0)
    };

    let prompt = format!("{}\n\nDescription:\n{}\n\nCommand:\n", PROMPT, description);

    let client = reqwest::blocking::Client::new();
    const URL: &str = "https://api.openai.com/v1/chat/completions";
    let res = client
        .post(URL)
        .header(
            "Authorization",
            format!("Bearer {}", dotenv!("OPENAI_API_KEY")),
        )
        .header("Content-Type", "application/json; charset=utf-8")
        .body(
            json!({
                "model": "gpt-3.5-turbo",
                "messages": [{
                    "role": "user",
                    "content": prompt,
                }],
                "temperature": 0.7,
            })
            .to_string(),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();

    let res: Response = serde_json::from_str(&res).unwrap();
    let text = res.choices[0].message.content.clone();

    if text.contains("isgfoivbuwivodsubfahioabrfk") {
        println!("The description is too vague");
    } else {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        clipboard.set_text(text.clone()).unwrap();
        println!("\n> {}\n", text);
    }
}

#[derive(serde::Deserialize, Debug)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(serde::Deserialize, Debug)]
struct Message {
    content: String,
}
