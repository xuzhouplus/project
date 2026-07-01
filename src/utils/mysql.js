function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\\''`)}'`;
}

/**
 * @param {{
 *   db_host?: string | null,
 *   db_port?: number | null,
 *   db_name?: string | null,
 *   db_username?: string | null,
 *   db_password?: string | null,
 * }} environment
 * @returns {string}
 */
export function buildMysqlConnectionCommand(environment) {
  const username = environment.db_username?.trim();

  if (!username) {
    throw new Error("缺少用户名，无法生成连接命令");
  }

  const host = environment.db_host?.trim();

  const port = environment.db_port;

  const parts = ["mysql"];
  if (host) {
    parts.push(`-h ${shellQuote(host)}`);
    if (port) {
      parts.push(`-P ${port}`);
    }
  }
  if (username) {
    parts.push(`-u ${shellQuote(username)}`);
  }
  parts.push("-p");

  return parts.join(" ");
}

/**
 * @param {string} text
 */
export async function copyToClipboard(text) {
  await navigator.clipboard.writeText(text);
}
