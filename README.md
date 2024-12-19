# Shared lib for the Rustafarian Group

Shared code for the Rustafarian group:

- Messages: messages to send between clients, servers, and simulation controller;
- Assembler/Disassembler: assemble/disassemble text into fragments

## Getting started

Import the library:

```rust
rustafarian-shared = { git = "https://github.com/Rustafarian-Unitn/rustafarian-shared", branch = "main" }
```

### Implementation

How to implement new messages?

For the messages, you need to implement a Wrapper for the messages you need. For example, the chat clients expects ChatResponses (ClientList, Message), and the ServerType (Chat, Browser).

Create a Wrapper like this:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatResponseWrapper {
    Chat(ChatResponse),
    ServerType(ServerTypeResponse),
}
```

Then, you can handle the packet received, reassembling the message, and then parse it like this:

```rust
fn handle_reassembled_string(reassembled_string: String) {
    // Attempt to deserialize the reassembled string into a ResponseWrapper
    match serde_json::from_str::<ResponseWrapper>(&reassembled_string) {
        Ok(response) => {
            // Successfully deserialized
            match response {
                ResponseWrapper::Chat(chat_response) => {
                    println!("Received a ChatResponse: {:?}", chat_response);
                    // Process the chat response
                }
                ResponseWrapper::ServerType(server_response) => {
                    println!("Received a ServerTypeResponse: {:?}", server_response);
                    // Process the server type response
                }
                // Add handling for additional response types as needed
            }
        }
        Err(e) => {
            // Handle deserialization error
            eprintln!("Failed to deserialize response: {}", e);
        }
    }
}

```

To send a message, you need to stringify it, like this:

```rust
let serialized = serde_json::to_string(&message).unwrap();
let data = serialized.as_bytes();
```