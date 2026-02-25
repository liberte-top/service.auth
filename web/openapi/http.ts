import axios, { type AxiosRequestConfig } from "axios";

const baseURL = import.meta.env.VITE_AUTH_API_BASE_URL ?? "http://localhost:3333";

const AXIOS = axios.create({ baseURL });

export async function customInstance<T>(config: AxiosRequestConfig): Promise<T> {
  const response = await AXIOS.request<T>(config);
  return response.data;
}
