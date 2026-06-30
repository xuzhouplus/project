import { invoke } from "@tauri-apps/api/core";

export const API_CODE = {
  OK: 0,
  NOT_FOUND: 404,
  INTERNAL: 500,
};

export class ApiError extends Error {
  constructor(code, message) {
    super(message);
    this.name = "ApiError";
    this.code = code;
  }
}

/**
 * @template T
 * @param {string} cmd
 * @param {Record<string, unknown>} [args]
 * @returns {Promise<T>}
 */
export async function invokeApi(cmd, args) {
  /** @type {{ code: number; message: string; data?: T }} */
  const response = await invoke(cmd, args ?? {});

  if (response.code !== API_CODE.OK) {
    throw new ApiError(response.code, response.message);
  }

  if (response.data === undefined || response.data === null) {
    throw new ApiError(response.code, "响应缺少 data 字段");
  }

  return response.data;
}
