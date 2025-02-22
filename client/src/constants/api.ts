// TODO - control the API domain/port from a .env variable

export const API = {
  CREATE_USER: "http://localhost:3000/create_user",
  CONFIRM_SUBSCRIPTION: "http://localhost:3000/confirm",
  CANCEL_SUBSCRIPTION: "http://localhost:3000/cancel",
  GET_USER: "http://localhost:3000/user",
  GET_USER_PLAN: (planId: string) => `http://localhost:3000/plan/${planId}`,
  GET_USER_LINKS: "http://localhost:3000/user/links",
  CREATE_LINK: "http://localhost:3000/link",
  UPDATE_LINK: "http://localhost:3000/link",
  DELETE_LINK: (linkId: string) => `http://localhost:3000/link/${linkId}`,
  SUGGEST: (query: string) => `http://localhost:3000/suggest/${query}`,
  FEEDBACK: "http://localhost:3000/feedback",
  CREATE_SETTINGS: "http://localhost:3000/settings",
  UPDATE_SETTINGS: "http://localhost:3000/settings",
  GET_SETTINGS: "http://localhost:3000/settings",
  GET_USER_DATA: "http://localhost:3000/user_data",
} as const;
