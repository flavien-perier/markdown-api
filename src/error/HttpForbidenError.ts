import HttpError from "./HttpError";

export default class HttpForbidenError extends HttpError {
    protected get statusCode() {
        return 403;
    }
}
