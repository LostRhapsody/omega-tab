// TODO - control the API domain/port from a .env variable

export const API = {
  CREATE_USER: "http://localhost:3000/create_user",
  CONFIRM_SUBSCRIPTION: (userEmail: string, userId: string) =>
    `http://localhost:3000/confirm/${userEmail}/${userId}`,
  CANCEL_SUBSCRIPTION: (userEmail: string, userId: string) =>
    `http://localhost:3000/cancel/${userEmail}/${userId}`,
  GET_USER: (userId: string) => `http://localhost:3000/user/${userId}`,
  GET_USER_PLAN: (planId: string) => `http://localhost:3000/plan/${planId}`,
  GET_USER_LINKS: (userId: string) =>
    `http://localhost:3000/user/${userId}/links`,
  CREATE_LINK: "http://localhost:3000/link",
  UPDATE_LINK: "http://localhost:3000/link",
  DELETE_LINK: (linkId: string) => `http://localhost:3000/link/${linkId}`,
  SUGGEST: (query: string) => `http://localhost:3000/suggest/${query}`,
  FEEDBACK: (userId: string, userEmail: string) =>
    `http://localhost:3000/feedback/${userId}/${userEmail}`,
  CREATE_SETTINGS: (userId: string) =>
    `http://localhost:3000/settings/${userId}`,
  UPDATE_SETTINGS: (userId: string) =>
    `http://localhost:3000/settings/${userId}`,
  GET_SETTINGS: (userId: string) => `http://localhost:3000/settings/${userId}`,
  GET_USER_DATA: "http://localhost:3000/user_data",
} as const;
