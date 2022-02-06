import HttpError from "./http.error";

export default class HttpUnauthorizedError extends HttpError {
    protected get statusCode() {
        return 401;
    }
}
