use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleDarkMode,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    dark_mode: bool,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            dark_mode: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://robohash.org/{}?set=set4&size=200x200",
                                    u
                                ),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message_text = input.value();
                    if !message_text.trim().is_empty() {
                        let message = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(message_text),
                            data_array: None,
                        };
                        if let Err(e) = self
                            .wss
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&message).unwrap())
                        {
                            log::debug!("error sending to channel: {:?}", e);
                        }
                        input.set_value("");
                    }
                };
                false
            }
            Msg::ToggleDarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let current_username = user.username.borrow().clone();
        
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_dark_mode = ctx.link().callback(|_| Msg::ToggleDarkMode);
        
        let bg_class = if self.dark_mode {
            "bg-gray-900 text-white"
        } else {
            "bg-gradient-to-br from-purple-50 via-blue-50 to-indigo-50"
        };
        
        let sidebar_class = if self.dark_mode {
            "bg-gray-800 border-gray-700"
        } else {
            "bg-white border-gray-200 shadow-lg"
        };
        
        let header_class = if self.dark_mode {
            "bg-gray-800 border-gray-700"
        } else {
            "bg-white border-gray-200 shadow-sm"
        };

        html! {
            <div class={format!("flex w-screen h-screen {}", bg_class)}>
                <div class={format!("flex-none w-72 h-screen border-r-2 {}", sidebar_class)}>
                    <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
                        <div class="flex items-center gap-2">
                            <span class="text-2xl">{"👥"}</span>
                            <span class="text-xl font-bold">{"Online Users"}</span>
                        </div>
                        <button 
                            onclick={toggle_dark_mode}
                            class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                            title={if self.dark_mode { "Switch to Light Mode" } else { "Switch to Dark Mode" }}
                        >
                            <span class="text-xl">{if self.dark_mode { "🌞" } else { "🌙" }}</span>
                        </button>
                    </div>
                    
                    <div class="p-2 space-y-2">
                        {
                            self.users.iter().map(|u| {
                                let is_current_user = u.name == current_username;
                                let user_bg = if is_current_user {
                                    if self.dark_mode { 
                                        "bg-gradient-to-r from-blue-600 to-purple-600 text-white" 
                                    } else { 
                                        "bg-gradient-to-r from-blue-500 to-purple-500 text-white" 
                                    }
                                } else {
                                    if self.dark_mode { 
                                        "bg-gray-700 hover:bg-gray-600" 
                                    } else { 
                                        "bg-gray-50 hover:bg-gray-100" 
                                    }
                                };
                                
                                html!{
                                    <div class={format!("flex items-center p-3 rounded-xl transition-all duration-200 {}", user_bg)}>
                                        <div class="relative">
                                            <img class="w-12 h-12 rounded-full ring-2 ring-white/30" src={u.avatar.clone()} alt="avatar"/>
                                            <div class="absolute -bottom-1 -right-1 w-4 h-4 bg-green-500 border-2 border-white rounded-full"></div>
                                        </div>
                                        <div class="ml-3 flex-grow">
                                            <div class="flex items-center gap-2">
                                                <span class="font-semibold">{u.name.clone()}</span>
                                                {if is_current_user {
                                                    html!{<span class="text-xs bg-white/20 px-2 py-1 rounded-full">{"You"}</span>}
                                                } else {
                                                    html!{}
                                                }}
                                            </div>
                                            <div class={format!("text-xs {}", if is_current_user { "text-white/80" } else { "text-gray-500" })}>
                                                {if is_current_user { "That's you! 🎉" } else { "Active now" }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
                
                <div class="grow h-screen flex flex-col">
                    <div class={format!("w-full h-16 border-b-2 flex items-center justify-between px-6 {}", header_class)}>
                        <div class="flex items-center gap-3">
                            <span class="text-2xl">{"💬"}</span>
                            <div>
                                <h1 class="text-xl font-bold">{"WebChat Room"}</h1>
                                <p class="text-sm text-gray-500">{format!("{} people online", self.users.len())}</p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2 text-sm text-gray-500">
                            <span>{"🟢"}</span>
                            <span>{"Connected"}</span>
                        </div>
                    </div>
                    
                    <div class="w-full flex-grow overflow-y-auto p-4 space-y-4">
                        {
                            self.messages.iter().map(|m| {
                                let is_own_message = m.from == current_username;
                                let user = self.users.iter().find(|u| u.name == m.from);
                                
                                let container_class = if is_own_message {
                                    "flex justify-end"
                                } else {
                                    "flex justify-start"
                                };
                                
                                let message_class = if is_own_message {
                                    if self.dark_mode {
                                        "bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-2xl rounded-br-md p-4 max-w-md shadow-lg"
                                    } else {
                                        "bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-2xl rounded-br-md p-4 max-w-md shadow-lg"
                                    }
                                } else {
                                    if self.dark_mode {
                                        "bg-gray-700 text-white rounded-2xl rounded-bl-md p-4 max-w-md shadow-lg"
                                    } else {
                                        "bg-white border border-gray-200 text-gray-800 rounded-2xl rounded-bl-md p-4 max-w-md shadow-lg"
                                    }
                                };
                                
                                html!{
                                    <div class={container_class}>
                                        <div class="flex items-end gap-3 max-w-2xl">
                                            {if !is_own_message {
                                                if let Some(user) = user {
                                                    html!{
                                                        <img class="w-10 h-10 rounded-full ring-2 ring-white/20 flex-shrink-0" src={user.avatar.clone()} alt="avatar"/>
                                                    }
                                                } else {
                                                    html!{<div class="w-10 h-10 bg-gray-400 rounded-full flex-shrink-0"></div>}
                                                }
                                            } else {
                                                html!{}
                                            }}
                                            
                                            <div class={message_class}>
                                                {if !is_own_message {
                                                    html!{
                                                        <div class="text-sm font-semibold mb-2 opacity-80">
                                                            {m.from.clone()}
                                                        </div>
                                                    }
                                                } else {
                                                    html!{}
                                                }}
                                                
                                                <div class="text-sm leading-relaxed">
                                                    {if m.message.ends_with(".gif") || (m.message.starts_with("http") && (m.message.contains(".jpg") || m.message.contains(".png"))) {
                                                        html!{<img class="mt-2 rounded-lg max-w-xs" src={m.message.clone()} alt="shared image"/>}
                                                    } else {
                                                        html!{m.message.clone()}
                                                    }}
                                                </div>
                                            </div>
                                            
                                            {if is_own_message {
                                                html!{
                                                    <img class="w-10 h-10 rounded-full ring-2 ring-white/20 flex-shrink-0" 
                                                         src={format!("https://robohash.org/{}?set=set4&size=200x200", current_username)} 
                                                         alt="your avatar"/>
                                                }
                                            } else {
                                                html!{}
                                            }}
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    
                    <div class={format!("w-full p-4 border-t-2 {}", header_class)}>
                        <div class="flex items-center gap-3">
                            <input 
                                ref={self.chat_input.clone()} 
                                type="text" 
                                placeholder="💭 Type your message here..." 
                                class={format!(
                                    "flex-grow py-3 px-4 rounded-full border-2 outline-none transition-all focus:ring-4 {}",
                                    if self.dark_mode {
                                        "bg-gray-700 border-gray-600 text-white placeholder-gray-400 focus:border-blue-500 focus:ring-blue-500/20"
                                    } else {
                                        "bg-gray-50 border-gray-200 text-gray-800 placeholder-gray-500 focus:border-blue-500 focus:ring-blue-500/20"
                                    }
                                )}
                                name="message" 
                                required=true 
                            />
                            <button 
                                onclick={submit} 
                                class="p-3 bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 rounded-full shadow-lg hover:shadow-xl transition-all duration-200 transform hover:scale-105"
                                title="Send message"
                            >
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white w-6 h-6">
                                    <path d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                        
                        <div class="text-center mt-3">
                            <p class="text-xs text-gray-500">
                                {"Press Enter to send"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}