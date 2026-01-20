const apiDomain = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";
const apiBase = `${apiDomain}/api`;

export const API = {
  LOGIN: `${apiBase}/login`,
  REGISTER: `${apiBase}/register`,
  CREATE_USER: `${apiBase}/create_user`,
  GET_USER: `${apiBase}/user`,
  GET_USER_LINKS: `${apiBase}/user/links`,
  CREATE_LINK: `${apiBase}/link`,
  UPDATE_LINK: `${apiBase}/link`,
  DELETE_LINK: (linkId: string) => `${apiBase}/link/${linkId}`,
  SUGGEST: (query: string) => `${apiBase}/suggest/${query}`,
  SEARCH_ICONS: (query: string) => `${apiBase}/icons/search/${encodeURIComponent(query)}`,
  FEEDBACK: `${apiBase}/feedback`,
  CREATE_SETTINGS: `${apiBase}/settings`,
  UPDATE_SETTINGS: `${apiBase}/settings`,
  GET_SETTINGS: `${apiBase}/settings`,
  GET_USER_DATA: `${apiBase}/user_data`,
  STAGING_LOGIN: `${apiBase}/staging_login`,
} as const;
