// TODO - control the API domain/port from a .env variable

export const API = {
  CREATE_USER: "http://localhost:3000/create_user",
  CONFIRM_SUBSCRIPTION: (userEmail: string, userId: string) => `http://localhost:3000/confirm/${userEmail}/${userId}`,
  GET_USER: (userId: string) => `http://localhost:3000/user/${userId}`,
  GET_USER_PLAN: (planId: string) => `http://localhost:3000/plan/${planId}`,
  GET_USER_LINKS: (userId: string) =>
    `http://localhost:3000/user/${userId}/links`,
  CREATE_LINK: "http://localhost:3000/link",
} as const;
