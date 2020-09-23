import HttpError from "./HttpError";

export default class HttpUnauthorizedError extends HttpError {
    protected get statusCode() {
        return 401;
    }
}
