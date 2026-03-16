export class AppError extends Error {
  code: number;

  constructor(message: string, code = 503) {
    super(message);
    this.code = code;
  }

  static missingApiOrigin() {
    return new AppError("AUTH_API_INTERNAL_URL is required", 503);
  }

  static authenticatedEmailMissing() {
    return new AppError("authenticated context missing email", 503);
  }

  static logoutFailed(status: number) {
    return new AppError(`logout failed with status ${status}`, 503);
  }

  static logoutCookieMissing() {
    return new AppError("logout response missing set-cookie", 503);
  }

  static logoutCookieNameMissing() {
    return new AppError("logout response missing cookie name", 503);
  }
}
