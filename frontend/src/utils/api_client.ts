import { CSRF_HEADER_KEY, IDEMPOTENCY_HEADER_KEY } from '../constants';

const API_URL = import.meta.env.PUBLIC_API_URL;

export class ApiError extends Error {
  constructor(
    public status: number,
    public statusText: string,
    public body: unknown,
  ) {
    super(`Api Error ${status}: ${statusText}`);
    this.name = 'ApiError';
  }
}

let csrfToken: string | undefined = undefined;

type CustomHeadersInit = HeadersInit & {
  [IDEMPOTENCY_HEADER_KEY]?: string;
  [CSRF_HEADER_KEY]: string;
};

type CustomReqInit = RequestInit & {
  headers?: CustomHeadersInit;
};

export async function apiFetch<T>(
  path: string,
  options: CustomReqInit = {},
  idemKey: string | undefined = undefined,
): Promise<T> {
  if (!csrfToken) {
    csrfToken =
      document.cookie
        .split('; ')
        .find((item) => item.startsWith('csrf_token='))
        ?.split('=')[1] || '';
  }

  const reqOpts: CustomReqInit = {
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
      [CSRF_HEADER_KEY]: csrfToken,
      ...options.headers,
    },
    ...options,
  };

  if (idemKey && reqOpts.headers) {
    reqOpts.headers[IDEMPOTENCY_HEADER_KEY] = idemKey;
  }

  const res = await fetch(`${API_URL}/${path}`, reqOpts);

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.message || 'A technical error occured.');
  }

  // 1. Explicit no-content statuses (204, 205) — the spec says no body.
  if (res.status === 204 || res.status === 205) {
    return undefined as T;
  }

  // 2. Upstream explicitly reported zero length.
  const contentLength = res.headers.get('content-length');
  if (contentLength === '0') {
    return undefined as T;
  }

  // 3. Fallback: read text, and only parse if there's actual content.
  //    This handles 200s with empty bodies, chunked responses without
  //    Content-Length, and whitespace-only payloads.
  const text = await res.text();
  if (text.length === 0 || text.trim().length === 0) {
    return undefined as T;
  }

  // 4. Parse the body. If the server promised JSON but sent garbage,
  //    surface that as a real error instead of silently returning null.
  try {
    return JSON.parse(text) as T;
  } catch {
    throw new ApiError(res.status, res.statusText, { parseError: text });
  }
}
