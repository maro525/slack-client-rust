import { invoke } from "@tauri-apps/api/tauri";
import type { Message } from "~/model/messages";

export const search = async (query: string): Promise<Message[]> => {
  const res = await invoke<{ messages: Message[] }>("search_messages", {
    query,
  }).catch((err) => {
    throw new Error(err);
  });

  return res.messages;
};

export const hello = () => {
  return invoke("hello");
};
