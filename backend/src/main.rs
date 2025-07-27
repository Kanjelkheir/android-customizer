use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use gemini_rust::{Gemini, Message, Role, Content};
use std::env;
use root_server::utils::json_parse;
use serde_json::json;



fn main() {
    trpl::run(async {
        // Create a TCP Listener
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        // Loop over the incoming connections
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream).await;

        }
    });

}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let status_line = "HTTP/1.1 200 OK";
    let gemini_output = call_gemini("J701F").await.unwrap();
    let device_data = json_parse(gemini_output);
    let json_content = match serde_json::to_string_pretty(&device_data) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize JSON: {}", e);
            // Return an error JSON or handle appropriately
            return; // Or send a 500 error response
        }
    };
    let length = json_content.len() as u64;
    let response = format!(
        "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, json_content
    );
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush response: {}", e);
        std::process::exit(1);
    }
}

async fn call_gemini(model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY")?;
    let client = Gemini::new(api_key);
    let message = format!("1.Stock Firmware: J701FXXSACUC1_J701FOJVACUC2_XSG
\n2. Odin Tool: Odin3_v3.14.4\n3. **TWRP .tar image: j7 core-j7 neo-j7 nxt.tar\n
        4. SHRP Recovery: Shrp_v2.1_J7velte.zip\n
        5. OrangeFox Recovery: OrangeFox-J7velte-beta@R11.1.zip**\n
        6. Custom ROM: PixelExperience_Plus-A11-Exynos7870-UNOFFICIAL.zip\n
        7. ADB Tools Folder: platform-tools (or platform-tools-latest-windows)\n
        this is a mockup data demonstration as what you should return for the following device model: {}\n
        fill the mockup data with actual data and links, don't return anything else than the mockup data I sent you
        except with actual links and data. Make sure the links are valid and doesn't return not found.\n
        these are working links for example, they should be something like these:
        http://www.mediafire.com/file/ujk2hkeo1x9bqg1/j7_core-j7_neo-j7_nxt.tar/file : twrp
        //https://samfw.com/firmware/SM-J701F/XSG/J701FXXSACUC1 : stock firmware
        //https://technastic.com/odin-download-samsung-latest-all-versions/ : odin
        //https://orangefox.download/device/j7velte : Orangefox  Recovery
        //https://sourceforge.net/projects/repomagno/files/PixelExperiencePlus-A11/PixelExperience_Plus-A11-Exynos7870-UNOFFICIAL.zip/download : Custom firmware
        //https://sourceforge.net/projects/shrp/files/J7%20Nxt/Shrp_v2.1_J7velte.zip/download : SHRP Recovery

these are example of working links, they should be something like these. AND PLEASE USE PLAIN TEXT, NO BOLD OR CODE BLOCKS
 ", model);
    let content = client.generate_content().with_user_message(message).execute().await?;
    Ok(content.text())
}
