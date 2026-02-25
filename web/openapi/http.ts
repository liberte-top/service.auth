import axios, { type AxiosError, type AxiosRequestConfig } from "axios";

function resolveBaseUrl(): string | undefined {
  if (typeof window === "undefined") {
    return process.env.AUTH_API_BASE_URL;
  }
  return process.env.NEXT_PUBLIC_AUTH_API_BASE_URL;
}

export const AXIOS_INSTANCE = axios.create({
  baseURL: resolveBaseUrl(),
});

export const customInstance = <T>(
  config: AxiosRequestConfig,
  options?: AxiosRequestConfig,
): Promise<T> => {
  const mergedConfig = { ...config, ...options };
  return AXIOS_INSTANCE(mergedConfig).then(({ data }) => data as T);
};

export type ErrorType<Error> = AxiosError<Error>;
export type BodyType<BodyData> = BodyData;
