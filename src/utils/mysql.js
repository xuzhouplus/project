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
  const host = environment.db_host?.trim();
  if (!host) {
    throw new Error("缺少数据库主机，无法生成连接命令");
  }

  const port = environment.db_port;
  const username = environment.db_username?.trim();

  const parts = ["mysql", `-h ${shellQuote(host)}`];
  if (port) {
    parts.push(`-P ${port}`);
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
