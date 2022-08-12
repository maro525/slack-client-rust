export interface Response {
  messages: Message[];
}

export interface Message {
  id: string;
  user_id: string;
  user_name: string;
  channel_name: string;
  text: string;
  parmalink: string;
  created_at: string;
}
