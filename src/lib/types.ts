export type PasswordOptions = {
  length: number;
  lowercase: boolean;
  uppercase: boolean;
  digits: boolean;
  symbols: boolean;
  avoidAmbiguous: boolean;
  noRepeat: boolean;
};

export type PassphraseOptions = {
  words: number;
  separator: string;
  capitalize: boolean;
  addNumber: boolean;
};

export type GeneratedPassword = {
  value: string;
  entropyBits: number;
  poolSize: number;
  score: number;
  label: string;
  crackTimeOnline: string;
  crackTimeOffline: string;
};

export type MailAccount = {
  id: string;
  address: string;
  password: string;
  token: string;
};

export type MailSummary = {
  id: string;
  fromName: string;
  fromAddress: string;
  subject: string;
  intro: string;
  seen: boolean;
  createdAt: string;
  hasAttachments: boolean;
  otp: string | null;
};

export type MailDetail = {
  id: string;
  subject: string;
  fromName: string;
  fromAddress: string;
  createdAt: string;
  text: string;
  otp: string | null;
};

export type InboxEvent = {
  messages: MailSummary[];
  newCount: number;
};

export type HistoryItem = {
  id: string;
  kind: "password" | "passphrase" | "email";
  value: string;
  note: string;
  at: number;
};
