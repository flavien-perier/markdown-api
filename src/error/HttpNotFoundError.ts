import HttpError from "./HttpError";

export default class HttpNotFoundError extends HttpError {
    protected get statusCode() {
        return 404;
    }
}
