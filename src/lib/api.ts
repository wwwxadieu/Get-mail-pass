import { invoke } from "@tauri-apps/api/core";
import type {
  GeneratedPassword,
  MailAccount,
  MailDetail,
  MailSummary,
  PassphraseOptions,
  PasswordOptions,
} from "./types";

export const api = {
  generatePassword: (options: PasswordOptions) =>
    invoke<GeneratedPassword>("generate_password", { options }),

  generatePassphrase: (options: PassphraseOptions) =>
    invoke<GeneratedPassword>("generate_passphrase", { options }),

  generateBatch: (options: PasswordOptions, count: number) =>
    invoke<string[]>("generate_batch", { options, count }),

  mailDomains: () => invoke<string[]>("mail_domains"),

  mailCheckConnection: () => invoke<string>("mail_check_connection"),

  mailCreate: (localPart?: string, domain?: string) =>
    invoke<MailAccount>("mail_create", {
      localPart: localPart || null,
      domain: domain || null,
    }),

  mailRestore: (id: string, address: string, password: string) =>
    invoke<MailAccount>("mail_restore", { id, address, password }),

  mailInbox: () => invoke<MailSummary[]>("mail_inbox"),

  mailRead: (id: string) => invoke<MailDetail>("mail_read", { id }),

  mailDelete: (id: string) => invoke<void>("mail_delete", { id }),

  mailDestroy: () => invoke<void>("mail_destroy"),
};

export function errText(e: unknown): string {
  if (typeof e === "string") return e;
  if (e instanceof Error) return e.message;
  return "Đã xảy ra lỗi không xác định";
}
