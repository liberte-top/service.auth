import axios, { type AxiosRequestConfig } from "axios";

const AXIOS = axios.create();

export async function customInstance<T>(config: AxiosRequestConfig): Promise<T> {
  const response = await AXIOS.request<T>(config);
  return response.data;
}
